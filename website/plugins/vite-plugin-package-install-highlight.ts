import type { Plugin } from "vite";

const MANAGERS = [
  { id: "npm", prefix: "npm install" },
  { id: "yarn", prefix: "yarn add" },
  { id: "pnpm", prefix: "pnpm add" },
  { id: "bun", prefix: "bun add" },
];

interface Options {
  compile: (input: string, opts?: object) => { output: string | object };
  plugins?: unknown[];
}

export default function packageInstallHighlightPlugin({ compile, plugins }: Options): Plugin {
  return {
    name: "vite-plugin-package-install-highlight",
    enforce: "post" as const,

    transform(code, id) {
      if (!/\.(?:mdx|tsx?)$/.test(id)) return null;

      const pattern = /(\{\s*package:\s*)"([^"]+)"(\s*\})/g;
      const replacements: Array<{ start: number; end: number; replacement: string }> = [];

      let m: RegExpExecArray | null;
      while ((m = pattern.exec(code)) !== null) {
        const before = code.slice(Math.max(0, m.index - 100), m.index);
        if (!before.includes("PackageInstall")) continue;

        const pkg = m[2];
        const highlighted: Record<string, string> = {};

        for (const mgr of MANAGERS) {
          const cmd = `${mgr.prefix} ${pkg}`;
          const md = "```sh\n" + cmd + "\n```";
          try {
            const result = compile(md, { plugins });
            const output = typeof result.output === "string" ? result.output : "";
            const inner = output.match(/<code[^>]*>([\s\S]*?)<\/code>/);
            highlighted[mgr.id] = inner ? inner[1] : cmd;
          } catch {
            highlighted[mgr.id] = cmd;
          }
        }

        const replacement = `${m[1]}"${pkg}", highlighted: ${JSON.stringify(highlighted)}${m[3]}`;
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
