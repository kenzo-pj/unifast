import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createBasicProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");

const baseline = createBasicProcessor();

const sectionizeOpts = { sectionize: { enabled: true } };
const definitionListOpts = { definitionList: { enabled: true } };
const rubyAnnotationOpts = { rubyAnnotation: { enabled: true } };
const cjkOpts = { cjk: { enabled: true } };

describe("Sectionize (readme)", () => {
  bench("unifast", () => { compile(readme, sectionizeOpts); });
  bench("unified (baseline)", () => { baseline.processSync(readme); });
});

describe("Definition List (readme)", () => {
  bench("unifast", () => { compile(readme, definitionListOpts); });
  bench("unified (baseline)", () => { baseline.processSync(readme); });
});

describe("Ruby Annotation (readme)", () => {
  bench("unifast", () => { compile(readme, rubyAnnotationOpts); });
  bench("unified (baseline)", () => { baseline.processSync(readme); });
});

describe("CJK (readme)", () => {
  bench("unifast", () => { compile(readme, cjkOpts); });
  bench("unified (baseline)", () => { baseline.processSync(readme); });
});
