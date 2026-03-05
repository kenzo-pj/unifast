import type { HastRoot } from "./hast";
import type { CompileOptions } from "./options";

export type UnifastPlugin = {
  name: string;
  options?: Partial<CompileOptions>;
  hastTransform?: (hast: HastRoot) => HastRoot;
  mdxJsTransform?: (js: string) => string;
};
