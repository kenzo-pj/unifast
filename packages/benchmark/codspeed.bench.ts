/**
 * CodSpeed regression tracking benchmarks — unifast only.
 * Comparison benchmarks (vs unified) are in separate files for local use.
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile, syntect, treeSitter } from "@unifast/node";
import { highlight } from "@unifast/highlight";
import { createShikiPlugin } from "@unifast/shiki";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");
const allFeatures = readFileSync(join(fixturesDir, "all-features.md"), "utf-8");

const frontmatterBlock = `---
title: Benchmark Document
author: unifast
date: 2026-03-04
tags: [benchmark, markdown, performance]
description: A test document for benchmarking.
---

`;
const simpleWithFm = frontmatterBlock + simple;
const readmeWithFm = frontmatterBlock + readme;
const largeWithFm = frontmatterBlock + large;

// Plugins
const highlightPlugin = highlight();
const syntectPlugin = syntect();
const treeSitterPlugin = treeSitter();
const shikiPlugin = await createShikiPlugin({
  themes: "github-dark",
  langs: ["javascript", "typescript", "rust", "bash", "json", "html", "css", "python", "yaml", "toml"],
});

// --- Basic Compile ---

describe("Compile", () => {
  bench("simple", () => { compile(simple); });
  bench("readme", () => { compile(readme); });
  bench("large", () => { compile(large); });
});

// --- GFM ---

const gfmOpts = { gfm: { tables: true, taskList: true, strikethrough: true, autolink: true } };
const gfmFootnotesOpts = { gfm: { tables: true, taskList: true, strikethrough: true, autolink: true, footnotes: true } };

describe("GFM", () => {
  bench("simple", () => { compile(simple, gfmOpts); });
  bench("readme", () => { compile(readme, gfmOpts); });
  bench("large", () => { compile(large, gfmOpts); });
  bench("all-features + footnotes", () => { compile(allFeatures, gfmFootnotesOpts); });
});

// --- Frontmatter ---

describe("Frontmatter", () => {
  const opts = { frontmatter: { yaml: true } };
  bench("simple", () => { compile(simpleWithFm, opts); });
  bench("readme", () => { compile(readmeWithFm, opts); });
  bench("large", () => { compile(largeWithFm, opts); });
});

// --- Sanitize ---

describe("Sanitize", () => {
  const opts = { sanitize: { enabled: true } };
  bench("simple", () => { compile(simple, opts); });
  bench("readme", () => { compile(readme, opts); });
  bench("large", () => { compile(large, opts); });
});

// --- TOC + Slug ---

describe("TOC + Slug", () => {
  const opts = { toc: { enabled: true }, slug: { mode: "github" as const } };
  bench("simple", () => { compile(simple, opts); });
  bench("readme", () => { compile(readme, opts); });
  bench("large", () => { compile(large, opts); });
});

// --- Extras ---

describe("Extras", () => {
  bench("external-links", () => { compile(readme, { externalLinks: { enabled: true, rel: "nofollow noopener noreferrer" } }); });
  bench("autolink-headings", () => { compile(readme, { slug: { mode: "github" as const }, autolinkHeadings: { enabled: true, behavior: "prepend" as const } }); });
  bench("smartypants", () => { compile(readme, { smartypants: { enabled: true } }); });
  bench("emoji", () => { compile(allFeatures, { emoji: { enabled: true } }); });
  bench("breaks", () => { compile(readme, { breaks: { enabled: true } }); });
  bench("math", () => { compile(allFeatures, { math: { enabled: true } }); });
  bench("github-alerts", () => { compile(allFeatures, { githubAlert: { enabled: true } }); });
  bench("directive", () => { compile(allFeatures, { directive: { enabled: true } }); });
  bench("wiki-link", () => { compile(allFeatures, { wikiLink: { enabled: true } }); });
});

// --- Rehype-compatible features ---

describe("Rehype Features", () => {
  bench("comment-removal", () => { compile(allFeatures, { commentRemoval: { enabled: true } }); });
  bench("add-classes", () => { compile(readme, { addClass: { enabled: true, rules: { "h1": "title", "p": "text", "a": "link" } } }); });
  bench("minify", () => { compile(readme, { minify: { enabled: true } }); });
  bench("reading-time", () => { compile(readme, { readingTime: { enabled: true } }); });
  bench("accessible-emoji", () => { compile(allFeatures, { accessibleEmoji: { enabled: true } }); });
  bench("img-lazy-loading", () => { compile(readme, { imgLazyLoading: { enabled: true } }); });
  bench("figure", () => { compile(readme, { figure: { enabled: true } }); });
  bench("code-meta", () => { compile(allFeatures, { codeMeta: { enabled: true } }); });
  bench("excerpt", () => { compile(allFeatures, { excerpt: { enabled: true } }); });
  bench("custom-heading-id", () => { compile(allFeatures, { customHeadingId: { enabled: true } }); });
});

// --- Unifast-only features ---

describe("Unifast-only", () => {
  bench("sectionize", () => { compile(readme, { sectionize: { enabled: true } }); });
  bench("definition-list", () => { compile(allFeatures, { definitionList: { enabled: true } }); });
  bench("ruby-annotation", () => { compile(allFeatures, { rubyAnnotation: { enabled: true } }); });
  bench("cjk", () => { compile(allFeatures, { cjk: { enabled: true } }); });
});

// --- Syntax Highlighting ---

describe("Highlight", () => {
  bench("highlight.js", () => { compile(readme, { plugins: [highlightPlugin] }); });
  bench("shiki", () => { compile(readme, { plugins: [shikiPlugin] }); });
  bench("syntect", () => { compile(readme, { plugins: [syntectPlugin] }); });
  bench("tree-sitter", () => { compile(readme, { plugins: [treeSitterPlugin] }); });
});
