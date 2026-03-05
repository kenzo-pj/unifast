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
        href: "/docs/introduction/what-is-unifast",
        slug: "introduction/what-is-unifast",
      },
      {
        labelKey: "quickStart",
        href: "/docs/introduction/quick-start",
        slug: "introduction/quick-start",
      },
      {
        labelKey: "keyConcepts",
        href: "/docs/introduction/key-concepts",
        slug: "introduction/key-concepts",
      },
    ],
  },
  {
    labelKey: "guides",
    items: [
      {
        labelKey: "syntaxHighlighting",
        href: "/docs/guides/syntax-highlighting",
        slug: "guides/syntax-highlighting",
      },
      { labelKey: "usingMdx", href: "/docs/guides/mdx", slug: "guides/mdx" },
      { labelKey: "viteIntegration", href: "/docs/guides/vite", slug: "guides/vite" },
      { labelKey: "reactIntegration", href: "/docs/guides/react", slug: "guides/react" },
    ],
  },
  {
    labelKey: "packages",
    groups: [
      {
        labelKey: "core",
        items: [
          {
            labelKey: "hastToHtml",
            href: "/docs/packages/core/hast-to-html",
            slug: "packages/core/hast-to-html",
          },
          { labelKey: "gfmFn", href: "/docs/packages/core/gfm", slug: "packages/core/gfm" },
          {
            labelKey: "frontmatterFn",
            href: "/docs/packages/core/frontmatter",
            slug: "packages/core/frontmatter",
          },
          {
            labelKey: "sanitizeFn",
            href: "/docs/packages/core/sanitize",
            slug: "packages/core/sanitize",
          },
          {
            labelKey: "syntectFn",
            href: "/docs/packages/core/syntect",
            slug: "packages/core/syntect",
          },
          { labelKey: "tocFn", href: "/docs/packages/core/toc", slug: "packages/core/toc" },
          {
            labelKey: "externalLinksFn",
            href: "/docs/packages/core/external-links",
            slug: "packages/core/external-links",
          },
          {
            labelKey: "autolinkHeadingsFn",
            href: "/docs/packages/core/autolink-headings",
            slug: "packages/core/autolink-headings",
          },
          {
            labelKey: "sectionizeFn",
            href: "/docs/packages/core/sectionize",
            slug: "packages/core/sectionize",
          },
          {
            labelKey: "breaksFn",
            href: "/docs/packages/core/breaks",
            slug: "packages/core/breaks",
          },
          {
            labelKey: "smartypantsFn",
            href: "/docs/packages/core/smartypants",
            slug: "packages/core/smartypants",
          },
          { labelKey: "emojiFn", href: "/docs/packages/core/emoji", slug: "packages/core/emoji" },
          {
            labelKey: "githubAlertFn",
            href: "/docs/packages/core/github-alert",
            slug: "packages/core/github-alert",
          },
          { labelKey: "mathFn", href: "/docs/packages/core/math", slug: "packages/core/math" },
          {
            labelKey: "directiveFn",
            href: "/docs/packages/core/directive",
            slug: "packages/core/directive",
          },
          {
            labelKey: "wikiLinkFn",
            href: "/docs/packages/core/wiki-link",
            slug: "packages/core/wiki-link",
          },
          {
            labelKey: "definitionListFn",
            href: "/docs/packages/core/definition-list",
            slug: "packages/core/definition-list",
          },
          {
            labelKey: "rubyAnnotationFn",
            href: "/docs/packages/core/ruby-annotation",
            slug: "packages/core/ruby-annotation",
          },
          { labelKey: "cjkFn", href: "/docs/packages/core/cjk", slug: "packages/core/cjk" },
          {
            labelKey: "codeImportFn",
            href: "/docs/packages/core/code-import",
            slug: "packages/core/code-import",
          },
        ],
      },
      {
        labelKey: "node",
        items: [
          {
            labelKey: "compileFn",
            href: "/docs/packages/node/compile",
            slug: "packages/node/compile",
          },
        ],
      },
      {
        labelKey: "shiki",
        items: [
          {
            labelKey: "createShikiPluginFn",
            href: "/docs/packages/shiki/create-shiki-plugin",
            slug: "packages/shiki/create-shiki-plugin",
          },
          {
            labelKey: "createShikiTransformerFn",
            href: "/docs/packages/shiki/create-shiki-transformer",
            slug: "packages/shiki/create-shiki-transformer",
          },
        ],
      },
      {
        labelKey: "react",
        items: [
          {
            labelKey: "compileToReactFn",
            href: "/docs/packages/react/compile-to-react",
            slug: "packages/react/compile-to-react",
          },
          {
            labelKey: "hastToReactFn",
            href: "/docs/packages/react/hast-to-react",
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
