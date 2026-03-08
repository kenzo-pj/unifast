import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile, syntect } from "@unifast/node";
import { createHighlightProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");

const syntectPlugin = syntect();
const unifiedHighlight = createHighlightProcessor();

describe("syntect vs rehype-highlight (readme)", () => {
  bench("unifast (syntect)", () => { compile(readme, { plugins: [syntectPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(readme); });
});
