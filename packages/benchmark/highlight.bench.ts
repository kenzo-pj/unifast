import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createHighlightProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const processor = createHighlightProcessor();

const highlightOptions = { highlight: { enabled: true, engine: "syntect" as const } };

describe("Highlight (simple)", () => {
  bench("unifast", () => {
    compile(simple, highlightOptions);
  });
  bench("unified", () => {
    processor.processSync(simple);
  });
});

describe("Highlight (readme)", () => {
  bench("unifast", () => {
    compile(readme, highlightOptions);
  });
  bench("unified", () => {
    processor.processSync(readme);
  });
});

describe("Highlight (large)", () => {
  bench("unifast", () => {
    compile(large, highlightOptions);
  });
  bench("unified", () => {
    processor.processSync(large);
  });
});
