import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile, gfm, frontmatter, sanitize, syntect, toc } from "@unifast/node";
import {
  createGfmProcessor,
  createFrontmatterProcessor,
  createSanitizeProcessor,
  createHighlightProcessor,
  createTocProcessor,
  createCombinedProcessor,
} from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");

const frontmatterBlock = `---
title: Benchmark Document
author: unifast
date: 2026-03-04
tags: [benchmark, markdown, performance]
description: A test document for benchmarking.
---

`;

const fmSimple = frontmatterBlock + simple;
const fmReadme = frontmatterBlock + readme;
const fmLarge = frontmatterBlock + large;

const gfmPlugin = gfm();
const fmPlugin = frontmatter();
const sanitizePlugin = sanitize();
const syntectPlugin = syntect();
const tocPlugin = toc();

const unifiedGfm = createGfmProcessor();
const unifiedFm = createFrontmatterProcessor();
const unifiedSanitize = createSanitizeProcessor();
const unifiedHighlight = createHighlightProcessor();
const unifiedToc = createTocProcessor();
const unifiedCombined = createCombinedProcessor();

describe("Plugin: GFM (simple)", () => {
  bench("unifast", () => { compile(simple, { plugins: [gfmPlugin] }); });
  bench("unified", () => { unifiedGfm.processSync(simple); });
});

describe("Plugin: GFM (readme)", () => {
  bench("unifast", () => { compile(readme, { plugins: [gfmPlugin] }); });
  bench("unified", () => { unifiedGfm.processSync(readme); });
});

describe("Plugin: GFM (large)", () => {
  bench("unifast", () => { compile(large, { plugins: [gfmPlugin] }); });
  bench("unified", () => { unifiedGfm.processSync(large); });
});

describe("Plugin: Frontmatter (simple)", () => {
  bench("unifast", () => { compile(fmSimple, { plugins: [fmPlugin] }); });
  bench("unified", () => { unifiedFm.processSync(fmSimple); });
});

describe("Plugin: Frontmatter (readme)", () => {
  bench("unifast", () => { compile(fmReadme, { plugins: [fmPlugin] }); });
  bench("unified", () => { unifiedFm.processSync(fmReadme); });
});

describe("Plugin: Frontmatter (large)", () => {
  bench("unifast", () => { compile(fmLarge, { plugins: [fmPlugin] }); });
  bench("unified", () => { unifiedFm.processSync(fmLarge); });
});

describe("Plugin: Sanitize (simple)", () => {
  bench("unifast", () => { compile(simple, { plugins: [sanitizePlugin] }); });
  bench("unified", () => { unifiedSanitize.processSync(simple); });
});

describe("Plugin: Sanitize (readme)", () => {
  bench("unifast", () => { compile(readme, { plugins: [sanitizePlugin] }); });
  bench("unified", () => { unifiedSanitize.processSync(readme); });
});

describe("Plugin: Sanitize (large)", () => {
  bench("unifast", () => { compile(large, { plugins: [sanitizePlugin] }); });
  bench("unified", () => { unifiedSanitize.processSync(large); });
});

describe("Plugin: Highlight (simple)", () => {
  bench("unifast", () => { compile(simple, { plugins: [syntectPlugin] }); });
  bench("unified", () => { unifiedHighlight.processSync(simple); });
});

describe("Plugin: Highlight (readme)", () => {
  bench("unifast", () => { compile(readme, { plugins: [syntectPlugin] }); });
  bench("unified", () => { unifiedHighlight.processSync(readme); });
});

describe("Plugin: Highlight (large)", () => {
  bench("unifast", () => { compile(large, { plugins: [syntectPlugin] }); });
  bench("unified", () => { unifiedHighlight.processSync(large); });
});

describe("Plugin: TOC (simple)", () => {
  bench("unifast", () => { compile(simple, { plugins: [tocPlugin] }); });
  bench("unified", () => { unifiedToc.processSync(simple); });
});

describe("Plugin: TOC (readme)", () => {
  bench("unifast", () => { compile(readme, { plugins: [tocPlugin] }); });
  bench("unified", () => { unifiedToc.processSync(readme); });
});

describe("Plugin: TOC (large)", () => {
  bench("unifast", () => { compile(large, { plugins: [tocPlugin] }); });
  bench("unified", () => { unifiedToc.processSync(large); });
});

describe("Plugin: Combined (simple)", () => {
  bench("unifast", () => {
    compile(fmSimple, { plugins: [gfmPlugin, fmPlugin, sanitizePlugin, syntectPlugin, tocPlugin] });
  });
  bench("unified", () => { unifiedCombined.processSync(fmSimple); });
});

describe("Plugin: Combined (readme)", () => {
  bench("unifast", () => {
    compile(fmReadme, { plugins: [gfmPlugin, fmPlugin, sanitizePlugin, syntectPlugin, tocPlugin] });
  });
  bench("unified", () => { unifiedCombined.processSync(fmReadme); });
});

describe("Plugin: Combined (large)", () => {
  bench("unifast", () => {
    compile(fmLarge, { plugins: [gfmPlugin, fmPlugin, sanitizePlugin, syntectPlugin, tocPlugin] });
  });
  bench("unified", () => { unifiedCombined.processSync(fmLarge); });
});
