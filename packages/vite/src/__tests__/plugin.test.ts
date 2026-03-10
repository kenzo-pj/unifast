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
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "Hello world", "test.md");

    expect(result).not.toBeNull();
    expect(result.code).toContain("export const html =");
    expect(result.code).toContain("export const frontmatter =");
    expect(result.code).toContain("export const toc =");
    expect(result.code).toContain("export default ");
  });

  it(".md transform returns map: null", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "Hello", "test.md");

    expect(result.map).toBeNull();
  });

  it("parses simple frontmatter key-value pairs", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "---\ntitle: Hello\n---\nBody text", "test.md");

    const { code } = result;
    expect(code).toContain('"title":"Hello"');
  });

  it("returns empty frontmatter when none present", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "Just a body", "test.md");

    const { code } = result;
    expect(code).toContain("export const frontmatter = {};");
  });

  it("handles Windows line endings in frontmatter", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "---\r\ntitle: Win\r\n---\r\nBody", "test.md");

    const { code } = result;
    expect(code).toContain('"title":"Win"');
  });

  it("splits frontmatter only on the first colon", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "---\ntime: 12:30:45\n---\nBody", "test.md");

    const { code } = result;
    expect(code).toContain('"time":45045');
  });

  it("handles frontmatter key with empty value", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "---\ndraft:\n---\nBody", "test.md");

    const { code } = result;
    expect(code).toContain('"draft":null');
  });

  it("escapes < to &lt; in fallback HTML", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "<script>alert(1)</script>", "test.md");

    const { code } = result;
    expect(code).toContain("&lt;");
    expect(code).not.toContain("<script>");
  });

  it("converts newlines to <br> in fallback HTML", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "line1\nline2\nline3", "test.md");

    const { code } = result;
    expect(code).toContain("<br>");
  });

  it("handles empty body (frontmatter only)", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "---\ntitle: OnlyMeta\n---\n", "test.md");

    expect(result).not.toBeNull();
    expect(result.code).toContain("export const html =");
  });

  it(".mdx transform returns null when compiler is not available", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "# Hello MDX", "component.mdx");

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
    mockCompile.mockReturnValue({
      output: "<h1>Hello</h1>",
      frontmatter: { title: "Hello" },
      toc: [{ depth: 1, text: "Hello", slug: "hello" }],
    });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "# Hello", "page.md");

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
    mockCompile.mockImplementation(() => {
      throw new Error("compile error");
    });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "Bad <content>", "broken.md");

    expect(result).not.toBeNull();
    expect(result.code).toContain("<pre>");
    expect(result.code).toContain("&lt;");
  });

  it(".mdx transform produces JSX imports and passes compiler output through", async () => {
    mockCompile.mockReturnValue({
      output: [
        "function MDXContent({ components: _components = {}, ...props }) {",
        "const _c = (t) => _components[t] || t;",
        '  return _jsx(_c("h1"), { children: "MDX Content" });',
        "}",
        "export default MDXContent;",
      ].join("\n"),
      frontmatter: { layout: "post" },
      toc: [{ depth: 1, text: "MDX Content", slug: "mdx-content" }],
    });

    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "# MDX Content", "doc.mdx");

    expect(result).not.toBeNull();
    expect(result.code).toContain(
      'import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime"',
    );
    expect(result.code).toContain("export const frontmatter =");
    expect(result.code).toContain("export const toc =");
    expect(result.code).toContain("_c(");
    expect(result.code).toContain("export default MDXContent;");
    expect(mockCompile).toHaveBeenCalledWith(
      "# MDX Content",
      expect.objectContaining({
        inputKind: "mdx",
        outputKind: "mdxJs",
      }),
    );
  });
});
