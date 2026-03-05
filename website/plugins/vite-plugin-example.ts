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

      const fenced = "```md\n" + source + "\n```\n";
      const code = options.compile(fenced, {
        ...opts,
        lineNumbers: { enabled: true },
        inputKind: "md",
        outputKind: "html",
      });

      return [
        `export const source = ${JSON.stringify(source)};`,
        `export const html = ${JSON.stringify(preview.output as string)};`,
        `export const codeHtml = ${JSON.stringify(code.output as string)};`,
        `export default { source, html, codeHtml };`,
      ].join("\n");
    },

    handleHotUpdate({ file, server }) {
      if (!file.endsWith(".md")) return;
      const exampleId = file + QUERY;
      const mod = server.moduleGraph.getModuleById(exampleId);
      if (mod) return [mod];
    },
  };
}
