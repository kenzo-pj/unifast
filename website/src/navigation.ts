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
          { labelKey: "gfmFn", href: "/docs/packages/node/gfm", slug: "packages/node/gfm" },
          {
            labelKey: "frontmatterFn",
            href: "/docs/packages/node/frontmatter",
            slug: "packages/node/frontmatter",
          },
          {
            labelKey: "sanitizeFn",
            href: "/docs/packages/node/sanitize",
            slug: "packages/node/sanitize",
          },
          {
            labelKey: "syntectFn",
            href: "/docs/packages/node/syntect",
            slug: "packages/node/syntect",
          },
          { labelKey: "tocFn", href: "/docs/packages/node/toc", slug: "packages/node/toc" },
          {
            labelKey: "externalLinksFn",
            href: "/docs/packages/node/external-links",
            slug: "packages/node/external-links",
          },
          {
            labelKey: "autolinkHeadingsFn",
            href: "/docs/packages/node/autolink-headings",
            slug: "packages/node/autolink-headings",
          },
          {
            labelKey: "sectionizeFn",
            href: "/docs/packages/node/sectionize",
            slug: "packages/node/sectionize",
          },
          {
            labelKey: "breaksFn",
            href: "/docs/packages/node/breaks",
            slug: "packages/node/breaks",
          },
          {
            labelKey: "smartypantsFn",
            href: "/docs/packages/node/smartypants",
            slug: "packages/node/smartypants",
          },
          { labelKey: "emojiFn", href: "/docs/packages/node/emoji", slug: "packages/node/emoji" },
          {
            labelKey: "githubAlertFn",
            href: "/docs/packages/node/github-alert",
            slug: "packages/node/github-alert",
          },
          { labelKey: "mathFn", href: "/docs/packages/node/math", slug: "packages/node/math" },
          {
            labelKey: "directiveFn",
            href: "/docs/packages/node/directive",
            slug: "packages/node/directive",
          },
          {
            labelKey: "wikiLinkFn",
            href: "/docs/packages/node/wiki-link",
            slug: "packages/node/wiki-link",
          },
          {
            labelKey: "definitionListFn",
            href: "/docs/packages/node/definition-list",
            slug: "packages/node/definition-list",
          },
          {
            labelKey: "rubyAnnotationFn",
            href: "/docs/packages/node/ruby-annotation",
            slug: "packages/node/ruby-annotation",
          },
          { labelKey: "cjkFn", href: "/docs/packages/node/cjk", slug: "packages/node/cjk" },
          {
            labelKey: "codeImportFn",
            href: "/docs/packages/node/code-import",
            slug: "packages/node/code-import",
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
