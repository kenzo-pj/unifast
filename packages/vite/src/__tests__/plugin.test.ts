import fs from "node:fs";

import { describe, it, expect, expectTypeOf, vi, beforeEach } from "vitest";

vi.mock(import("node:module"), () => ({
  createRequire: () => (id: string) => {
    throw new Error(`Cannot find module '${id}'`);
  },
}));

describe("unifastPlugin (fallback – no compiler)", () => {
  beforeEach(() => {
    vi.resetModules();
    vi.restoreAllMocks();
  });

  async function getPlugin(options = {}) {
    const mod = await import("../../src/index.js");
    return mod.default(options);
  }

  it("has name 'vite-plugin-unifast'", async () => {
    const plugin = await getPlugin();
    expect(plugin.name).toBe("vite-plugin-unifast");
  });

  it("has enforce 'pre'", async () => {
    const plugin = await getPlugin();
    expect(plugin.enforce).toBe("pre");
  });

  it("has a transform hook", async () => {
    const plugin = await getPlugin();
    expectTypeOf(plugin.transform).toBeFunction();
  });

  it("has a handleHotUpdate hook", async () => {
    const plugin = await getPlugin();
    expectTypeOf(plugin.handleHotUpdate).toBeFunction();
  });

  it.each([".ts", ".js", ".css", ".html", ".json"])(
    "transform returns null for %s files",
    async (ext) => {
      const plugin = await getPlugin();
      const transform = plugin.transform as Function;
      const result = transform.call({}, "", `file${ext}`);
      expect(result).toBeNull();
    },
  );

  it("transform returns null for .MD (case-sensitive regex)", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "FILE.MD");
    expect(result).toBeNull();
  });

  it(".md transform returns code with all expected exports", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("Hello world");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    expect(result).not.toBeNull();
    expect(result.code).toContain("export const html =");
    expect(result.code).toContain("export const frontmatter =");
    expect(result.code).toContain("export const toc =");
    expect(result.code).toContain("export default ");
  });

  it(".md transform returns map: null", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("Hello");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    expect(result.map).toBeNull();
  });

  it("parses simple frontmatter key-value pairs", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\ntitle: Hello\n---\nBody text");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain('"title":"Hello"');
  });

  it("returns empty frontmatter when none present", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("Just a body");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain("export const frontmatter = {};");
  });

  it("handles Windows line endings in frontmatter", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\r\ntitle: Win\r\n---\r\nBody");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain('"title":"Win"');
  });

  it("splits frontmatter only on the first colon", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\ntime: 12:30:45\n---\nBody");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain('"time":45045');
  });

  it("handles frontmatter key with empty value", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\ndraft:\n---\nBody");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain('"draft":null');
  });

  it("escapes < to &lt; in fallback HTML", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("<script>alert(1)</script>");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain("&lt;");
    expect(code).not.toContain("<script>");
  });

  it("converts newlines to <br> in fallback HTML", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("line1\nline2\nline3");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain("<br>");
  });

  it("handles empty body (frontmatter only)", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\ntitle: OnlyMeta\n---\n");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    expect(result).not.toBeNull();
    expect(result.code).toContain("export const html =");
  });

  it(".mdx transform returns null when compiler is not available", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("# Hello MDX");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "component.mdx");

    expect(result).toBeNull();
  });

  it("handleHotUpdate returns module array for .md file", async () => {
    const plugin = await getPlugin();
    const handleHotUpdate = plugin.handleHotUpdate as Function;
    const mockMod = { id: "test.md" };
    const ctx = {
      file: "docs/readme.md",
      server: {
        moduleGraph: {
          getModuleById: vi.fn().mockReturnValue(mockMod),
        },
      },
    };
    const result = handleHotUpdate.call({}, ctx);
    expect(result).toEqual([mockMod]);
  });

  it("handleHotUpdate returns module array for .mdx file", async () => {
    const plugin = await getPlugin();
    const handleHotUpdate = plugin.handleHotUpdate as Function;
    const mockMod = { id: "component.mdx" };
    const ctx = {
      file: "src/component.mdx",
      server: {
        moduleGraph: {
          getModuleById: vi.fn().mockReturnValue(mockMod),
        },
      },
    };
    const result = handleHotUpdate.call({}, ctx);
    expect(result).toEqual([mockMod]);
  });

  it("handleHotUpdate returns undefined for non-md file", async () => {
    const plugin = await getPlugin();
    const handleHotUpdate = plugin.handleHotUpdate as Function;
    const ctx = {
      file: "src/index.ts",
      server: {
        moduleGraph: {
          getModuleById: vi.fn(),
        },
      },
    };
    const result = handleHotUpdate.call({}, ctx);
    expect(result).toBeUndefined();
  });

  it("handleHotUpdate returns undefined when module not in graph", async () => {
    const plugin = await getPlugin();
    const handleHotUpdate = plugin.handleHotUpdate as Function;
    const ctx = {
      file: "docs/missing.md",
      server: {
        moduleGraph: {
          getModuleById: vi.fn().mockReturnValue(),
        },
      },
    };
    const result = handleHotUpdate.call({}, ctx);
    expect(result).toBeUndefined();
  });
});

describe("unifastPlugin (with compiler)", () => {
  const mockCompile = vi.fn();

  beforeEach(() => {
    vi.resetModules();
    vi.restoreAllMocks();
    mockCompile.mockReset();

    vi.doMock("node:module", () => ({
      createRequire: () => (id: string) => {
        if (id === "@unifast/node") {
          return { compile: mockCompile };
        }
        throw new Error(`Cannot find module '${id}'`);
      },
    }));
  });

  async function getPlugin(options = {}) {
    const mod = await import("../../src/index.js");
    return mod.default(options);
  }

  it(".md transform uses compiler output when available", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("# Hello");
    mockCompile.mockReturnValue({
      output: "<h1>Hello</h1>",
      frontmatter: { title: "Hello" },
      toc: [{ depth: 1, text: "Hello", slug: "hello" }],
    });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "page.md");

    expect(result).not.toBeNull();
    expect(result.code).toContain("<h1>Hello</h1>");
    expect(result.code).toContain('"title":"Hello"');
    expect(result.code).toContain('"slug":"hello"');
    expect(mockCompile).toHaveBeenCalledWith(
      "# Hello",
      expect.objectContaining({
        inputKind: "md",
        outputKind: "html",
      }),
    );
  });

  it(".md transform wraps in <pre> when compiler throws", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("Bad <content>");
    mockCompile.mockImplementation(() => {
      throw new Error("compile error");
    });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "broken.md");

    expect(result).not.toBeNull();
    expect(result.code).toContain("<pre>");
    expect(result.code).toContain("&lt;");
  });

  it(".mdx transform produces JSX imports when compiler is available", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("# MDX Content");
    mockCompile.mockReturnValue({
      output: "function MDXContent() { return _jsx('h1', { children: 'MDX Content' }); }",
      frontmatter: { layout: "post" },
      toc: [{ depth: 1, text: "MDX Content", slug: "mdx-content" }],
    });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "doc.mdx");

    expect(result).not.toBeNull();
    expect(result.code).toContain(
      'import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime"',
    );
    expect(result.code).toContain("export const frontmatter =");
    expect(result.code).toContain("export const toc =");
    expect(mockCompile).toHaveBeenCalledWith(
      "# MDX Content",
      expect.objectContaining({
        inputKind: "mdx",
        outputKind: "mdxJs",
      }),
    );
  });

  it(".mdx transform injects components prop support", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("# Hello");
    mockCompile.mockReturnValue({
      output: [
        "function MDXContent(props) {",
        '  return _jsx("h1", { children: "Hello" });',
        "}",
        "export default MDXContent;",
      ].join("\n"),
      frontmatter: {},
      toc: [],
    });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.mdx");

    expect(result.code).toContain("{ components: _components = {}");
    expect(result.code).toContain("const _c = (t) => _components[t] || t;");
    expect(result.code).toContain('_jsx(_c("h1")');
    expect(result.code).not.toContain('_jsx("h1"');
  });

  it(".mdx transform does not replace capitalized component references", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("# Hi");
    mockCompile.mockReturnValue({
      output: [
        'import { Alert } from "~/components/Alert";',
        "function MDXContent(props) {",
        "  return _jsxs(_Fragment, { children: [",
        '    _jsx(Alert, { children: "warn" }),',
        '    _jsx("p", { children: "text" })',
        "  ] });",
        "}",
        "export default MDXContent;",
      ].join("\n"),
      frontmatter: {},
      toc: [],
    });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.mdx");

    expect(result.code).toContain("_jsx(Alert,");
    expect(result.code).toContain('_jsx(_c("p")');
  });

  it(".mdx transform preserves pre tag for highlighted code blocks with __rawCode", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("```js\nconst x = 1;\n```");
    const highlightedHtml = '<pre><code class="language-js"><span>const</span> x = 1;</code></pre>';
    mockCompile
      .mockReturnValueOnce({
        output: [
          "function MDXContent(props) {",
          '  return _jsx("pre", { children: _jsx("code", { children: "const x = 1;", className: "language-js" }) });',
          "}",
          "export default MDXContent;",
        ].join("\n"),
        frontmatter: {},
        toc: [],
      })
      .mockReturnValueOnce({
        output: highlightedHtml,
        frontmatter: {},
        toc: [],
      });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "code.mdx");

    // Should use "pre" tag (resolved via _c), not "div"
    expect(result.code).toContain('_jsx(_c("pre")');
    expect(result.code).not.toContain('_jsx(_c("div")');
    // Should include __rawCode prop
    expect(result.code).toContain("__rawCode:");
    expect(result.code).toContain("const x = 1;");
    // Should have dangerouslySetInnerHTML with outer <pre> stripped
    expect(result.code).toContain("dangerouslySetInnerHTML");
    // Inner HTML should NOT contain the full <pre>-wrapped output
    expect(result.code).not.toContain(JSON.stringify(highlightedHtml));
  });
});
