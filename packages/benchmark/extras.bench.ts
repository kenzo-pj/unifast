import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import {
  createExternalLinksProcessor,
  createAutolinkHeadingsProcessor,
  createSmartypantsProcessor,
  createEmojiProcessor,
  createBreaksProcessor,
  createMathProcessor,
  createGithubAlertProcessor,
  createDirectiveProcessor,
  createWikiLinkProcessor,
} from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");

const unifiedExtLinks = createExternalLinksProcessor();
const unifiedAutolink = createAutolinkHeadingsProcessor();
const unifiedSmartypants = createSmartypantsProcessor();
const unifiedEmoji = createEmojiProcessor();
const unifiedBreaks = createBreaksProcessor();
const unifiedMath = createMathProcessor();
const unifiedGithubAlert = createGithubAlertProcessor();
const unifiedDirective = createDirectiveProcessor();
const unifiedWikiLink = createWikiLinkProcessor();

const extLinksOpts = { externalLinks: { enabled: true, rel: "nofollow noopener noreferrer" } };
const autolinkOpts = { slug: { mode: "github" as const }, autolinkHeadings: { enabled: true, behavior: "prepend" as const } };
const smartypantsOpts = { smartypants: { enabled: true } };
const emojiOpts = { emoji: { enabled: true } };
const breaksOpts = { breaks: { enabled: true } };
const mathOpts = { math: { enabled: true } };
const githubAlertOpts = { githubAlert: { enabled: true } };
const directiveOpts = { directive: { enabled: true } };
const wikiLinkOpts = { wikiLink: { enabled: true } };

describe("External Links (readme)", () => {
  bench("unifast", () => { compile(readme, extLinksOpts); });
  bench("unified", () => { unifiedExtLinks.processSync(readme); });
});

describe("Autolink Headings (readme)", () => {
  bench("unifast", () => { compile(readme, autolinkOpts); });
  bench("unified", () => { unifiedAutolink.processSync(readme); });
});

describe("Smartypants (readme)", () => {
  bench("unifast", () => { compile(readme, smartypantsOpts); });
  bench("unified", () => { unifiedSmartypants.processSync(readme); });
});

describe("Emoji (readme)", () => {
  bench("unifast", () => { compile(readme, emojiOpts); });
  bench("unified", () => { unifiedEmoji.processSync(readme); });
});

describe("Breaks (readme)", () => {
  bench("unifast", () => { compile(readme, breaksOpts); });
  bench("unified", () => { unifiedBreaks.processSync(readme); });
});

describe("Math (readme)", () => {
  bench("unifast", () => { compile(readme, mathOpts); });
  bench("unified", () => { unifiedMath.processSync(readme); });
});

describe("GitHub Alerts (readme)", () => {
  bench("unifast", () => { compile(readme, githubAlertOpts); });
  bench("unified", () => { unifiedGithubAlert.processSync(readme); });
});

describe("Directive (readme)", () => {
  bench("unifast", () => { compile(readme, directiveOpts); });
  bench("unified", () => { unifiedDirective.processSync(readme); });
});

describe("Wiki Link (readme)", () => {
  bench("unifast", () => { compile(readme, wikiLinkOpts); });
  bench("unified", () => { unifiedWikiLink.processSync(readme); });
});
