import type { Locale } from "~/i18n/locales/en";

export interface NavItem {
  labelKey: keyof Locale["nav"];
  href: string;
  slug?: string;
}

export interface NavGroup {
  labelKey: keyof Locale["nav"];
  items: NavItem[];
}

export interface NavSection {
  labelKey: keyof Locale["nav"];
  items?: NavItem[];
  groups?: NavGroup[];
}

export const NAV: NavSection[] = [
  {
    labelKey: "introduction",
    items: [
      {
        labelKey: "whatIsUnifast",
        href: "/docs/introduction/what-is-unifast/",
        slug: "introduction/what-is-unifast",
      },
      {
        labelKey: "quickStart",
        href: "/docs/introduction/quick-start/",
        slug: "introduction/quick-start",
      },
      {
        labelKey: "keyConcepts",
        href: "/docs/introduction/key-concepts/",
        slug: "introduction/key-concepts",
      },
    ],
  },
  {
    labelKey: "guides",
    items: [
      {
        labelKey: "syntaxHighlighting",
        href: "/docs/guides/syntax-highlighting/",
        slug: "guides/syntax-highlighting",
      },
      { labelKey: "usingMdx", href: "/docs/guides/mdx/", slug: "guides/mdx" },
      { labelKey: "viteIntegration", href: "/docs/guides/vite/", slug: "guides/vite" },
      { labelKey: "reactIntegration", href: "/docs/guides/react/", slug: "guides/react" },
    ],
  },
  {
    labelKey: "packages",
    groups: [
      {
        labelKey: "core",
        items: [
          {
            labelKey: "escapeHtmlFn",
            href: "/docs/packages/core/escape-html/",
            slug: "packages/core/escape-html",
          },
          {
            labelKey: "extractLangFn",
            href: "/docs/packages/core/extract-lang/",
            slug: "packages/core/extract-lang",
          },
          {
            labelKey: "extractTextFn",
            href: "/docs/packages/core/extract-text/",
            slug: "packages/core/extract-text",
          },
          {
            labelKey: "findCodeChildFn",
            href: "/docs/packages/core/find-code-child/",
            slug: "packages/core/find-code-child",
          },
          {
            labelKey: "hastToHtml",
            href: "/docs/packages/core/hast-to-html/",
            slug: "packages/core/hast-to-html",
          },
          {
            labelKey: "visitHastFn",
            href: "/docs/packages/core/visit-hast/",
            slug: "packages/core/visit-hast",
          },
        ],
      },
      {
        labelKey: "node",
        items: [
          {
            labelKey: "autolinkHeadingsFn",
            href: "/docs/packages/node/autolink-headings/",
            slug: "packages/node/autolink-headings",
          },
          {
            labelKey: "breaksFn",
            href: "/docs/packages/node/breaks/",
            slug: "packages/node/breaks",
          },
          { labelKey: "cjkFn", href: "/docs/packages/node/cjk/", slug: "packages/node/cjk" },
          {
            labelKey: "codeImportFn",
            href: "/docs/packages/node/code-import/",
            slug: "packages/node/code-import",
          },
          {
            labelKey: "compileFn",
            href: "/docs/packages/node/compile/",
            slug: "packages/node/compile",
          },
          {
            labelKey: "definitionListFn",
            href: "/docs/packages/node/definition-list/",
            slug: "packages/node/definition-list",
          },
          {
            labelKey: "directiveFn",
            href: "/docs/packages/node/directive/",
            slug: "packages/node/directive",
          },
          { labelKey: "emojiFn", href: "/docs/packages/node/emoji/", slug: "packages/node/emoji" },
          {
            labelKey: "externalLinksFn",
            href: "/docs/packages/node/external-links/",
            slug: "packages/node/external-links",
          },
          {
            labelKey: "frontmatterFn",
            href: "/docs/packages/node/frontmatter/",
            slug: "packages/node/frontmatter",
          },
          { labelKey: "gfmFn", href: "/docs/packages/node/gfm/", slug: "packages/node/gfm" },
          {
            labelKey: "githubAlertFn",
            href: "/docs/packages/node/github-alert/",
            slug: "packages/node/github-alert",
          },
          { labelKey: "mathFn", href: "/docs/packages/node/math/", slug: "packages/node/math" },
          {
            labelKey: "rubyAnnotationFn",
            href: "/docs/packages/node/ruby-annotation/",
            slug: "packages/node/ruby-annotation",
          },
          {
            labelKey: "sanitizeFn",
            href: "/docs/packages/node/sanitize/",
            slug: "packages/node/sanitize",
          },
          {
            labelKey: "sectionizeFn",
            href: "/docs/packages/node/sectionize/",
            slug: "packages/node/sectionize",
          },
          {
            labelKey: "smartypantsFn",
            href: "/docs/packages/node/smartypants/",
            slug: "packages/node/smartypants",
          },
          {
            labelKey: "syntectFn",
            href: "/docs/packages/node/syntect/",
            slug: "packages/node/syntect",
          },
          { labelKey: "tocFn", href: "/docs/packages/node/toc/", slug: "packages/node/toc" },
          {
            labelKey: "treeSitterFn",
            href: "/docs/packages/node/tree-sitter/",
            slug: "packages/node/tree-sitter",
          },
          {
            labelKey: "wikiLinkFn",
            href: "/docs/packages/node/wiki-link/",
            slug: "packages/node/wiki-link",
          },
          {
            labelKey: "abbrFn",
            href: "/docs/packages/node/abbr/",
            slug: "packages/node/abbr",
          },
          {
            labelKey: "accessibleEmojiFn",
            href: "/docs/packages/node/accessible-emoji/",
            slug: "packages/node/accessible-emoji",
          },
          {
            labelKey: "addClassesFn",
            href: "/docs/packages/node/add-classes/",
            slug: "packages/node/add-classes",
          },
          {
            labelKey: "codeMetaFn",
            href: "/docs/packages/node/code-meta/",
            slug: "packages/node/code-meta",
          },
          {
            labelKey: "commentRemovalFn",
            href: "/docs/packages/node/comment-removal/",
            slug: "packages/node/comment-removal",
          },
          {
            labelKey: "customHeadingIdFn",
            href: "/docs/packages/node/custom-heading-id/",
            slug: "packages/node/custom-heading-id",
          },
          {
            labelKey: "excerptFn",
            href: "/docs/packages/node/excerpt/",
            slug: "packages/node/excerpt",
          },
          {
            labelKey: "figureFn",
            href: "/docs/packages/node/figure/",
            slug: "packages/node/figure",
          },
          {
            labelKey: "imgLazyLoadingFn",
            href: "/docs/packages/node/img-lazy-loading/",
            slug: "packages/node/img-lazy-loading",
          },
          {
            labelKey: "minifyFn",
            href: "/docs/packages/node/minify/",
            slug: "packages/node/minify",
          },
          {
            labelKey: "readingTimeFn",
            href: "/docs/packages/node/reading-time/",
            slug: "packages/node/reading-time",
          },
        ],
      },
      {
        labelKey: "shiki",
        items: [
          {
            labelKey: "createShikiPluginFn",
            href: "/docs/packages/shiki/create-shiki-plugin/",
            slug: "packages/shiki/create-shiki-plugin",
          },
          {
            labelKey: "createShikiTransformerFn",
            href: "/docs/packages/shiki/create-shiki-transformer/",
            slug: "packages/shiki/create-shiki-transformer",
          },
        ],
      },
      {
        labelKey: "highlight",
        items: [
          {
            labelKey: "highlightFn",
            href: "/docs/packages/highlight/highlight/",
            slug: "packages/highlight/highlight",
          },
        ],
      },
      {
        labelKey: "vite",
        items: [
          {
            labelKey: "unifastPluginFn",
            href: "/docs/packages/vite/unifast-plugin/",
            slug: "packages/vite/unifast-plugin",
          },
        ],
      },
      {
        labelKey: "react",
        items: [
          {
            labelKey: "compileToReactFn",
            href: "/docs/packages/react/compile-to-react/",
            slug: "packages/react/compile-to-react",
          },
          {
            labelKey: "hastToReactFn",
            href: "/docs/packages/react/hast-to-react/",
            slug: "packages/react/hast-to-react",
          },
        ],
      },
    ],
  },
];

export function flattenNav(): NavItem[] {
  return NAV.flatMap((section) => {
    const items: NavItem[] = [];
    if (section.items) items.push(...section.items);
    if (section.groups) {
      for (const group of section.groups) {
        items.push(...group.items);
      }
    }
    return items;
  });
}
