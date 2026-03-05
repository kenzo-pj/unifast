import type { UnifastPlugin } from "@unifast/core";

export type TocPluginOptions = {
  maxDepth?: number;
};

export function toc(options?: TocPluginOptions): UnifastPlugin {
  return {
    name: "toc",
    options: {
      toc: {
        enabled: true,
        maxDepth: options?.maxDepth ?? 3,
      },
    },
  };
}
