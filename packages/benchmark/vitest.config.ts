import codspeedPlugin from "@codspeed/vitest-plugin";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [codspeedPlugin()],
  test: {
    pool: "forks",
    benchmark: {},
    testTimeout: 900_000,
    hookTimeout: 120_000,
    teardownTimeout: 120_000,
  },
});
