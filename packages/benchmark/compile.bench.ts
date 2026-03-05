import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createBasicProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const processor = createBasicProcessor();

describe("Markdown to HTML (simple)", () => {
  bench("unifast", () => {
    compile(simple);
  });
  bench("unified", () => {
    processor.processSync(simple);
  });
});

describe("Markdown to HTML (readme)", () => {
  bench("unifast", () => {
    compile(readme);
  });
  bench("unified", () => {
    processor.processSync(readme);
  });
});

describe("Markdown to HTML (large)", () => {
  bench("unifast", () => {
    compile(large);
  });
  bench("unified", () => {
    processor.processSync(large);
  });
});
