import type { HastRoot, HastElement, HastText, HastNode } from "@unifast/core";
import { describe, it, expect } from "vitest";

import { highlight } from "../index";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function makeText(value: string): HastText {
  return { type: "text", value };
}

function makeCode(lang: string, text: string): HastElement {
  return {
    type: "element",
    tagName: "code",
    properties: { className: [`language-${lang}`] },
    children: [makeText(text)],
  };
}

function makePre(code: HastElement): HastElement {
  return {
    type: "element",
    tagName: "pre",
    properties: {},
    children: [code],
  };
}

function makeCodeBlock(lang: string, text: string): HastElement {
  return makePre(makeCode(lang, text));
}

function makeRoot(...children: HastNode[]): HastRoot {
  return { type: "root", children };
}

function getTransform() {
  const plugin = highlight();
  return plugin.hastTransform!;
}

// ---------------------------------------------------------------------------
// 1. Plugin metadata
// ---------------------------------------------------------------------------

describe("plugin metadata", () => {
  it("has name 'highlight'", () => {
    const plugin = highlight();
    expect(plugin.name).toBe("highlight");
  });

  it("has options with highlight.enabled = false", () => {
    const plugin = highlight();
    expect(plugin.options).toStrictEqual({
      highlight: { enabled: false },
    });
  });

  it("exposes hastTransform as a function", () => {
    const plugin = highlight();
    expect(typeof plugin.hastTransform).toBe("function");
  });
});

// ---------------------------------------------------------------------------
// 2. JavaScript code block highlighting
// ---------------------------------------------------------------------------

describe("JavaScript highlighting", () => {
  it("adds hljs class and produces highlighted spans", () => {
    const transform = getTransform();
    const root = makeRoot(makeCodeBlock("javascript", "const x = 1;"));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    expect(pre.tagName).toBe("pre");
    const code = pre.children[0] as HastElement;
    expect(code.tagName).toBe("code");
    expect(code.properties.className).toStrictEqual(["language-javascript", "hljs"]);
    // Lowlight should produce at least one span element for syntax highlighting
    const hasSpan = code.children.some((c) => c.type === "element" && c.tagName === "span");
    expect(hasSpan).toBeTruthy();
  });
});

// ---------------------------------------------------------------------------
// 3. Python code block highlighting
// ---------------------------------------------------------------------------

describe("Python highlighting", () => {
  it("highlights Python code and adds hljs class", () => {
    const transform = getTransform();
    const root = makeRoot(makeCodeBlock("python", 'def hello():\n    print("hi")'));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const code = pre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-python", "hljs"]);
    const hasSpan = code.children.some((c) => c.type === "element" && c.tagName === "span");
    expect(hasSpan).toBeTruthy();
  });
});

// ---------------------------------------------------------------------------
// 4. Multiple code blocks in same document
// ---------------------------------------------------------------------------

describe("multiple code blocks", () => {
  it("highlights all code blocks in the same document", () => {
    const transform = getTransform();
    const root = makeRoot(
      makeCodeBlock("javascript", "const a = 1;"),
      makeCodeBlock("python", "x = 2"),
    );
    const result = transform(root);

    const pre1 = result.children[0] as HastElement;
    const code1 = pre1.children[0] as HastElement;
    expect(code1.properties.className).toStrictEqual(["language-javascript", "hljs"]);

    const pre2 = result.children[1] as HastElement;
    const code2 = pre2.children[0] as HastElement;
    expect(code2.properties.className).toStrictEqual(["language-python", "hljs"]);
  });
});

// ---------------------------------------------------------------------------
// 5. Pre without code child -> unchanged
// ---------------------------------------------------------------------------

describe("pre without code child", () => {
  it("leaves pre unchanged when it has no code child", () => {
    const transform = getTransform();
    const pre: HastElement = {
      type: "element",
      tagName: "pre",
      properties: {},
      children: [makeText("plain text")],
    };
    const root = makeRoot(pre);
    const result = transform(root);

    const resultPre = result.children[0] as HastElement;
    expect(resultPre.tagName).toBe("pre");
    expect(resultPre.children).toHaveLength(1);
    expect(resultPre.children[0]).toStrictEqual(makeText("plain text"));
  });
});

// ---------------------------------------------------------------------------
// 6. Code without language className -> unchanged
// ---------------------------------------------------------------------------

describe("code without language className", () => {
  it("leaves code block unchanged when no language- class is present", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: ["some-other-class"] },
      children: [makeText("no lang")],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    expect(resultCode.properties.className).toStrictEqual(["some-other-class"]);
    expect(resultCode.children).toStrictEqual([makeText("no lang")]);
  });
});

// ---------------------------------------------------------------------------
// 7. Code with unregistered language -> unchanged
// ---------------------------------------------------------------------------

describe("unregistered language", () => {
  it("leaves code block unchanged for an unregistered language", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: ["language-nonexistent-xyzzy"] },
      children: [makeText("hello")],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    expect(resultCode.properties.className).toStrictEqual(["language-nonexistent-xyzzy"]);
    expect(resultCode.children).toStrictEqual([makeText("hello")]);
  });
});

// ---------------------------------------------------------------------------
// 8. Empty code block
// ---------------------------------------------------------------------------

describe("empty code block", () => {
  it("handles an empty code block with a known language", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: ["language-javascript"] },
      children: [],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    expect(resultCode.properties.className).toStrictEqual(["language-javascript", "hljs"]);
    // Empty input should produce no children or empty children
    expect(resultCode.children).toHaveLength(0);
  });
});

// ---------------------------------------------------------------------------
// 9. Code with nested elements (text extraction works)
// ---------------------------------------------------------------------------

describe("nested elements for text extraction", () => {
  it("extracts text from nested elements within code", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: ["language-javascript"] },
      children: [
        {
          type: "element",
          tagName: "span",
          properties: {},
          children: [makeText("const ")],
        },
        {
          type: "element",
          tagName: "span",
          properties: {},
          children: [makeText("x = 1;")],
        },
      ],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    expect(resultCode.properties.className).toStrictEqual(["language-javascript", "hljs"]);
    // The highlighted output should have spans from lowlight
    const hasSpan = resultCode.children.some((c) => c.type === "element" && c.tagName === "span");
    expect(hasSpan).toBeTruthy();
  });
});

// ---------------------------------------------------------------------------
// 10. className as non-array (string) -> ignored
// ---------------------------------------------------------------------------

describe("className as non-array", () => {
  it("returns null language when className is a string instead of array", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: "language-javascript" },
      children: [makeText("const x = 1;")],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    // ClassName is not an array so extractLang returns null -> no transformation
    expect(resultCode.properties.className).toBe("language-javascript");
    expect(resultCode.children).toStrictEqual([makeText("const x = 1;")]);
  });
});

// ---------------------------------------------------------------------------
// 11. className with non-language- classes -> ignored
// ---------------------------------------------------------------------------

describe("className with non-language- classes", () => {
  it("ignores classes that do not start with language-", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: ["highlight", "code-block", "fancy"] },
      children: [makeText("hello")],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    expect(resultCode.properties.className).toStrictEqual(["highlight", "code-block", "fancy"]);
    expect(resultCode.children).toStrictEqual([makeText("hello")]);
  });
});

// ---------------------------------------------------------------------------
// 12. Preserves non-pre elements (p, div, etc.)
// ---------------------------------------------------------------------------

describe("preserves non-pre elements", () => {
  it("passes through p elements unchanged", () => {
    const transform = getTransform();
    const p: HastElement = {
      type: "element",
      tagName: "p",
      properties: { className: ["intro"] },
      children: [makeText("Hello world")],
    };
    const root = makeRoot(p);
    const result = transform(root);

    const resultP = result.children[0] as HastElement;
    expect(resultP.tagName).toBe("p");
    expect(resultP.children).toStrictEqual([makeText("Hello world")]);
  });

  it("passes through div elements unchanged", () => {
    const transform = getTransform();
    const div: HastElement = {
      type: "element",
      tagName: "div",
      properties: {},
      children: [makeText("content")],
    };
    const root = makeRoot(div);
    const result = transform(root);

    const resultDiv = result.children[0] as HastElement;
    expect(resultDiv.tagName).toBe("div");
    expect(resultDiv.children).toStrictEqual([makeText("content")]);
  });
});

// ---------------------------------------------------------------------------
// 13. Passes through text nodes unchanged
// ---------------------------------------------------------------------------

describe("text nodes", () => {
  it("passes through text nodes unchanged", () => {
    const transform = getTransform();
    const text = makeText("just text");
    const root = makeRoot(text);
    const result = transform(root);

    expect(result.children[0]).toStrictEqual(makeText("just text"));
  });
});

// ---------------------------------------------------------------------------
// 14. Passes through comment nodes unchanged
// ---------------------------------------------------------------------------

describe("comment nodes", () => {
  it("passes through comment nodes unchanged", () => {
    const transform = getTransform();
    const comment: HastNode = { type: "comment", value: "a comment" };
    const root = makeRoot(comment);
    const result = transform(root);

    expect(result.children[0]).toStrictEqual({ type: "comment", value: "a comment" });
  });
});

// ---------------------------------------------------------------------------
// 15. Passes through raw nodes unchanged
// ---------------------------------------------------------------------------

describe("raw nodes", () => {
  it("passes through raw nodes unchanged", () => {
    const transform = getTransform();
    const raw: HastNode = { type: "raw", value: "<div>raw html</div>" };
    const root = makeRoot(raw);
    const result = transform(root);

    expect(result.children[0]).toStrictEqual({ type: "raw", value: "<div>raw html</div>" });
  });
});

// ---------------------------------------------------------------------------
// 16. Passes through doctype nodes unchanged
// ---------------------------------------------------------------------------

describe("doctype nodes", () => {
  it("passes through doctype nodes unchanged", () => {
    const transform = getTransform();
    const doctype: HastNode = { type: "doctype" };
    const root = makeRoot(doctype);
    const result = transform(root);

    expect(result.children[0]).toStrictEqual({ type: "doctype" });
  });
});

// ---------------------------------------------------------------------------
// 17. Recursively transforms nested elements (div > pre > code)
// ---------------------------------------------------------------------------

describe("recursive transformation", () => {
  it("transforms pre>code inside a div", () => {
    const transform = getTransform();
    const div: HastElement = {
      type: "element",
      tagName: "div",
      properties: {},
      children: [makeCodeBlock("javascript", "let y = 2;")],
    };
    const root = makeRoot(div);
    const result = transform(root);

    const resultDiv = result.children[0] as HastElement;
    expect(resultDiv.tagName).toBe("div");
    const pre = resultDiv.children[0] as HastElement;
    expect(pre.tagName).toBe("pre");
    const code = pre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-javascript", "hljs"]);
    const hasSpan = code.children.some((c) => c.type === "element" && c.tagName === "span");
    expect(hasSpan).toBeTruthy();
  });
});

// ---------------------------------------------------------------------------
// 18. Deeply nested structure (section > article > pre > code)
// ---------------------------------------------------------------------------

describe("deeply nested structure", () => {
  it("transforms pre>code inside section>article", () => {
    const transform = getTransform();
    const section: HastElement = {
      type: "element",
      tagName: "section",
      properties: {},
      children: [
        {
          type: "element",
          tagName: "article",
          properties: {},
          children: [makeCodeBlock("python", "print('deep')")],
        },
      ],
    };
    const root = makeRoot(section);
    const result = transform(root);

    const resultSection = result.children[0] as HastElement;
    const article = resultSection.children[0] as HastElement;
    const pre = article.children[0] as HastElement;
    const code = pre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-python", "hljs"]);
    const hasSpan = code.children.some((c) => c.type === "element" && c.tagName === "span");
    expect(hasSpan).toBeTruthy();
  });
});

// ---------------------------------------------------------------------------
// 19. Empty root
// ---------------------------------------------------------------------------

describe("empty root", () => {
  it("handles an empty root with no children", () => {
    const transform = getTransform();
    const root = makeRoot();
    const result = transform(root);

    expect(result.type).toBe("root");
    expect(result.children).toHaveLength(0);
  });
});

// ---------------------------------------------------------------------------
// 20. Code with special characters (HTML entities, quotes)
// ---------------------------------------------------------------------------

describe("special characters", () => {
  it("handles code containing HTML special characters", () => {
    const transform = getTransform();
    const source = String.raw`const x = "<div class=\"test\">&amp;</div>";`;
    const root = makeRoot(makeCodeBlock("javascript", source));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const code = pre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-javascript", "hljs"]);
    // The highlight should still produce output (not crash)
    expect(code.children.length).toBeGreaterThanOrEqual(0);
  });

  it("handles code with angle brackets and ampersands", () => {
    const transform = getTransform();
    const source = "if (a < b && c > d) {}";
    const root = makeRoot(makeCodeBlock("javascript", source));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const code = pre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-javascript", "hljs"]);
  });
});

// ---------------------------------------------------------------------------
// 21. Code with unicode characters
// ---------------------------------------------------------------------------

describe("unicode characters", () => {
  it("handles code with unicode identifiers and strings", () => {
    const transform = getTransform();
    const source = 'const greeting = "こんにちは世界"; // 🌍';
    const root = makeRoot(makeCodeBlock("javascript", source));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const code = pre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-javascript", "hljs"]);

    // Collect all text content from the highlighted output
    function collectText(node: HastNode): string {
      if (node.type === "text") return node.value;
      if (node.type === "element" || node.type === "root") {
        return node.children.map(collectText).join("");
      }
      return "";
    }
    const outputText = code.children.map(collectText).join("");
    expect(outputText).toContain("こんにちは世界");
  });
});

// ---------------------------------------------------------------------------
// 22. First language- class wins when multiple present
// ---------------------------------------------------------------------------

describe("first language- class wins", () => {
  it("uses the first language- class when multiple are present", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: ["language-python", "language-javascript"] },
      children: [makeText("x = 1")],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    // The first language- class is "python", so it should be highlighted as python
    expect(resultCode.properties.className).toStrictEqual(["language-python", "hljs"]);
  });
});

// ---------------------------------------------------------------------------
// 23. Code with className having empty array
// ---------------------------------------------------------------------------

describe("empty className array", () => {
  it("leaves code unchanged when className is an empty array", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: [] },
      children: [makeText("no highlight")],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    expect(resultCode.children).toStrictEqual([makeText("no highlight")]);
  });
});

// ---------------------------------------------------------------------------
// Additional edge cases
// ---------------------------------------------------------------------------

describe("preserves existing code properties", () => {
  it("keeps existing properties on the code element after highlighting", () => {
    const transform = getTransform();
    const code: HastElement = {
      type: "element",
      tagName: "code",
      properties: { className: ["language-javascript"], "data-line": "1-3", id: "my-code" },
      children: [makeText("const a = 1;")],
    };
    const root = makeRoot(makePre(code));
    const result = transform(root);

    const pre = result.children[0] as HastElement;
    const resultCode = pre.children[0] as HastElement;
    expect(resultCode.properties["data-line"]).toBe("1-3");
    expect(resultCode.properties.id).toBe("my-code");
    expect(resultCode.properties.className).toStrictEqual(["language-javascript", "hljs"]);
  });
});

describe("mixed content in root", () => {
  it("handles a document with mixed text, elements, and code blocks", () => {
    const transform = getTransform();
    const root = makeRoot(
      makeText("intro text"),
      {
        type: "element",
        tagName: "p",
        properties: {},
        children: [makeText("paragraph")],
      } as HastElement,
      makeCodeBlock("javascript", "const z = 3;"),
      { type: "comment", value: "end" } as HastNode,
    );
    const result = transform(root);

    expect(result.children).toHaveLength(4);
    expect(result.children[0]).toStrictEqual(makeText("intro text"));
    expect((result.children[1] as HastElement).tagName).toBe("p");

    const pre = result.children[2] as HastElement;
    const code = pre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-javascript", "hljs"]);

    expect(result.children[3]).toStrictEqual({ type: "comment", value: "end" });
  });
});

describe("root node inside element children", () => {
  it("transforms root nodes nested in element children", () => {
    // While unusual, the transformNode function handles root nodes recursively
    const transform = getTransform();
    // We can't directly nest a root in children via the type system easily,
    // But the code handles it. We test via the top-level root which exercises
    // The root -> children -> map path.
    const root: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "div",
          properties: {},
          children: [makeCodeBlock("css", "body { color: red; }")],
        },
      ],
    };
    const result = transform(root);

    const div = result.children[0] as HastElement;
    const pre = div.children[0] as HastElement;
    const code = pre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-css", "hljs"]);
  });
});

describe("pre with multiple children including code", () => {
  it("finds and highlights the code element among siblings", () => {
    const transform = getTransform();
    const pre: HastElement = {
      type: "element",
      tagName: "pre",
      properties: {},
      children: [makeText("prefix "), makeCode("javascript", "const a = 1;"), makeText(" suffix")],
    };
    const root = makeRoot(pre);
    const result = transform(root);

    const resultPre = result.children[0] as HastElement;
    // When code is found, the pre's children get replaced with just the highlighted code
    const code = resultPre.children[0] as HastElement;
    expect(code.properties.className).toStrictEqual(["language-javascript", "hljs"]);
  });
});
