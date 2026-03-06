import { describe, expect, it } from "vitest";

import { createShikiTransformer } from "../transformer.js";
import type { HastRoot } from "../types.js";

describe(createShikiTransformer, () => {
  it("transforms a code block with known language", async () => {
    const transformer = await createShikiTransformer({
      themes: ["github-dark"],
      langs: ["rust"],
    });

    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "pre",
          properties: {},
          children: [
            {
              type: "element",
              tagName: "code",
              properties: { className: ["language-rust"] },
              children: [{ type: "text", value: "fn main() {}" }],
            },
          ],
        },
      ],
    };

    const result = transformer.transform(hast);
    const pre = result.children[0];
    expect(pre.type).toBe("element");
    if (pre.type === "element") {
      expect(pre.tagName).toBe("pre");
      const cls = pre.properties.class ?? pre.properties.className;
      const clsStr = Array.isArray(cls) ? cls.join(" ") : String(cls);
      expect(clsStr).toContain("shiki");
    }
  });

  it("leaves unknown languages unchanged", async () => {
    const transformer = await createShikiTransformer({
      themes: ["github-dark"],
      langs: ["rust"],
    });

    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "pre",
          properties: {},
          children: [
            {
              type: "element",
              tagName: "code",
              properties: { className: ["language-unknownlang123"] },
              children: [{ type: "text", value: "some code" }],
            },
          ],
        },
      ],
    };

    const result = transformer.transform(hast);
    const pre = result.children[0];
    if (pre.type === "element") {
      const code = pre.children[0];
      if (code.type === "element") {
        expect(code.properties.className).toEqual(["language-unknownlang123"]);
      }
    }
  });

  it("preserves non-code elements", async () => {
    const transformer = await createShikiTransformer({
      themes: ["github-dark"],
      langs: ["rust"],
    });

    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "p",
          properties: {},
          children: [{ type: "text", value: "hello" }],
        },
      ],
    };

    const result = transformer.transform(hast);
    expect(result).toEqual(hast);
  });
});
