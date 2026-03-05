import type { UnifastPlugin } from "@unifast/core";

export type GfmPluginOptions = {
  tables?: boolean;
  taskList?: boolean;
  strikethrough?: boolean;
  footnotes?: boolean;
  autolink?: boolean;
};

export function gfm(options?: GfmPluginOptions): UnifastPlugin {
  return {
    name: "gfm",
    options: {
      gfm: {
        tables: options?.tables ?? true,
        taskList: options?.taskList ?? true,
        strikethrough: options?.strikethrough ?? true,
        footnotes: options?.footnotes ?? true,
        autolink: options?.autolink ?? true,
      },
    },
  };
}
