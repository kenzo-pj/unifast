import type { HastRoot } from "@unifast/core";
import { describe, expect, it } from "vitest";

import { hastToHtml } from "../hast-utils.js";

describe(hastToHtml, () => {
  it("serializes a text node", () => {
    const hast: HastRoot = {
      type: "root",
      children: [{ type: "text", value: "hello" }],
    };
    expect(hastToHtml(hast)).toBe("hello");
  });

  it("escapes HTML in text nodes", () => {
    const hast: HastRoot = {
      type: "root",
      children: [{ type: "text", value: "a < b & c > d" }],
    };
    expect(hastToHtml(hast)).toBe("a &lt; b &amp; c &gt; d");
  });

  it("serializes an element with attributes", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "a",
          properties: { href: "http://example.com" },
          children: [{ type: "text", value: "click" }],
        },
      ],
    };
    expect(hastToHtml(hast)).toBe('<a href="http://example.com">click</a>');
  });

  it("converts className array to class attribute", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "code",
          properties: { className: ["language-rust", "highlighted"] },
          children: [{ type: "text", value: "code" }],
        },
      ],
    };
    expect(hastToHtml(hast)).toBe('<code class="language-rust highlighted">code</code>');
  });

  it("serializes void elements", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "br",
          properties: {},
          children: [],
        },
      ],
    };
    expect(hastToHtml(hast)).toBe("<br />");
  });

  it("serializes raw nodes as-is", () => {
    const hast: HastRoot = {
      type: "root",
      children: [{ type: "raw", value: "<b>bold</b>" }],
    };
    expect(hastToHtml(hast)).toBe("<b>bold</b>");
  });

  it("serializes comment nodes", () => {
    const hast: HastRoot = {
      type: "root",
      children: [{ type: "comment", value: " todo " }],
    };
    expect(hastToHtml(hast)).toBe("<!-- todo -->");
  });

  it("serializes boolean attributes", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "input",
          properties: { type: "checkbox", checked: true, disabled: true },
          children: [],
        },
      ],
    };
    const html = hastToHtml(hast);
    expect(html).toContain("checked");
    expect(html).toContain("disabled");
    expect(html).toContain('type="checkbox"');
  });

  it("serializes a full code block", () => {
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
    expect(hastToHtml(hast)).toBe('<pre><code class="language-rust">fn main() {}</code></pre>');
  });
});
