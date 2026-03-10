import { describe, expect, expectTypeOf, it } from "vitest";

import {
  abbr,
  accessibleEmoji,
  addClasses,
  autolinkHeadings,
  breaks,
  cjk,
  codeMeta,
  commentRemoval,
  compile,
  customHeadingId,
  definitionList,
  directive,
  emoji,
  excerpt,
  externalLinks,
  frontmatter,
  gfm,
  githubAlert,
  imgLazyLoading,
  math,
  minify,
  readingTime,
  rubyAnnotation,
  sanitize,
  sectionize,
  smartypants,
  toc,
  wikiLink,
} from "../index.js";

describe("plugin e2e round-trip", () => {
  describe("gfm plugin", () => {
    it("renders tables", () => {
      const result = compile("| a | b |\n|---|---|\n| 1 | 2 |", {
        plugins: [gfm()],
      });
      expect(result.output).toBe(
        "<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody><tr><td>1</td><td>2</td></tr></tbody></table>",
      );
    });

    it("renders task lists", () => {
      const result = compile("- [x] done\n- [ ] todo", {
        plugins: [gfm()],
      });
      expect(result.output).toBe(
        '<ul><li><input checked disabled type="checkbox" /><p>done</p></li><li><input disabled type="checkbox" /><p>todo</p></li></ul>',
      );
    });

    it("renders strikethrough", () => {
      const result = compile("~~deleted~~", { plugins: [gfm()] });
      expect(result.output).toBe("<p><del>deleted</del></p>");
    });

    it("renders footnotes", () => {
      const result = compile("text[^1]\n\n[^1]: note", {
        plugins: [gfm()],
      });
      expect(result.output).toBe(
        '<p>text<sup class="footnote-ref"><a class="footnote-ref" href="#fn-1">1</a></sup></p>',
      );
    });

    it("autolinks URLs", () => {
      const result = compile("Visit https://example.com today", {
        plugins: [gfm()],
      });
      expect(result.output).toBe(
        '<p>Visit <a href="https://example.com">https://example.com</a> today</p>',
      );
    });
  });

  describe("frontmatter plugin", () => {
    it("extracts YAML frontmatter", () => {
      const md = "---\ntitle: Hello\nauthor: World\n---\n\n# Content";
      const result = compile(md, { plugins: [frontmatter()] });
      expect(result.frontmatter).toStrictEqual({ title: "Hello", author: "World" });
      expect(result.output).toBe('<h1 id="content">Content</h1>');
    });

    it("extracts TOML frontmatter", () => {
      const md = '+++\ntitle = "Hello"\n+++\n\n# Content';
      const result = compile(md, { plugins: [frontmatter({ toml: true })] });
      expect(result.frontmatter.title).toBe("Hello");
      expect(result.output).toBe('<h1 id="content">Content</h1>');
    });

    it("extracts JSON frontmatter with ;;; delimiters", () => {
      const md = ';;;\n{"title": "Hello", "draft": false}\n;;;\n\n# Content';
      const result = compile(md, { plugins: [frontmatter({ json: true })] });
      expect(result.frontmatter).toStrictEqual({ title: "Hello", draft: false });
      expect(result.output).toBe('<h1 id="content">Content</h1>');
    });
  });

  describe("toc plugin", () => {
    it("extracts table of contents", () => {
      const md = "# First\n\n## Second\n\n### Third\n\n#### Fourth";
      const result = compile(md, { plugins: [toc()] });
      expect(result.toc).toStrictEqual([
        { depth: 1, text: "First", slug: "first" },
        { depth: 2, text: "Second", slug: "second" },
        { depth: 3, text: "Third", slug: "third" },
      ]);
    });

    it("respects maxDepth", () => {
      const md = "# H1\n\n## H2\n\n### H3";
      const result = compile(md, { plugins: [toc({ maxDepth: 2 })] });
      expect(result.toc).toHaveLength(2);
      expect(result.toc[0].depth).toBe(1);
      expect(result.toc[1].depth).toBe(2);
    });
  });

  describe("readingTime plugin", () => {
    it("returns reading time for English text", () => {
      const words = Array.from({ length: 200 }, () => "word").join(" ");
      const result = compile(words, { plugins: [readingTime()] });
      expect(result.readingTime).toStrictEqual({ words: 200, minutes: 1 });
    });

    it("returns reading time for CJK text", () => {
      const cjkText = "今日は天気がとても良いです。公園で散歩をしました。";
      const result = compile(cjkText, { plugins: [readingTime()] });
      expect(result.readingTime).toBeDefined();
      expect(result.readingTime!.minutes).toBe(1);
      expect(result.readingTime!.words).toBeGreaterThan(0);
    });

    it("is undefined when plugin not used", () => {
      const result = compile("hello world");
      expect(result.readingTime).toBeUndefined();
    });
  });

  describe("excerpt plugin", () => {
    it("extracts excerpt with separator", () => {
      const md = "This is the intro.\n\n<!-- more -->\n\nThis is the rest.";
      const result = compile(md, { plugins: [excerpt()] });
      expect(result.excerpt).toBe("This is the intro.");
    });

    it("falls back to first paragraph", () => {
      const md = "First paragraph.\n\nSecond paragraph.";
      const result = compile(md, {
        plugins: [excerpt({ fallbackParagraphs: 1 })],
      });
      expect(result.excerpt).toBe("First paragraph.");
    });

    it("is undefined when plugin not used", () => {
      const result = compile("hello world");
      expect(result.excerpt).toBeUndefined();
    });
  });

  describe("sanitize plugin", () => {
    it("removes script tags and preserves safe HTML", () => {
      const result = compile("<script>alert('xss')</script>\n\n**bold**", {
        plugins: [sanitize()],
      });
      expect(result.output).toBe(
        "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;<p><strong>bold</strong></p>",
      );
    });
  });

  describe("externalLinks plugin", () => {
    it("adds rel and target to external links", () => {
      const result = compile("[link](https://example.com)", {
        plugins: [externalLinks({ target: "_blank" })],
      });
      expect(result.output).toBe(
        '<p><a href="https://example.com" rel="noopener noreferrer" target="_blank">link</a></p>',
      );
    });
  });

  describe("autolinkHeadings plugin", () => {
    it("adds anchor links to headings", () => {
      const result = compile("# Hello World", {
        plugins: [autolinkHeadings()],
      });
      expect(result.output).toBe(
        '<h1 id="hello-world"><a aria-hidden="true" class="anchor" href="#hello-world"></a>Hello World</h1>',
      );
    });
  });

  describe("smartypants plugin", () => {
    it("converts quotes to smart quotes", () => {
      const result = compile("\"hello\" and 'world'", {
        plugins: [smartypants()],
      });
      expect(result.output).toBe("<p>\u201Chello\u201D and \u2018world\u2019</p>");
    });

    it("converts dashes", () => {
      const result = compile("a -- b --- c", {
        plugins: [smartypants()],
      });
      expect(result.output).toBe("<p>a \u2013 b \u2014 c</p>");
    });

    it("converts ellipses", () => {
      const result = compile("wait...", { plugins: [smartypants()] });
      expect(result.output).toBe("<p>wait\u2026</p>");
    });
  });

  describe("emoji plugin", () => {
    it("converts shortcodes to emoji", () => {
      const result = compile(":smile:", { plugins: [emoji()] });
      expect(result.output).toBe("<p>\u{1F604}</p>");
    });
  });

  describe("breaks plugin", () => {
    it("converts newlines to <br>", () => {
      const result = compile("line1\nline2", { plugins: [breaks()] });
      expect(result.output).toBe("<p>line1<br />line2</p>");
    });
  });

  describe("math plugin", () => {
    it("renders inline math", () => {
      const result = compile("$E = mc^2$", { plugins: [math()] });
      expect(result.output).toBe('<p><code class="math math-inline">E = mc^2</code></p>');
    });

    it("renders display math with $$ in text", () => {
      const result = compile("$$\nx^2 + y^2 = z^2\n$$", {
        plugins: [math()],
      });
      expect(result.output).toBe(
        '<p><pre class="math math-display"><code>\nx^2 + y^2 = z^2\n</code></pre></p>',
      );
    });

    it("renders fenced math code block", () => {
      const result = compile("```math\nx^2\n```", { plugins: [math()] });
      expect(result.output).toBe('<pre class="math math-display"><code>x^2</code></pre>');
    });
  });

  describe("githubAlert plugin", () => {
    it("renders note alert with icon", () => {
      const result = compile("> [!NOTE]\n> Important info", {
        plugins: [githubAlert()],
      });
      expect(result.output).toContain('<div class="alert alert-note">');
      expect(result.output).toContain('<p class="alert-title">');
      expect(result.output).toContain("Note</p>");
      expect(result.output).toContain("<p>Important info</p>");
      expect(result.output).toContain('<svg aria-hidden="true"');
    });
  });

  describe("sectionize plugin", () => {
    it("wraps sections in <section>", () => {
      const result = compile("# First\n\npara\n\n# Second\n\npara2", {
        plugins: [sectionize()],
      });
      expect(result.output).toBe(
        '<section><h1 id="first">First</h1><p>para</p></section><section><h1 id="second">Second</h1><p>para2</p></section>',
      );
    });
  });

  describe("directive plugin", () => {
    it("parses container directive", () => {
      const result = compile(":::note\nContent\n:::", {
        plugins: [directive()],
      });
      expect(result.output).toBe(
        '<div class="directive directive-note" data-directive="note"><p>Content</p></div>',
      );
    });
  });

  describe("definitionList plugin", () => {
    it("renders definition lists", () => {
      const result = compile("Term\n: Definition", {
        plugins: [definitionList()],
      });
      expect(result.output).toBe("<dl><dt>Term</dt><dd>Definition</dd></dl>");
    });
  });

  describe("rubyAnnotation plugin", () => {
    it("renders ruby annotations", () => {
      const result = compile("{漢字|かんじ}", {
        plugins: [rubyAnnotation()],
      });
      expect(result.output).toBe("<p><ruby>漢字<rp>(</rp><rt>かんじ</rt><rp>)</rp></ruby></p>");
    });
  });

  describe("cjk plugin", () => {
    it("passes CJK text through", () => {
      const result = compile("日本語text日本語", { plugins: [cjk()] });
      expect(result.output).toBe("<p>日本語text日本語</p>");
    });
  });

  describe("wikiLink plugin", () => {
    it("converts wiki links to anchor tags", () => {
      const result = compile("[[Page Name]]", { plugins: [wikiLink()] });
      expect(result.output).toBe(
        '<p><a class="wiki-link" href="/wiki/page-name">Page Name</a></p>',
      );
    });
  });

  describe("abbr plugin", () => {
    it("wraps abbreviations in <abbr>", () => {
      const md = "*[HTML]: Hyper Text Markup Language\n\nThe HTML spec.";
      const result = compile(md, { plugins: [abbr()] });
      expect(result.output).toBe(
        '<p>The <abbr title="Hyper Text Markup Language">HTML</abbr> spec.</p>',
      );
    });
  });

  describe("accessibleEmoji plugin", () => {
    it("wraps emoji with aria attributes", () => {
      const result = compile("Hello 😄 world", {
        plugins: [accessibleEmoji()],
      });
      expect(result.output).toBe(
        '<p>Hello <span aria-label="grinning face with smiling eyes" role="img">😄</span> world</p>',
      );
    });
  });

  describe("addClasses plugin", () => {
    it("adds CSS classes to elements", () => {
      const result = compile("# Hello\n\nParagraph", {
        plugins: [addClasses({ h1: "text-xl font-bold", p: "text-base" })],
      });
      expect(result.output).toBe(
        '<h1 class="text-xl font-bold" id="hello">Hello</h1><p class="text-base">Paragraph</p>',
      );
    });
  });

  describe("codeMeta plugin", () => {
    it("extracts title from code meta", () => {
      const md = '```ts title="example.ts"\nconst x = 1;\n```';
      const result = compile(md, { plugins: [codeMeta()] });
      expect(result.output).toBe(
        '<pre data-lang="ts" data-title="example.ts"><code class="language-ts">const x = 1;</code></pre>',
      );
    });
  });

  describe("commentRemoval plugin", () => {
    it("removes HTML comments", () => {
      const result = compile("before <!-- comment --> after", {
        plugins: [commentRemoval()],
      });
      expect(result.output).toBe("<p>before  after</p>");
    });
  });

  describe("customHeadingId plugin", () => {
    it("sets custom heading id", () => {
      const result = compile("# Heading {#my-id}", {
        plugins: [customHeadingId()],
      });
      expect(result.output).toBe('<h1 id="my-id">Heading</h1>');
    });
  });

  describe("imgLazyLoading plugin", () => {
    it("adds lazy loading to images", () => {
      const result = compile("![alt](image.png)", {
        plugins: [imgLazyLoading()],
      });
      expect(result.output).toBe(
        '<p><img alt="alt" decoding="async" loading="lazy" src="image.png" /></p>',
      );
    });

    it("skips first N images", () => {
      const md = "![a](1.png)\n\n![b](2.png)";
      const result = compile(md, {
        plugins: [imgLazyLoading({ skipFirst: 1 })],
      });
      expect(result.output).toBe(
        '<p><img alt="a" src="1.png" /></p><p><img alt="b" decoding="async" loading="lazy" src="2.png" /></p>',
      );
    });
  });

  describe("minify plugin", () => {
    it("minifies HTML output", () => {
      const result = compile("# Hello\n\nParagraph", {
        plugins: [minify()],
      });
      expect(result.output).toBe("<h1 id=hello>Hello</h1><p>Paragraph");
    });
  });

  describe("hastTransform with mdast outputKind", () => {
    it("does not crash and skips hastTransform when outputKind is mdast", () => {
      let called = false;
      const result = compile("# Hello", {
        outputKind: "mdast",
        plugins: [
          {
            name: "test-hast-noop",
            hastTransform: (hast) => {
              called = true;
              return hast;
            },
          },
        ],
      });
      expect(called).toBe(false);
      expect(result.output).toBeDefined();
      expect(result.output.length).toBeGreaterThan(0);
    });
  });

  describe("result shape", () => {
    it("always includes required fields", () => {
      const result = compile("# Hello");
      expect(result).toHaveProperty("output");
      expect(result).toHaveProperty("frontmatter");
      expect(result).toHaveProperty("diagnostics");
      expect(result).toHaveProperty("stats");
      expect(result).toHaveProperty("toc");
      expectTypeOf(result.stats.parseMs).toBeNumber();
      expectTypeOf(result.stats.transformMs).toBeNumber();
      expectTypeOf(result.stats.emitMs).toBeNumber();
    });
  });
});
