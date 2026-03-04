import type { CompileOptions } from "@unifast/core";

export type HighlightPluginOptions = {
  engine?: "none" | "builtin";
};

export function highlight(
  options?: HighlightPluginOptions,
): Partial<CompileOptions> {
  return {
    highlight: {
      enabled: true,
      engine: options?.engine ?? "builtin",
    },
  };
}
