import type { UnifastPlugin } from "@unifast/core";
import { createShikiTransformer, type ShikiTransformerOptions } from "./transformer.js";

export async function createShikiPlugin(
  options?: ShikiTransformerOptions,
): Promise<UnifastPlugin> {
  const transformer = await createShikiTransformer(options);
  return {
    name: "shiki",
    options: {
      highlight: { enabled: false },
    },
    hastTransform: (hast) => transformer.transform(hast),
    mdxJsTransform: (js) => transformer.transformMdxJs(js),
  };
}
