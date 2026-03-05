import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile, syntect } from "@unifast/node";
import { createHighlightProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const syntectPlugin = syntect();
const unifiedHighlight = createHighlightProcessor();

describe("syntect vs rehype-highlight (simple)", () => {
  bench("unifast (syntect)", () => { compile(simple, { plugins: [syntectPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(simple); });
});

describe("syntect vs rehype-highlight (readme)", () => {
  bench("unifast (syntect)", () => { compile(readme, { plugins: [syntectPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(readme); });
});

describe("syntect vs rehype-highlight (large)", () => {
  bench("unifast (syntect)", () => { compile(large, { plugins: [syntectPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(large); });
});
