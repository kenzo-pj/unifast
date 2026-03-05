import type { UnifastPlugin } from "@unifast/core";

export type SyntectPluginOptions = {
  engine?: "none" | "syntect";
};

export function syntect(
  options?: SyntectPluginOptions,
): UnifastPlugin {
  return {
    name: "syntect",
    options: {
      highlight: {
        enabled: true,
        engine: options?.engine ?? "syntect",
      },
    },
  };
}
