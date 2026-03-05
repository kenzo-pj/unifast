import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile, treeSitter } from "@unifast/node";
import { createHighlightProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const treeSitterPlugin = treeSitter();
const unifiedHighlight = createHighlightProcessor();

describe("tree-sitter vs rehype-highlight (simple)", () => {
  bench("unifast (tree-sitter)", () => { compile(simple, { plugins: [treeSitterPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(simple); });
});

describe("tree-sitter vs rehype-highlight (readme)", () => {
  bench("unifast (tree-sitter)", () => { compile(readme, { plugins: [treeSitterPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(readme); });
});

describe("tree-sitter vs rehype-highlight (large)", () => {
  bench("unifast (tree-sitter)", () => { compile(large, { plugins: [treeSitterPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(large); });
});
