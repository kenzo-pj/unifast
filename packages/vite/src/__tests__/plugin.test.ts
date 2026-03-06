import fs from "node:fs";

import { describe, it, expect, vi, beforeEach } from "vitest";

// ---------------------------------------------------------------------------
// Fallback tests: @unifast/node is NOT available
// ---------------------------------------------------------------------------
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

  // ---- 1. Plugin metadata ------------------------------------------------
  it("has name 'vite-plugin-unifast'", async () => {
    const plugin = await getPlugin();
    expect(plugin.name).toBe("vite-plugin-unifast");
  });

  it("has enforce 'pre'", async () => {
    const plugin = await getPlugin();
    expect(plugin.enforce).toBe("pre");
  });

  // ---- 2. Hook existence --------------------------------------------------
  it("has a transform hook", async () => {
    const plugin = await getPlugin();
    expect(typeof plugin.transform).toBe("function");
  });

  it("has a handleHotUpdate hook", async () => {
    const plugin = await getPlugin();
    expect(typeof plugin.handleHotUpdate).toBe("function");
  });

  // ---- 3. transform returns null for non-md files -------------------------
  it.each([".ts", ".js", ".css", ".html", ".json"])(
    "transform returns null for %s files",
    async (ext) => {
      const plugin = await getPlugin();
      const transform = plugin.transform as Function;
      const result = transform.call({}, "", `file${ext}`);
      expect(result).toBeNull();
    },
  );

  // ---- 4. Case-sensitive: .MD should not match ----------------------------
  it("transform returns null for .MD (case-sensitive regex)", async () => {
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "FILE.MD");
    expect(result).toBeNull();
  });

  // ---- 5 & 6. .md fallback transform structure ----------------------------
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

  // ---- 7. Frontmatter parsing (simple) ------------------------------------
  it("parses simple frontmatter key-value pairs", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\ntitle: Hello\n---\nBody text");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain('"title":"Hello"');
  });

  // ---- 8. No frontmatter → empty object -----------------------------------
  it("returns empty frontmatter when none present", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("Just a body");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain("export const frontmatter = {};");
  });

  // ---- 9. Windows line endings (\r\n) -------------------------------------
  it("handles Windows line endings in frontmatter", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\r\ntitle: Win\r\n---\r\nBody");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain('"title":"Win"');
  });

  // ---- 10. Value with colons (split on first colon only) ------------------
  it("splits frontmatter only on the first colon", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\ntime: 12:30:45\n---\nBody");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain('"time":45045');
  });

  // ---- 11. Key with empty value -------------------------------------------
  it("handles frontmatter key with empty value", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\ndraft:\n---\nBody");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain('"draft":null');
  });

  // ---- 12. HTML escaping (< → &lt;) ---------------------------------------
  it("escapes < to &lt; in fallback HTML", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("<script>alert(1)</script>");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain("&lt;");
    expect(code).not.toContain("<script>");
  });

  // ---- 13. Newlines become <br> -------------------------------------------
  it("converts newlines to <br> in fallback HTML", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("line1\nline2\nline3");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    const { code } = result;
    expect(code).toContain("<br>");
  });

  // ---- 14. Empty body ----------------------------------------------------
  it("handles empty body (frontmatter only)", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("---\ntitle: OnlyMeta\n---\n");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "test.md");

    expect(result).not.toBeNull();
    expect(result.code).toContain("export const html =");
  });

  // ---- 15. .mdx returns null when compiler unavailable --------------------
  it(".mdx transform returns null when compiler is not available", async () => {
    vi.spyOn(fs, "readFileSync").mockReturnValue("# Hello MDX");
    const plugin = await getPlugin();
    const transform = plugin.transform as Function;
    const result = transform.call({}, "", "component.mdx");

    expect(result).toBeNull();
  });

  // ---- 16. handleHotUpdate returns module for .md -------------------------
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

  // ---- 17. handleHotUpdate returns module for .mdx ------------------------
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

  // ---- 18. handleHotUpdate returns undefined for non-md -------------------
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

  // ---- 19. handleHotUpdate returns undefined when module not in graph ------
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

// ---------------------------------------------------------------------------
// Compiler-available tests: @unifast/node IS found
// ---------------------------------------------------------------------------
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

  // ---- 20. .md transform uses compiler output -----------------------------
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

  // ---- 21. .md transform handles compiler throwing -------------------------
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

  // ---- 22. .mdx transform produces JSX imports ----------------------------
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
});
