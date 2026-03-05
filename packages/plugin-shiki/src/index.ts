export type {
  HastNode,
  HastRoot,
  HastElement,
  HastText,
  HastRaw,
  HastComment,
  HastDoctype,
} from "./types.js";

export { hastToHtml } from "./hast-utils.js";

export {
  createShikiTransformer,
  type ShikiTransformerOptions,
  type ShikiTransformer,
} from "./transformer.js";

export { createShikiPlugin } from "./plugin.js";
