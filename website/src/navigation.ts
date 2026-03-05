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
      { labelKey: "whatIsUnifast", href: "/docs/introduction/what-is-unifast", slug: "introduction/what-is-unifast" },
      { labelKey: "quickStart", href: "/docs/introduction/quick-start", slug: "introduction/quick-start" },
      { labelKey: "keyConcepts", href: "/docs/introduction/key-concepts", slug: "introduction/key-concepts" },
    ],
  },
  {
    labelKey: "guides",
    items: [
      { labelKey: "syntaxHighlighting", href: "/docs/guides/syntax-highlighting", slug: "guides/syntax-highlighting" },
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
          { labelKey: "overview", href: "/docs/packages/core/overview", slug: "packages/core/overview" },
          { labelKey: "hastToHtml", href: "/docs/packages/core/hast-to-html", slug: "packages/core/hast-to-html" },
        ],
      },
      {
        labelKey: "node",
        items: [
          { labelKey: "overview", href: "/docs/packages/node/overview", slug: "packages/node/overview" },
          { labelKey: "compileFn", href: "/docs/packages/node/compile", slug: "packages/node/compile" },
        ],
      },
      {
        labelKey: "mdx",
        items: [
          { labelKey: "overview", href: "/docs/packages/mdx/overview", slug: "packages/mdx/overview" },
          { labelKey: "compileMdxFn", href: "/docs/packages/mdx/compile-mdx", slug: "packages/mdx/compile-mdx" },
        ],
      },
    ],
  },
  {
    labelKey: "plugins",
    groups: [
      {
        labelKey: "pluginFrontmatter",
        items: [
          { labelKey: "overview", href: "/docs/plugins/plugin-frontmatter/overview", slug: "plugins/plugin-frontmatter/overview" },
          { labelKey: "frontmatterFn", href: "/docs/plugins/plugin-frontmatter/frontmatter", slug: "plugins/plugin-frontmatter/frontmatter" },
        ],
      },
      {
        labelKey: "pluginGfm",
        items: [
          { labelKey: "overview", href: "/docs/plugins/plugin-gfm/overview", slug: "plugins/plugin-gfm/overview" },
          { labelKey: "gfmFn", href: "/docs/plugins/plugin-gfm/gfm", slug: "plugins/plugin-gfm/gfm" },
        ],
      },
      {
        labelKey: "pluginSyntect",
        items: [
          { labelKey: "overview", href: "/docs/plugins/plugin-syntect/overview", slug: "plugins/plugin-syntect/overview" },
          { labelKey: "syntectFn", href: "/docs/plugins/plugin-syntect/syntect", slug: "plugins/plugin-syntect/syntect" },
        ],
      },
      {
        labelKey: "pluginShiki",
        items: [
          { labelKey: "overview", href: "/docs/plugins/plugin-shiki/overview", slug: "plugins/plugin-shiki/overview" },
          { labelKey: "createShikiPluginFn", href: "/docs/plugins/plugin-shiki/create-shiki-plugin", slug: "plugins/plugin-shiki/create-shiki-plugin" },
          { labelKey: "createShikiTransformerFn", href: "/docs/plugins/plugin-shiki/create-shiki-transformer", slug: "plugins/plugin-shiki/create-shiki-transformer" },
        ],
      },
      {
        labelKey: "pluginSanitize",
        items: [
          { labelKey: "overview", href: "/docs/plugins/plugin-sanitize/overview", slug: "plugins/plugin-sanitize/overview" },
          { labelKey: "sanitizeFn", href: "/docs/plugins/plugin-sanitize/sanitize", slug: "plugins/plugin-sanitize/sanitize" },
        ],
      },
      {
        labelKey: "pluginToc",
        items: [
          { labelKey: "overview", href: "/docs/plugins/plugin-toc/overview", slug: "plugins/plugin-toc/overview" },
          { labelKey: "tocFn", href: "/docs/plugins/plugin-toc/toc", slug: "plugins/plugin-toc/toc" },
        ],
      },
      {
        labelKey: "pluginReact",
        items: [
          { labelKey: "overview", href: "/docs/plugins/plugin-react/overview", slug: "plugins/plugin-react/overview" },
          { labelKey: "compileToReactFn", href: "/docs/plugins/plugin-react/compile-to-react", slug: "plugins/plugin-react/compile-to-react" },
          { labelKey: "hastToReactFn", href: "/docs/plugins/plugin-react/hast-to-react", slug: "plugins/plugin-react/hast-to-react" },
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
