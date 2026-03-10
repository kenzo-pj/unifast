import { describe, it, expect } from "vitest";

import {
  gfm,
  frontmatter,
  sanitize,
  syntect,
  treeSitter,
  toc,
  externalLinks,
  autolinkHeadings,
  smartypants,
  wikiLink,
  codeImport,
  emoji,
  breaks,
  math,
  githubAlert,
  sectionize,
  directive,
  definitionList,
  rubyAnnotation,
  cjk,
} from "../plugins";

describe(gfm, () => {
  it("returns defaults when called without options", () => {
    const plugin = gfm();
    expect(plugin.name).toBe("gfm");
    expect(plugin.options).toStrictEqual({
      gfm: { tables: true, taskList: true, strikethrough: true, footnotes: true, autolink: true },
    });
  });

  it("respects partial overrides", () => {
    const plugin = gfm({ tables: false, footnotes: false });
    expect(plugin.options!.gfm).toStrictEqual({
      tables: false,
      taskList: true,
      strikethrough: true,
      footnotes: false,
      autolink: true,
    });
  });
});

describe(frontmatter, () => {
  it("returns defaults when called without options", () => {
    const plugin = frontmatter();
    expect(plugin.name).toBe("frontmatter");
    expect(plugin.options).toStrictEqual({
      frontmatter: { yaml: true, toml: false, json: false },
    });
  });

  it("enables toml", () => {
    const plugin = frontmatter({ toml: true });
    expect(plugin.options!.frontmatter).toStrictEqual({ yaml: true, toml: true, json: false });
  });
});

describe(sanitize, () => {
  it("returns defaults", () => {
    const plugin = sanitize();
    expect(plugin.name).toBe("sanitize");
    expect(plugin.options!.sanitize).toStrictEqual({ enabled: true, schema: undefined });
  });

  it("accepts custom schema", () => {
    const schema = { allowedTags: ["p", "a"] };
    const plugin = sanitize({ schema });
    expect(plugin.options!.sanitize!.schema).toBe(schema);
  });
});

describe("highlight plugins", () => {
  it("syntect defaults to syntect engine", () => {
    const plugin = syntect();
    expect(plugin.name).toBe("syntect");
    expect(plugin.options!.highlight).toStrictEqual({ enabled: true, engine: "syntect" });
  });

  it("treeSitter defaults to treeSitter engine", () => {
    const plugin = treeSitter();
    expect(plugin.name).toBe("tree-sitter");
    expect(plugin.options!.highlight).toStrictEqual({ enabled: true, engine: "treeSitter" });
  });
});

describe(toc, () => {
  it("defaults maxDepth to 3", () => {
    const plugin = toc();
    expect(plugin.options!.toc).toStrictEqual({ enabled: true, maxDepth: 3 });
  });

  it("overrides maxDepth", () => {
    const plugin = toc({ maxDepth: 6 });
    expect(plugin.options!.toc!.maxDepth).toBe(6);
  });
});

describe(externalLinks, () => {
  it("returns defaults", () => {
    const plugin = externalLinks();
    expect(plugin.options!.externalLinks).toStrictEqual({
      enabled: true,
      rel: "noopener noreferrer",
      target: undefined,
    });
  });

  it("overrides rel and target", () => {
    const plugin = externalLinks({ rel: "noopener", target: "_blank" });
    expect(plugin.options!.externalLinks).toStrictEqual({
      enabled: true,
      rel: "noopener",
      target: "_blank",
    });
  });
});

describe(autolinkHeadings, () => {
  it("defaults to prepend", () => {
    const plugin = autolinkHeadings();
    expect(plugin.options!.autolinkHeadings).toStrictEqual({ enabled: true, behavior: "prepend" });
  });

  it("accepts wrap behavior", () => {
    expect(autolinkHeadings({ behavior: "wrap" }).options!.autolinkHeadings!.behavior).toBe("wrap");
  });
});

describe(smartypants, () => {
  it("returns defaults", () => {
    const plugin = smartypants();
    expect(plugin.options!.smartypants).toStrictEqual({
      enabled: true,
      quotes: true,
      dashes: true,
      ellipses: true,
    });
  });

  it("disables quotes", () => {
    expect(smartypants({ quotes: false }).options!.smartypants!.quotes).toBeFalsy();
  });
});

describe(wikiLink, () => {
  it("defaults hrefTemplate", () => {
    const plugin = wikiLink();
    expect(plugin.options!.wikiLink).toStrictEqual({
      enabled: true,
      hrefTemplate: "/wiki/{slug}",
    });
  });

  it("overrides hrefTemplate", () => {
    const plugin = wikiLink({ hrefTemplate: "/pages/${slug}" });
    expect(plugin.options!.wikiLink!.hrefTemplate).toBe("/pages/${slug}");
  });
});

describe(codeImport, () => {
  it("defaults rootDir to undefined", () => {
    const plugin = codeImport();
    expect(plugin.options!.codeImport).toStrictEqual({ enabled: true, rootDir: undefined });
  });

  it("accepts rootDir", () => {
    const plugin = codeImport({ rootDir: "/src" });
    expect(plugin.options!.codeImport!.rootDir).toBe("/src");
  });
});

describe("simple toggle plugins", () => {
  it.each([
    ["emoji", emoji],
    ["breaks", breaks],
    ["math", math],
    ["github-alert", githubAlert],
    ["sectionize", sectionize],
    ["directive", directive],
    ["definition-list", definitionList],
    ["ruby-annotation", rubyAnnotation],
    ["cjk", cjk],
  ] as const)("%s returns enabled:true", (expectedName, factory) => {
    const plugin = factory();
    expect(plugin.name).toBe(expectedName);
    const optionKey = Object.keys(plugin.options!)[0];
    expect(
      (plugin.options as Record<string, Record<string, unknown>>)[optionKey].enabled,
    ).toBeTruthy();
  });
});
