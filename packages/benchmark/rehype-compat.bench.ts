import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import {
  createCommentRemovalProcessor,
  createAddClassesProcessor,
  createMinifyProcessor,
  createReadingTimeProcessor,
  createAccessibleEmojiProcessor,
  createImgLazyLoadingProcessor,
  createFigureProcessor,
  createBasicProcessor,
} from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");
const allFeatures = readFileSync(join(fixturesDir, "all-features.md"), "utf-8");

const unifiedCommentRemoval = createCommentRemovalProcessor();
const unifiedAddClasses = createAddClassesProcessor();
const unifiedMinify = createMinifyProcessor();
const unifiedReadingTime = createReadingTimeProcessor();
const unifiedAccessibleEmoji = createAccessibleEmojiProcessor();
const unifiedImgLazy = createImgLazyLoadingProcessor();
const unifiedFigure = createFigureProcessor();
const baseline = createBasicProcessor();

const commentRemovalOpts = { commentRemoval: { enabled: true } };
const addClassOpts = { addClass: { enabled: true, rules: { "h1": "title", "p": "text", "a": "link" } } };
const minifyOpts = { minify: { enabled: true } };
const readingTimeOpts = { readingTime: { enabled: true } };
const accessibleEmojiOpts = { accessibleEmoji: { enabled: true } };
const imgLazyOpts = { imgLazyLoading: { enabled: true } };
const figureOpts = { figure: { enabled: true } };
const codeMetaOpts = { codeMeta: { enabled: true } };
const excerptOpts = { excerpt: { enabled: true } };
const customHeadingIdOpts = { customHeadingId: { enabled: true } };

describe("Comment Removal (all-features)", () => {
  bench("unifast", () => { compile(allFeatures, commentRemovalOpts); });
  bench("unified (rehype-remove-comments)", () => { unifiedCommentRemoval.processSync(allFeatures); });
});

describe("Add Classes (readme)", () => {
  bench("unifast", () => { compile(readme, addClassOpts); });
  bench("unified (rehype-class-names)", () => { unifiedAddClasses.processSync(readme); });
});

describe("Minify (readme)", () => {
  bench("unifast", () => { compile(readme, minifyOpts); });
  bench("unified (rehype-preset-minify)", () => { unifiedMinify.processSync(readme); });
});

describe("Reading Time (readme)", () => {
  bench("unifast", () => { compile(readme, readingTimeOpts); });
  bench("unified + reading-time", () => { unifiedReadingTime.processSync(readme); });
});

describe("Accessible Emoji (all-features)", () => {
  bench("unifast", () => { compile(allFeatures, accessibleEmojiOpts); });
  bench("unified (rehype-accessible-emojis)", () => { unifiedAccessibleEmoji.processSync(allFeatures); });
});

describe("Image Lazy Loading (readme)", () => {
  bench("unifast", () => { compile(readme, imgLazyOpts); });
  bench("unified (rehype-image-native-lazy-loading)", () => { unifiedImgLazy.processSync(readme); });
});

describe("Figure (readme)", () => {
  bench("unifast", () => { compile(readme, figureOpts); });
  bench("unified (rehype-figure)", () => { unifiedFigure.processSync(readme); });
});

describe("Code Meta (all-features)", () => {
  bench("unifast", () => { compile(allFeatures, codeMetaOpts); });
  bench("unified (baseline)", () => { baseline.processSync(allFeatures); });
});

describe("Excerpt (all-features)", () => {
  bench("unifast", () => { compile(allFeatures, excerptOpts); });
  bench("unified (baseline)", () => { baseline.processSync(allFeatures); });
});

describe("Custom Heading ID (all-features)", () => {
  bench("unifast", () => { compile(allFeatures, customHeadingIdOpts); });
  bench("unified (baseline)", () => { baseline.processSync(allFeatures); });
});
