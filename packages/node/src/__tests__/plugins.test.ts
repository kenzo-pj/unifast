import type { UnifastPlugin, HastRoot } from "@unifast/core";
import { describe, expect, it } from "vitest";

import { compile } from "../index.js";

describe("unified plugin API", () => {
  it("compiles with option-based plugin", () => {
    const plugin: UnifastPlugin = {
      name: "test-gfm",
      options: {
        gfm: {
          tables: true,
          taskList: true,
          strikethrough: true,
          footnotes: true,
          autolink: true,
        },
      },
    };

    const result = compile("| a | b |\n|---|---|\n| 1 | 2 |", {
      plugins: [plugin],
    });

    expect(result.output).toContain("<table");
    expect(result.output).toContain("<td");
  });

  it("compiles with hastTransform plugin", () => {
    const uppercasePlugin: UnifastPlugin = {
      name: "uppercase",
      hastTransform: (hast: HastRoot): HastRoot => {
        function walk(node: HastRoot["children"][number]): HastRoot["children"][number] {
          if (node.type === "text") {
            return { ...node, value: node.value.toUpperCase() };
          }
          if ("children" in node) {
            return { ...node, children: node.children.map(walk) } as typeof node;
          }
          return node;
        }
        return { ...hast, children: hast.children.map(walk) };
      },
    };

    const result = compile("hello world", { plugins: [uppercasePlugin] });
    expect(result.output).toContain("HELLO WORLD");
  });

  it("compiles with mixed option + hastTransform plugins", () => {
    const gfmPlugin: UnifastPlugin = {
      name: "gfm",
      options: {
        gfm: {
          tables: true,
          taskList: true,
          strikethrough: true,
          footnotes: true,
          autolink: true,
        },
      },
    };

    const uppercasePlugin: UnifastPlugin = {
      name: "uppercase",
      hastTransform: (hast: HastRoot): HastRoot => {
        function walk(node: HastRoot["children"][number]): HastRoot["children"][number] {
          if (node.type === "text") {
            return { ...node, value: node.value.toUpperCase() };
          }
          if ("children" in node) {
            return { ...node, children: node.children.map(walk) } as typeof node;
          }
          return node;
        }
        return { ...hast, children: hast.children.map(walk) };
      },
    };

    const result = compile("| a | b |\n|---|---|\n| 1 | 2 |", {
      plugins: [gfmPlugin, uppercasePlugin],
    });

    expect(result.output).toContain("<table");
    expect(result.output).toContain("A");
    expect(result.output).toContain("B");
  });

  it("preserves hast output when user requests outputKind hast with transforms", () => {
    const plugin: UnifastPlugin = {
      name: "noop-transform",
      hastTransform: (hast: HastRoot) => hast,
    };

    const result = compile("hello", {
      outputKind: "hast",
      plugins: [plugin],
    });

    const parsed = JSON.parse(result.output as string) as { type: string; children: unknown[] };
    expect(parsed.type).toBe("root");
    expect(parsed.children).toBeDefined();
  });

  it("works with no plugins", () => {
    const result = compile("# Hello");
    expect(result.output).toContain("<h1");
    expect(result.output).toContain("Hello");
  });
});
