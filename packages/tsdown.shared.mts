import type { Options } from "tsdown";

export const shared: Options = {
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  dts: true,
  clean: true,
  exports: true,
};
