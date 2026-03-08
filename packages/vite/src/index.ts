import { createRequire } from "node:module";

import { escapeHtml } from "@unifast/core";
import matter from "gray-matter";
import type { Plugin } from "vite";

export interface UnifastPluginOptions {
  md?: CompileOptions;
  mdx?: CompileOptions;
}

type CompileOptions = Record<string, unknown>;

type CompileFn = (
  input: string,
  opts?: CompileOptions,
) => {
  output: string;
  frontmatter: Record<string, unknown>;
  toc: Array<{ depth: number; text: string; slug: string }>;
};

let compileFn: CompileFn | null = null;
let resolved = false;

function getCompile(): CompileFn | null {
  if (resolved) return compileFn;
  resolved = true;
  try {
    const require = createRequire(import.meta.url);
    compileFn = require("@unifast/node").compile;
  } catch {}
  return compileFn;
}

function extractFrontmatter(source: string): {
  frontmatter: Record<string, unknown>;
  body: string;
} {
  const { data, content } = matter(source);
  return { frontmatter: data, body: content };
}

export default function unifastPlugin(options: UnifastPluginOptions = {}): Plugin {
  return {
    name: "vite-plugin-unifast",
    enforce: "pre",

    transform(code, id) {
      if (!/\.(md|mdx)$/.test(id)) return null;

      const source = code;
      const isMdx = id.endsWith(".mdx");
      const compile = getCompile();

      if (isMdx) {
        const compileOpts = options.mdx ?? options.md ?? {};
        return compile ? transformMdx(source, compile, compileOpts) : null;
      }

      const compileOpts = options.md ?? {};
      return transformMd(source, compile, compileOpts);
    },

    handleHotUpdate({ file, server }) {
      if (/\.(md|mdx)$/.test(file)) {
        const mod = server.moduleGraph.getModuleById(file);
        if (mod) return [mod];
      }
    },
  };
}

function transformMd(source: string, compile: CompileFn | null, compileOpts: CompileOptions) {
  let html: string;
  let frontmatter: Record<string, unknown> = {};
  let tocData: Array<{ depth: number; text: string; slug: string }> = [];

  if (compile) {
    try {
      const result = compile(source, {
        ...compileOpts,
        inputKind: "md",
        outputKind: "html",
      });
      html = result.output;
      frontmatter = result.frontmatter ?? {};
      tocData = result.toc ?? [];
    } catch {
      html = `<pre>${escapeHtml(source)}</pre>`;
    }
  } else {
    const parsed = extractFrontmatter(source);
    ({ frontmatter } = parsed);
    html = `<div>${escapeHtml(parsed.body).replaceAll("\n", "<br>")}</div>`;
  }

  return {
    code: [
      `export const html = ${JSON.stringify(html)};`,
      `export const frontmatter = ${JSON.stringify(frontmatter)};`,
      `export const toc = ${JSON.stringify(tocData)};`,
      `export default { html, frontmatter, toc };`,
    ].join("\n"),
    map: null,
  };
}

function transformMdx(source: string, compile: CompileFn, compileOpts: CompileOptions) {
  const result = compile(source, {
    ...compileOpts,
    inputKind: "mdx",
    outputKind: "mdxJs",
  });

  return {
    code: [
      `import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";`,
      result.output,
      `export const frontmatter = ${JSON.stringify(result.frontmatter ?? {})};`,
      `export const toc = ${JSON.stringify(result.toc ?? [])};`,
    ].join("\n"),
    map: null,
  };
}
