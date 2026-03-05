import type { UnifastPlugin } from "@unifast/core";

export type FrontmatterPluginOptions = {
  yaml?: boolean;
  toml?: boolean;
  json?: boolean;
};

export function frontmatter(
  options?: FrontmatterPluginOptions,
): UnifastPlugin {
  return {
    name: "frontmatter",
    options: {
      frontmatter: {
        yaml: options?.yaml ?? true,
        toml: options?.toml ?? false,
        json: options?.json ?? false,
      },
    },
  };
}
