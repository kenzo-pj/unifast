import type { Plugin } from "vite";

const MANAGERS = [
  { id: "npm", prefix: "npm install" },
  { id: "yarn", prefix: "yarn add" },
  { id: "pnpm", prefix: "pnpm add" },
  { id: "bun", prefix: "bun add" },
];

export default function packageInstallHighlightPlugin(): Plugin {
  let compileFn: ((input: string, opts?: object) => { output: string }) | null = null;

  return {
    name: "vite-plugin-package-install-highlight",

    configResolved() {
      try {
        // Use createRequire to load the native binding (CJS)
        const { createRequire } = require("node:module");
        const req = createRequire(import.meta.url);
        compileFn = req("@unifast/node").compile;
      } catch {
        // Native binding not available — skip highlighting
      }
    },

    transform(code, id) {
      if (!/\.mdx$/.test(id) || !compileFn) return null;

      // Match: _jsx(PackageInstall, { package: "..." })
      const pattern = /_jsx\(PackageInstall,\s*\{\s*package:\s*"([^"]+)"\s*\}\)/g;
      const replacements: Array<{ start: number; end: number; replacement: string }> = [];

      let m: RegExpExecArray | null;
      while ((m = pattern.exec(code)) !== null) {
        const pkg = m[1];
        const highlighted: Record<string, string> = {};

        for (const mgr of MANAGERS) {
          const cmd = `${mgr.prefix} ${pkg}`;
          const md = "```sh\n" + cmd + "\n```";
          try {
            const result = compileFn(md, {
              highlight: { enabled: true, engine: "syntect" },
            });
            // Extract inner HTML from <pre><code ...>CONTENT</code></pre>
            const inner = result.output.match(/<code[^>]*>([\s\S]*?)<\/code>/);
            highlighted[mgr.id] = inner ? inner[1] : cmd;
          } catch {
            highlighted[mgr.id] = cmd;
          }
        }

        const replacement = `_jsx(PackageInstall, { package: "${pkg}", highlighted: ${JSON.stringify(highlighted)} })`;
        replacements.push({ start: m.index, end: m.index + m[0].length, replacement });
      }

      if (replacements.length === 0) return null;

      let result = code;
      for (let i = replacements.length - 1; i >= 0; i--) {
        const r = replacements[i];
        result = result.slice(0, r.start) + r.replacement + result.slice(r.end);
      }

      return { code: result, map: null };
    },
  };
}
