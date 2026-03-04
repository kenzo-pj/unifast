import type { CompileOptions } from "@unifast/core";

export type FrontmatterPluginOptions = {
  yaml?: boolean;
  toml?: boolean;
  json?: boolean;
};

export function frontmatter(
  options?: FrontmatterPluginOptions,
): Partial<CompileOptions> {
  return {
    frontmatter: {
      yaml: options?.yaml ?? true,
      toml: options?.toml ?? false,
      json: options?.json ?? false,
    },
  };
}
