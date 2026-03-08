import type { Plugin } from "vite";
import type { CompileOptions, CompileResult } from "@unifast/node";
import fs from "node:fs";
import path from "node:path";

type CompileFn = (input: string, opts?: CompileOptions) => CompileResult;

const QUERY = "?example";

export interface ExamplePluginOptions {
  compile: CompileFn;
  compileOptions?: CompileOptions;
}

interface CodeTab {
  label: string;
  html: string;
}

function compileCodeBlock(compile: CompileFn, opts: CompileOptions, source: string, lang: string): string {
  const fenced = "```" + lang + "\n" + source + "\n```\n";
  const result = compile(fenced, {
    ...opts,
    lineNumbers: { enabled: true },
    inputKind: "md",
    outputKind: "html",
  });
  return result.output as string;
}

export default function examplePlugin(options: ExamplePluginOptions): Plugin {
  return {
    name: "vite-plugin-example",
    enforce: "pre",

    resolveId(source, importer) {
      if (!source.endsWith(QUERY)) return null;
      const base = source.slice(0, -QUERY.length);
      const dir = importer ? path.dirname(importer) : process.cwd();
      return path.resolve(dir, base) + QUERY;
    },

    load(id) {
      if (!id.endsWith(QUERY)) return null;
      const filePath = id.slice(0, -QUERY.length);
      const source = fs.readFileSync(filePath, "utf-8");

      const opts = options.compileOptions ?? {};

      const preview = options.compile(source, {
        ...opts,
        inputKind: "md",
        outputKind: "html",
      });

      const mdFileName = path.basename(filePath);
      const codes: CodeTab[] = [
        { label: mdFileName, html: compileCodeBlock(options.compile, opts, source, "md") },
      ];

      const tsPath = filePath.replace(/\.md$/, ".ts");
      if (fs.existsSync(tsPath)) {
        const tsSource = fs.readFileSync(tsPath, "utf-8");
        const tsFileName = path.basename(tsPath);
        codes.push({ label: tsFileName, html: compileCodeBlock(options.compile, opts, tsSource, "ts") });
      }

      const mdCodeHtml = codes[0].html;

      return [
        `export const source = ${JSON.stringify(source)};`,
        `export const html = ${JSON.stringify(preview.output as string)};`,
        `export const codeHtml = ${JSON.stringify(mdCodeHtml)};`,
        `export const codes = ${JSON.stringify(codes)};`,
        `export default { source, html, codeHtml, codes };`,
      ].join("\n");
    },

    handleHotUpdate({ file, server }) {
      if (!file.endsWith(".md") && !file.endsWith(".ts")) return;
      const mdPath = file.endsWith(".ts") ? file.replace(/\.ts$/, ".md") : file;
      const exampleId = mdPath + QUERY;
      const mod = server.moduleGraph.getModuleById(exampleId);
      if (mod) return [mod];
    },
  };
}
