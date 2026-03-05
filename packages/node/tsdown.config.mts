import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  dts: true,
  clean: true,
  exports: true,
  deps: {
    neverBundle: [/\.node$/],
  },
});
