import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createCombinedProcessor } from "./setup/unified.js";

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

const processor = createCombinedProcessor();

const combinedOptions = {
  gfm: {
    tables: true,
    taskList: true,
    strikethrough: true,
    autolink: true,
  },
  frontmatter: { yaml: true },
  sanitize: { enabled: true },
  highlight: { enabled: true, engine: "syntect" as const },
  toc: { enabled: true },
  slug: { mode: "github" as const },
};

describe("Combined (simple)", () => {
  bench("unifast", () => {
    compile(simple, combinedOptions);
  });
  bench("unified", () => {
    processor.processSync(simple);
  });
});

describe("Combined (readme)", () => {
  bench("unifast", () => {
    compile(readme, combinedOptions);
  });
  bench("unified", () => {
    processor.processSync(readme);
  });
});

describe("Combined (large)", () => {
  bench("unifast", () => {
    compile(large, combinedOptions);
  });
  bench("unified", () => {
    processor.processSync(large);
  });
});
