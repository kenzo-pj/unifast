import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createFrontmatterProcessor } from "./setup/unified.js";

const frontmatterBlock = `---
title: Benchmark Document
author: unifast
date: 2026-03-04
tags: [benchmark, markdown, performance]
description: A test document for benchmarking.
---

`;

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple =
  frontmatterBlock + readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme =
  frontmatterBlock + readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large =
  frontmatterBlock + readFileSync(join(fixturesDir, "large.md"), "utf-8");

const processor = createFrontmatterProcessor();

const frontmatterOptions = { frontmatter: { yaml: true } };

describe("Frontmatter (simple)", () => {
  bench("unifast", () => {
    compile(simple, frontmatterOptions);
  });
  bench("unified", () => {
    processor.processSync(simple);
  });
});

describe("Frontmatter (readme)", () => {
  bench("unifast", () => {
    compile(readme, frontmatterOptions);
  });
  bench("unified", () => {
    processor.processSync(readme);
  });
});

describe("Frontmatter (large)", () => {
  bench("unifast", () => {
    compile(large, frontmatterOptions);
  });
  bench("unified", () => {
    processor.processSync(large);
  });
});
