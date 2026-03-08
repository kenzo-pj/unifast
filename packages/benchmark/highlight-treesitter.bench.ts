import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile, treeSitter } from "@unifast/node";
import { createHighlightProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");

const treeSitterPlugin = treeSitter();
const unifiedHighlight = createHighlightProcessor();

describe("tree-sitter vs rehype-highlight (readme)", () => {
  bench("unifast (tree-sitter)", () => { compile(readme, { plugins: [treeSitterPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(readme); });
});
