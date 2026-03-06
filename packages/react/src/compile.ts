import type { CompileOptions, CompileResult, HastRoot, TocEntry } from "@unifast/core";
import { compile } from "@unifast/node";

import { hastToReact } from "./hast-to-react";
import type { CreateElement, ComponentMap } from "./hast-to-react";

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
  const { components, createElement, Fragment, ...compileOpts } = options;

  const result = compile(input, { ...compileOpts, outputKind: "hast" });

  let hast: HastRoot;
  if (typeof result.output === "string") {
    try {
      hast = JSON.parse(result.output) as HastRoot;
    } catch {
      throw new Error("Failed to parse HAST output from compiler");
    }
  } else {
    hast = result.output as HastRoot;
  }

  const element = hastToReact(hast, { createElement, Fragment, components });

  return {
    element,
    frontmatter: result.frontmatter,
    diagnostics: result.diagnostics,
    stats: result.stats,
    toc: result.toc,
  };
}
