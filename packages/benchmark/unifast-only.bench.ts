import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createBasicProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const allFeatures = readFileSync(join(fixturesDir, "all-features.md"), "utf-8");

const baseline = createBasicProcessor();

const sectionizeOpts = { sectionize: { enabled: true } };
const definitionListOpts = { definitionList: { enabled: true } };
const rubyAnnotationOpts = { rubyAnnotation: { enabled: true } };
const cjkOpts = { cjk: { enabled: true } };

describe("Sectionize (readme)", () => {
  bench("unifast", () => { compile(readme, sectionizeOpts); });
  bench("unified (baseline)", () => { baseline.processSync(readme); });
});

describe("Definition List (all-features)", () => {
  bench("unifast", () => { compile(allFeatures, definitionListOpts); });
  bench("unified (baseline)", () => { baseline.processSync(allFeatures); });
});

describe("Ruby Annotation (all-features)", () => {
  bench("unifast", () => { compile(allFeatures, rubyAnnotationOpts); });
  bench("unified (baseline)", () => { baseline.processSync(allFeatures); });
});

describe("CJK (all-features)", () => {
  bench("unifast", () => { compile(allFeatures, cjkOpts); });
  bench("unified (baseline)", () => { baseline.processSync(allFeatures); });
});
