import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile, treeSitter } from "@unifast/node";
import { highlight } from "@unifast/highlight";
import { createShikiPlugin } from "@unifast/shiki";
import { createHighlightProcessor, createShikiProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const highlightPlugin = highlight();

const shikiPlugin = await createShikiPlugin({
  themes: "github-dark",
  langs: ["javascript", "typescript", "rust", "bash", "json", "html", "css", "python", "yaml", "toml"],
});

const treeSitterPlugin = treeSitter();

const unifiedHighlight = createHighlightProcessor();
const unifiedShiki = await createShikiProcessor();

describe("highlight.js: unifast vs unified (simple)", () => {
  bench("unifast (plugin-highlight)", () => { compile(simple, { plugins: [highlightPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(simple); });
});

describe("highlight.js: unifast vs unified (readme)", () => {
  bench("unifast (plugin-highlight)", () => { compile(readme, { plugins: [highlightPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(readme); });
});

describe("highlight.js: unifast vs unified (large)", () => {
  bench("unifast (plugin-highlight)", () => { compile(large, { plugins: [highlightPlugin] }); });
  bench("unified (rehype-highlight)", () => { unifiedHighlight.processSync(large); });
});

describe("shiki: unifast vs unified (simple)", () => {
  bench("unifast (plugin-shiki)", () => { compile(simple, { plugins: [shikiPlugin] }); });
  bench("unified (@shikijs/rehype)", async () => { await unifiedShiki.process(simple); });
});

describe("shiki: unifast vs unified (readme)", () => {
  bench("unifast (plugin-shiki)", () => { compile(readme, { plugins: [shikiPlugin] }); });
  bench("unified (@shikijs/rehype)", async () => { await unifiedShiki.process(readme); });
});

describe("shiki: unifast vs unified (large)", () => {
  bench("unifast (plugin-shiki)", () => { compile(large, { plugins: [shikiPlugin] }); });
  bench("unified (@shikijs/rehype)", async () => { await unifiedShiki.process(large); });
});

describe("tree-sitter: unifast (simple)", () => {
  bench("unifast (plugin-tree-sitter)", () => { compile(simple, { plugins: [treeSitterPlugin] }); });
});

describe("tree-sitter: unifast (readme)", () => {
  bench("unifast (plugin-tree-sitter)", () => { compile(readme, { plugins: [treeSitterPlugin] }); });
});

describe("tree-sitter: unifast (large)", () => {
  bench("unifast (plugin-tree-sitter)", () => { compile(large, { plugins: [treeSitterPlugin] }); });
});
