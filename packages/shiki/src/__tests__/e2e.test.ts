import type { HastRoot } from "@unifast/core";
import { describe, expect, it } from "vitest";

import { hastToHtml } from "../hast-utils.js";
import { createShikiTransformer } from "../transformer.js";

describe("e2e: HAST JSON -> shiki -> HTML", () => {
  it("full pipeline with manually constructed HAST", async () => {
    const hastJson: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "h1",
          properties: { id: "hello" },
          children: [{ type: "text", value: "Hello" }],
        },
        {
          type: "element",
          tagName: "pre",
          properties: {},
          children: [
            {
              type: "element",
              tagName: "code",
              properties: { className: ["language-typescript"] },
              children: [{ type: "text", value: "const x: number = 42;" }],
            },
          ],
        },
        {
          type: "element",
          tagName: "p",
          properties: {},
          children: [{ type: "text", value: "End." }],
        },
      ],
    };

    const transformer = await createShikiTransformer({
      themes: ["github-dark"],
      langs: ["typescript"],
    });

    const highlighted = transformer.transform(hastJson);
    const html = hastToHtml(highlighted);

    expect(html).toContain('<h1 id="hello">Hello</h1>');
    expect(html).toContain("<p>End.</p>");
    expect(html).toContain("shiki");
    expect(html).toContain("<span");
    expect(html).not.toContain('class="language-typescript"');
  });
});
