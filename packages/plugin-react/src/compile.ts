import type { CompileOptions, CompileResult, HastRoot, TocEntry } from "@unifast/core";
import { hastToReact, type CreateElement, type ComponentMap } from "./hast-to-react.js";

export type CompileToReactOptions = CompileOptions & {
  components?: ComponentMap;
  createElement: CreateElement;
  Fragment: unknown;
};

export type CompileToReactResult = {
  element: unknown;
  frontmatter: Record<string, unknown>;
  diagnostics: CompileResult["diagnostics"];
  stats: CompileResult["stats"];
  toc: TocEntry[];
};

export function compileToReact(
  input: string,
  options: CompileToReactOptions,
): CompileToReactResult {
  // Dynamic import avoidance: require @unifast/node at call time
  let compile: (input: string, options?: CompileOptions) => CompileResult;
  try {
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    const node = require("@unifast/node");
    compile = node.compile;
  } catch {
    throw new Error(
      "@unifast/node is required for compileToReact. " +
        "Install it with: pnpm add @unifast/node",
    );
  }

  const { components, createElement, Fragment, ...compileOpts } = options;

  // Force HAST output so we can convert to React elements
  const result = compile(input, { ...compileOpts, outputKind: "hast" });

  const hast: HastRoot = typeof result.output === "string"
    ? JSON.parse(result.output)
    : result.output as HastRoot;

  const element = hastToReact(hast, { createElement, Fragment, components });

  return {
    element,
    frontmatter: result.frontmatter,
    diagnostics: result.diagnostics,
    stats: result.stats,
    toc: result.toc,
  };
}
