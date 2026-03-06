import { defineConfig } from "tsdown";

import { shared } from "../tsdown.shared.mts";

export default defineConfig({
  ...shared,
  format: ["esm"],
});
