import type { UnifastPlugin, SanitizeSchema } from "@unifast/core";

export type SanitizePluginOptions = {
  enabled?: boolean;
  schema?: SanitizeSchema;
};

export function sanitize(
  options?: SanitizePluginOptions,
): UnifastPlugin {
  return {
    name: "sanitize",
    options: {
      sanitize: {
        enabled: options?.enabled ?? true,
        schema: options?.schema,
      },
    },
  };
}
