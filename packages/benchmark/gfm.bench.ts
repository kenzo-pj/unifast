import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createGfmProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const processor = createGfmProcessor();

const gfmOptions = {
  gfm: {
    tables: true,
    taskList: true,
    strikethrough: true,
    autolink: true,
  },
};

describe("GFM (simple)", () => {
  bench("unifast", () => {
    compile(simple, gfmOptions);
  });
  bench("unified", () => {
    processor.processSync(simple);
  });
});

describe("GFM (readme)", () => {
  bench("unifast", () => {
    compile(readme, gfmOptions);
  });
  bench("unified", () => {
    processor.processSync(readme);
  });
});

describe("GFM (large)", () => {
  bench("unifast", () => {
    compile(large, gfmOptions);
  });
  bench("unified", () => {
    processor.processSync(large);
  });
});
