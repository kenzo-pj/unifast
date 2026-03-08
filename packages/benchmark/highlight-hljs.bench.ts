import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { highlight } from "@unifast/highlight";
import { createHighlightProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");

const highlightPlugin = highlight();
const unifiedHighlight = createHighlightProcessor();

describe("highlight.js: unifast vs unified (readme)", () => {
  bench("unifast (plugin-highlight)", () => { compile(readme, { plugins: [highlightPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(readme); });
});
