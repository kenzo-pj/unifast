import fs from "node:fs";
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

        const innerHtml = result.output.replace(/^<pre[^>]*>/, "").replace(/<\/pre>\s*$/, "");
        return `_jsx("pre", { __rawCode: ${JSON.stringify(code)}, dangerouslySetInnerHTML: { __html: ${JSON.stringify(innerHtml)} } })`;
      } catch {
        return match;
      }
    },
  );
}

function injectComponentsSupport(jsOutput: string): string {
  const replaced = jsOutput.replace(
    /function MDXContent\(props\) \{/,
    "function MDXContent({ components: _components = {}, ...props }) {\nconst _c = (t) => _components[t] || t;",
  );
  if (replaced === jsOutput) return jsOutput;
  return replaced
    .replaceAll(/_jsx\("([a-z][a-z0-9]*)"/g, '_jsx(_c("$1")')
    .replaceAll(/_jsxs\("([a-z][a-z0-9]*)"/g, '_jsxs(_c("$1")');
}

function transformMdx(source: string, compile: CompileFn, compileOpts: CompileOptions) {
  const result = compile(source, {
    ...compileOpts,
    inputKind: "mdx",
    outputKind: "mdxJs",
  });

  const highlighted = highlightMdxCodeBlocks(result.output, compile, compileOpts);
  const withComponents = injectComponentsSupport(highlighted);

  return {
    code: [
      `import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";`,
      withComponents,
      `export const frontmatter = ${JSON.stringify(result.frontmatter ?? {})};`,
      `export const toc = ${JSON.stringify(result.toc ?? [])};`,
    ].join("\n"),
    map: null,
  };
}
