import { describe, it, expect } from "vitest";

import type { HastRoot, HastElement, HastText, HastComment, HastDoctype, HastRaw } from "../hast";
import { hastToHtml } from "../hast";

function root(...children: HastRoot["children"]): HastRoot {
  return { type: "root", children };
}

function el(
  tagName: string,
  properties: Record<string, unknown>,
  ...children: HastElement["children"]
): HastElement {
  return { type: "element", tagName, properties, children };
}

function text(value: string): HastText {
  return { type: "text", value };
}

describe(hastToHtml, () => {
  it("renders plain text", () => {
    expect(hastToHtml(root(text("hello")))).toBe("hello");
  });

  it("escapes special HTML characters in text", () => {
    expect(hastToHtml(root(text('<script>"alert&</script>')))).toBe(
      "&lt;script&gt;&quot;alert&amp;&lt;/script&gt;",
    );
  });

  it("renders a simple element", () => {
    expect(hastToHtml(root(el("p", {}, text("hi"))))).toBe("<p>hi</p>");
  });

  it("renders nested elements", () => {
    const tree = root(el("div", {}, el("p", {}, text("nested"))));
    expect(hastToHtml(tree)).toBe("<div><p>nested</p></div>");
  });

  it("renders void elements as self-closing", () => {
    expect(hastToHtml(root(el("br", {})))).toBe("<br />");
    expect(hastToHtml(root(el("img", { src: "a.png" })))).toBe('<img src="a.png" />');
    expect(hastToHtml(root(el("hr", {})))).toBe("<hr />");
    expect(hastToHtml(root(el("input", { type: "text" })))).toBe('<input type="text" />');
  });

  it("renders string attributes", () => {
    expect(hastToHtml(root(el("a", { href: "/path" }, text("link"))))).toBe(
      '<a href="/path">link</a>',
    );
  });

  it("renders boolean true attributes as bare attribute", () => {
    expect(hastToHtml(root(el("input", { disabled: true })))).toBe("<input disabled />");
  });

  it("omits boolean false attributes", () => {
    expect(hastToHtml(root(el("input", { disabled: false })))).toBe("<input />");
  });

  it("omits null and undefined attributes", () => {
    expect(hastToHtml(root(el("div", { "data-x": null, "data-y": undefined })))).toBe(
      "<div></div>",
    );
  });

  it("renders className array as class attribute", () => {
    expect(hastToHtml(root(el("div", { className: ["foo", "bar"] })))).toBe(
      '<div class="foo bar"></div>',
    );
  });

  it("omits empty className array", () => {
    expect(hastToHtml(root(el("div", { className: [] })))).toBe("<div></div>");
  });

  it("renders class string directly", () => {
    expect(hastToHtml(root(el("div", { class: "baz" })))).toBe('<div class="baz"></div>');
  });

  it("escapes attribute values", () => {
    expect(hastToHtml(root(el("div", { title: 'a"b&c<d' })))).toBe(
      '<div title="a&quot;b&amp;c&lt;d"></div>',
    );
  });

  it("sorts attributes alphabetically", () => {
    expect(hastToHtml(root(el("div", { z: "1", a: "2" })))).toBe('<div a="2" z="1"></div>');
  });

  it("renders comment nodes", () => {
    const comment: HastComment = { type: "comment", value: " todo " };
    expect(hastToHtml(root(comment))).toBe("<!-- todo -->");
  });

  it("renders doctype nodes", () => {
    const doctype: HastDoctype = { type: "doctype" };
    expect(hastToHtml(root(doctype))).toBe("<!DOCTYPE html>");
  });

  it("renders raw nodes without escaping", () => {
    const raw: HastRaw = { type: "raw", value: "<strong>raw</strong>" };
    expect(hastToHtml(root(raw))).toBe("<strong>raw</strong>");
  });

  it("renders multiple children", () => {
    expect(hastToHtml(root(el("p", {}, text("a")), el("p", {}, text("b"))))).toBe(
      "<p>a</p><p>b</p>",
    );
  });

  it("renders empty root", () => {
    expect(hastToHtml(root())).toBe("");
  });
});
