import { readFileSync } from "node:fs";
import { join } from "node:path";
import { group, bench, run } from "mitata";
import { compile } from "@unifast/node";
import {
  createBasicProcessor,
  createGfmProcessor,
  createAutolinkHeadingsProcessor,
  createBreaksProcessor,
  createWikiLinkProcessor,
  createMathProcessor,
  createSanitizeProcessor,
} from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const simple = readFileSync(join(fixturesDir, "simple.md"), "utf-8");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const large = readFileSync(join(fixturesDir, "large.md"), "utf-8");
const allFeatures = readFileSync(join(fixturesDir, "all-features.md"), "utf-8");

const basicProcessor = createBasicProcessor();
const gfmProcessor = createGfmProcessor();
const autolinkHeadingsProcessor = createAutolinkHeadingsProcessor();
const breaksProcessor = createBreaksProcessor();
const wikiLinkProcessor = createWikiLinkProcessor();
const mathProcessor = createMathProcessor();
const sanitizeProcessor = createSanitizeProcessor();

// --- Basic Markdown to HTML ---

group("Markdown to HTML (simple)", () => {
  bench("unifast", () => compile(simple));
  bench("unified", () => basicProcessor.processSync(simple));
  bench("Bun.markdown", () => Bun.markdown.html(simple));
});

group("Markdown to HTML (readme)", () => {
  bench("unifast", () => compile(readme));
  bench("unified", () => basicProcessor.processSync(readme));
  bench("Bun.markdown", () => Bun.markdown.html(readme));
});

group("Markdown to HTML (large)", () => {
  bench("unifast", () => compile(large));
  bench("unified", () => basicProcessor.processSync(large));
  bench("Bun.markdown", () => Bun.markdown.html(large));
});

// --- GFM ---

const unifastGfmOptions = {
  gfm: {
    tables: true,
    taskList: true,
    strikethrough: true,
    autolink: true,
  },
};

const bunGfmOptions = {
  tables: true,
  strikethrough: true,
  tasklists: true,
  autolinks: true,
};

group("GFM (simple)", () => {
  bench("unifast", () => compile(simple, unifastGfmOptions));
  bench("unified", () => gfmProcessor.processSync(simple));
  bench("Bun.markdown", () => Bun.markdown.html(simple, bunGfmOptions));
});

group("GFM (readme)", () => {
  bench("unifast", () => compile(readme, unifastGfmOptions));
  bench("unified", () => gfmProcessor.processSync(readme));
  bench("Bun.markdown", () => Bun.markdown.html(readme, bunGfmOptions));
});

group("GFM (large)", () => {
  bench("unifast", () => compile(large, unifastGfmOptions));
  bench("unified", () => gfmProcessor.processSync(large));
  bench("Bun.markdown", () => Bun.markdown.html(large, bunGfmOptions));
});

group("GFM (all-features)", () => {
  bench("unifast", () => compile(allFeatures, unifastGfmOptions));
  bench("unified", () => gfmProcessor.processSync(allFeatures));
  bench("Bun.markdown", () => Bun.markdown.html(allFeatures, bunGfmOptions));
});

// --- Heading IDs + Autolinks ---

const unifastHeadingsOptions = {
  slug: { mode: "github" as const },
  autolinkHeadings: { enabled: true, behavior: "prepend" as const },
};

group("Heading IDs + Autolinks (readme)", () => {
  bench("unifast", () => compile(readme, unifastHeadingsOptions));
  bench("unified", () => autolinkHeadingsProcessor.processSync(readme));
  bench("Bun.markdown", () => Bun.markdown.html(readme, { headings: true }));
});

group("Heading IDs + Autolinks (large)", () => {
  bench("unifast", () => compile(large, unifastHeadingsOptions));
  bench("unified", () => autolinkHeadingsProcessor.processSync(large));
  bench("Bun.markdown", () => Bun.markdown.html(large, { headings: true }));
});

// --- Hard/Soft Breaks ---

const unifastBreaksOptions = { breaks: { enabled: true } };

group("Breaks (readme)", () => {
  bench("unifast", () => compile(readme, unifastBreaksOptions));
  bench("unified", () => breaksProcessor.processSync(readme));
  bench("Bun.markdown", () => Bun.markdown.html(readme, { hardSoftBreaks: true }));
});

group("Breaks (all-features)", () => {
  bench("unifast", () => compile(allFeatures, unifastBreaksOptions));
  bench("unified", () => breaksProcessor.processSync(allFeatures));
  bench("Bun.markdown", () => Bun.markdown.html(allFeatures, { hardSoftBreaks: true }));
});

// --- Wiki Links ---

const unifastWikiLinkOptions = { wikiLink: { enabled: true } };

group("Wiki Links (all-features)", () => {
  bench("unifast", () => compile(allFeatures, unifastWikiLinkOptions));
  bench("unified", () => wikiLinkProcessor.processSync(allFeatures));
  bench("Bun.markdown", () => Bun.markdown.html(allFeatures, { wikiLinks: true }));
});

// --- Math (LaTeX) ---

const unifastMathOptions = { math: { enabled: true } };

group("Math (all-features)", () => {
  bench("unifast", () => compile(allFeatures, unifastMathOptions));
  bench("unified", () => mathProcessor.processSync(allFeatures));
  bench("Bun.markdown", () => Bun.markdown.html(allFeatures, { latexMath: true }));
});

// --- Tag Filter / Sanitize ---
// Note: Bun's tagFilter only filters specific GFM-disallowed HTML tags,
// while unifast's sanitize performs full HTML sanitization.

const unifastSanitizeOptions = { sanitize: { enabled: true } };

group("Tag Filter / Sanitize (readme)", () => {
  bench("unifast", () => compile(readme, unifastSanitizeOptions));
  bench("unified", () => sanitizeProcessor.processSync(readme));
  bench("Bun.markdown", () => Bun.markdown.html(readme, { tagFilter: true }));
});

group("Tag Filter / Sanitize (all-features)", () => {
  bench("unifast", () => compile(allFeatures, unifastSanitizeOptions));
  bench("unified", () => sanitizeProcessor.processSync(allFeatures));
  bench("Bun.markdown", () => Bun.markdown.html(allFeatures, { tagFilter: true }));
});

// --- All shared features combined ---

const unifastAllSharedOptions = {
  gfm: {
    tables: true,
    taskList: true,
    strikethrough: true,
    autolink: true,
  },
  slug: { mode: "github" as const },
  autolinkHeadings: { enabled: true, behavior: "prepend" as const },
  breaks: { enabled: true },
  wikiLink: { enabled: true },
  math: { enabled: true },
  sanitize: { enabled: true },
};

const bunAllSharedOptions = {
  tables: true,
  strikethrough: true,
  tasklists: true,
  autolinks: true,
  headings: true,
  hardSoftBreaks: true,
  wikiLinks: true,
  latexMath: true,
  tagFilter: true,
};

group("All shared features (readme)", () => {
  bench("unifast", () => compile(readme, unifastAllSharedOptions));
  bench("Bun.markdown", () => Bun.markdown.html(readme, bunAllSharedOptions));
});

group("All shared features (large)", () => {
  bench("unifast", () => compile(large, unifastAllSharedOptions));
  bench("Bun.markdown", () => Bun.markdown.html(large, bunAllSharedOptions));
});

group("All shared features (all-features)", () => {
  bench("unifast", () => compile(allFeatures, unifastAllSharedOptions));
  bench("Bun.markdown", () => Bun.markdown.html(allFeatures, bunAllSharedOptions));
});

await run();
