import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const unifast = require("@unifast/node");

const compileOptions = {
  toc: { enabled: true, maxDepth: 3 },
  lineNumbers: { enabled: true },
  plugins: [
    unifast.frontmatter(),
    unifast.treeSitter(),
    unifast.externalLinks({ target: "_blank" }),
    unifast.autolinkHeadings({ behavior: "prepend" }),
    unifast.githubAlert(),
    unifast.emoji(),
    unifast.smartypants(),
    unifast.breaks(),
    unifast.cjk(),
  ],
};

export function compileMd(source: string) {
  const result = unifast.compile(source, {
    ...compileOptions,
    inputKind: "md",
    outputKind: "html",
  });
  return {
    html: result.output as string,
    frontmatter: (result.frontmatter ?? {}) as Record<string, unknown>,
    toc: (result.toc ?? []) as Array<{ depth: number; text: string; slug: string }>,
  };
}
