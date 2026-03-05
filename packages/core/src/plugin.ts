import type { HastRoot } from "./hast.js";
import type { CompileOptions } from "./options.js";

export type UnifastPlugin = {
  name: string;
  options?: Partial<CompileOptions>;
  hastTransform?: (hast: HastRoot) => HastRoot;
  mdxJsTransform?: (js: string) => string;
};
