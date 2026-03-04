import type { CompileOptions, SanitizeSchema } from "@unifast/core";

export type SanitizePluginOptions = {
  enabled?: boolean;
  schema?: SanitizeSchema;
};

export function sanitize(
  options?: SanitizePluginOptions,
): Partial<CompileOptions> {
  return {
    sanitize: {
      enabled: options?.enabled ?? true,
      schema: options?.schema,
    },
  };
}
