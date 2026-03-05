import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createTocProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const processor = createTocProcessor();

const tocOptions = { toc: { enabled: true }, slug: { mode: "github" as const } };

describe("TOC + Slug (simple)", () => {
  bench("unifast", () => {
    compile(simple, tocOptions);
  });
  bench("unified", () => {
    processor.processSync(simple);
  });
});

describe("TOC + Slug (readme)", () => {
  bench("unifast", () => {
    compile(readme, tocOptions);
  });
  bench("unified", () => {
    processor.processSync(readme);
  });
});

describe("TOC + Slug (large)", () => {
  bench("unifast", () => {
    compile(large, tocOptions);
  });
  bench("unified", () => {
    processor.processSync(large);
  });
});
