import fs from "node:fs";
import { createRequire } from "node:module";

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
  const match = source.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n([\s\S]*)$/);
  if (!match) return { frontmatter: {}, body: source };

  const raw = match[1];
  const body = match[2];
  const frontmatter: Record<string, unknown> = {};

  for (const line of raw.split("\n")) {
    const idx = line.indexOf(":");
    if (idx > 0) {
      const key = line.slice(0, idx).trim();
      const value = line.slice(idx + 1).trim();
      frontmatter[key] = value;
    }
  }

  return { frontmatter, body };
}

export default function unifastPlugin(options: UnifastPluginOptions = {}): Plugin {
  return {
    name: "vite-plugin-unifast",
    enforce: "pre",

    transform(_code, id) {
      if (!/\.(md|mdx)$/.test(id)) return null;

      const source = fs.readFileSync(id, "utf8");
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
      html = `<pre>${source.replaceAll("<", "&lt;")}</pre>`;
    }
  } else {
    const parsed = extractFrontmatter(source);
    ({ frontmatter } = parsed);
    html = `<div>${parsed.body.replaceAll("<", "&lt;").replaceAll("\n", "<br>")}</div>`;
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

function highlightMdxCodeBlocks(
  jsOutput: string,
  compile: CompileFn,
  compileOpts: CompileOptions,
): string {
  return jsOutput.replaceAll(
    /_jsx\("pre", \{\s*children:\s*_jsx\("code", \{\s*children:\s*"((?:[^"\\]|\\.)*)"\s*,\s*className:\s*"language-(\w+)"\s*\}\)\s*\}\)/g,
    (match, rawCode, rawLang) => {
      try {
        const escapedCode = String(rawCode);
        const lang = String(rawLang);
        const code = escapedCode
          .replaceAll("\\n", "\n")
          .replaceAll('\\"', '"')
          .replaceAll("\\\\", "\\");

        const result = compile(`\`\`\`${lang}\n${code}\n\`\`\`\n`, {
          ...compileOpts,
          inputKind: "md",
          outputKind: "html",
        });

        return `_jsx("div", { dangerouslySetInnerHTML: { __html: ${JSON.stringify(result.output)} } })`;
      } catch {
        return match;
      }
    },
  );
}

function transformMdx(source: string, compile: CompileFn, compileOpts: CompileOptions) {
  const result = compile(source, {
    ...compileOpts,
    inputKind: "mdx",
    outputKind: "mdxJs",
  });

  const highlighted = highlightMdxCodeBlocks(result.output, compile, compileOpts);

  return {
    code: [
      `import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";`,
      highlighted,
      `export const frontmatter = ${JSON.stringify(result.frontmatter ?? {})};`,
      `export const toc = ${JSON.stringify(result.toc ?? [])};`,
    ].join("\n"),
    map: null,
  };
}
