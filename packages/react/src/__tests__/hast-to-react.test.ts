import type { HastRoot } from "@unifast/core";
import { createElement, Fragment } from "react";
import { describe, it, expect } from "vitest";

import { hastToReact } from "../hast-to-react.js";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const opts = { createElement, Fragment } as any;

describe("hastToReact", () => {
  it("converts a root with text", () => {
    const hast: HastRoot = {
      type: "root",
      children: [{ type: "text", value: "hello" }],
    };
    const el = hastToReact(hast, opts) as any;
    expect(el.type).toBe(Fragment);
    expect(el.props.children).toBe("hello");
  });

  it("converts a simple element", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "p",
          properties: {},
          children: [{ type: "text", value: "paragraph" }],
        },
      ],
    };
    const el = hastToReact(hast, opts) as any;
    const p = el.props.children;
    expect(p.type).toBe("p");
    expect(p.props.children).toBe("paragraph");
  });

  it("maps components", () => {
    function MyHeading(props: { children?: React.ReactNode }) {
      return createElement("h1", { className: "custom" }, props.children);
    }
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "h1",
          properties: {},
          children: [{ type: "text", value: "title" }],
        },
      ],
    };
    const el = hastToReact(hast, {
      ...opts,
      components: { h1: MyHeading },
    }) as any;
    const h1 = el.props.children;
    expect(h1.type).toBe(MyHeading);
  });

  it("converts className array to string", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "div",
          properties: { className: ["foo", "bar"] },
          children: [],
        },
      ],
    };
    const el = hastToReact(hast, opts) as any;
    const div = el.props.children;
    expect(div.props.className).toBe("foo bar");
  });

  it("converts style string to object", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "div",
          properties: { style: "color: red; font-size: 14px" },
          children: [],
        },
      ],
    };
    const el = hastToReact(hast, opts) as any;
    const div = el.props.children;
    expect(div.props.style).toEqual({ color: "red", fontSize: "14px" });
  });

  it("renames HTML attributes to React props", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "label",
          properties: { for: "input-id", tabindex: "0" },
          children: [],
        },
      ],
    };
    const el = hastToReact(hast, opts) as any;
    const label = el.props.children;
    expect(label.props.htmlFor).toBe("input-id");
    expect(label.props.tabIndex).toBe("0");
  });

  it("skips comment and doctype nodes", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        { type: "comment", value: "ignored" },
        { type: "doctype" },
        { type: "text", value: "kept" },
      ],
    };
    const el = hastToReact(hast, opts) as any;
    expect(el.props.children).toBe("kept");
  });

  it("handles nested elements", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "ul",
          properties: {},
          children: [
            {
              type: "element",
              tagName: "li",
              properties: {},
              children: [{ type: "text", value: "item 1" }],
            },
            {
              type: "element",
              tagName: "li",
              properties: {},
              children: [{ type: "text", value: "item 2" }],
            },
          ],
        },
      ],
    };
    const el = hastToReact(hast, opts) as any;
    const ul = el.props.children;
    expect(ul.type).toBe("ul");
    expect(ul.props.children).toHaveLength(2);
    expect(ul.props.children[0].type).toBe("li");
    expect(ul.props.children[1].props.children).toBe("item 2");
  });

  it("handles void elements", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "img",
          properties: { src: "test.png", alt: "test" },
          children: [],
        },
      ],
    };
    const el = hastToReact(hast, opts) as any;
    const img = el.props.children;
    expect(img.type).toBe("img");
    expect(img.props.src).toBe("test.png");
    expect(img.props.alt).toBe("test");
  });

  it("handles raw nodes", () => {
    const hast: HastRoot = {
      type: "root",
      children: [{ type: "raw", value: "<em>raw</em>" }],
    };
    const el = hastToReact(hast, opts) as any;
    const div = el.props.children;
    expect(div.type).toBe("div");
    expect(div.props).toHaveProperty("dangerouslySetInnerHTML");
  });

  it("drops null/false/undefined properties", () => {
    const hast: HastRoot = {
      type: "root",
      children: [
        {
          type: "element",
          tagName: "input",
          properties: { disabled: false, value: null, type: "text" },
          children: [],
        },
      ],
    };
    const el = hastToReact(hast, opts) as any;
    const input = el.props.children;
    expect(input.props.disabled).toBeUndefined();
    expect(input.props.value).toBeUndefined();
    expect(input.props.type).toBe("text");
  });
});
