import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createSanitizeProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const processor = createSanitizeProcessor();

const sanitizeOptions = { sanitize: { enabled: true } };

describe("Sanitize (simple)", () => {
  bench("unifast", () => {
    compile(simple, sanitizeOptions);
  });
  bench("unified", () => {
    processor.processSync(simple);
  });
});

describe("Sanitize (readme)", () => {
  bench("unifast", () => {
    compile(readme, sanitizeOptions);
  });
  bench("unified", () => {
    processor.processSync(readme);
  });
});

describe("Sanitize (large)", () => {
  bench("unifast", () => {
    compile(large, sanitizeOptions);
  });
  bench("unified", () => {
    processor.processSync(large);
  });
});
