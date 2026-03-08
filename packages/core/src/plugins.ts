import type { SanitizeSchema } from "./options";
import type { UnifastPlugin } from "./plugin";

export type GfmPluginOptions = {
  tables?: boolean;
  taskList?: boolean;
  strikethrough?: boolean;
  footnotes?: boolean;
  autolink?: boolean;
};

export function gfm(options?: GfmPluginOptions): UnifastPlugin {
  return {
    name: "gfm",
    options: {
      gfm: {
        tables: options?.tables ?? true,
        taskList: options?.taskList ?? true,
        strikethrough: options?.strikethrough ?? true,
        footnotes: options?.footnotes ?? true,
        autolink: options?.autolink ?? true,
      },
    },
  };
}

export type FrontmatterPluginOptions = {
  yaml?: boolean;
  toml?: boolean;
  json?: boolean;
};

export function frontmatter(options?: FrontmatterPluginOptions): UnifastPlugin {
  return {
    name: "frontmatter",
    options: {
      frontmatter: {
        yaml: options?.yaml ?? true,
        toml: options?.toml ?? false,
        json: options?.json ?? false,
      },
    },
  };
}

export type SanitizePluginOptions = {
  enabled?: boolean;
  schema?: SanitizeSchema;
};

export function sanitize(options?: SanitizePluginOptions): UnifastPlugin {
  return {
    name: "sanitize",
    options: {
      sanitize: {
        enabled: options?.enabled ?? true,
        schema: options?.schema,
      },
    },
  };
}

export type SyntectPluginOptions = {
  engine?: "none" | "syntect";
};

export function syntect(options?: SyntectPluginOptions): UnifastPlugin {
  return {
    name: "syntect",
    options: {
      highlight: {
        enabled: true,
        engine: options?.engine ?? "syntect",
      },
    },
  };
}

export type TreeSitterPluginOptions = {
  engine?: "none" | "treeSitter";
};

export function treeSitter(options?: TreeSitterPluginOptions): UnifastPlugin {
  return {
    name: "tree-sitter",
    options: {
      highlight: {
        enabled: true,
        engine: options?.engine ?? "treeSitter",
      },
    },
  };
}

export type TocPluginOptions = {
  maxDepth?: number;
};

export function toc(options?: TocPluginOptions): UnifastPlugin {
  return {
    name: "toc",
    options: {
      toc: {
        enabled: true,
        maxDepth: options?.maxDepth ?? 3,
      },
    },
  };
}

export type ExternalLinksPluginOptions = {
  rel?: string;
  target?: string;
};

export function externalLinks(options?: ExternalLinksPluginOptions): UnifastPlugin {
  return {
    name: "external-links",
    options: {
      externalLinks: {
        enabled: true,
        rel: options?.rel ?? "nofollow noopener noreferrer",
        target: options?.target,
      },
    },
  };
}

export type AutolinkHeadingsPluginOptions = {
  behavior?: "prepend" | "append" | "wrap";
};

export function autolinkHeadings(options?: AutolinkHeadingsPluginOptions): UnifastPlugin {
  return {
    name: "autolink-headings",
    options: {
      autolinkHeadings: {
        enabled: true,
        behavior: options?.behavior ?? "prepend",
      },
    },
  };
}

export type SmartypantsPluginOptions = {
  quotes?: boolean;
  dashes?: boolean;
  ellipses?: boolean;
};

export function smartypants(options?: SmartypantsPluginOptions): UnifastPlugin {
  return {
    name: "smartypants",
    options: {
      smartypants: {
        enabled: true,
        quotes: options?.quotes ?? true,
        dashes: options?.dashes ?? true,
        ellipses: options?.ellipses ?? true,
      },
    },
  };
}

export type WikiLinkPluginOptions = {
  hrefTemplate?: string;
};

export function wikiLink(options?: WikiLinkPluginOptions): UnifastPlugin {
  return {
    name: "wiki-link",
    options: {
      wikiLink: {
        enabled: true,
        hrefTemplate: options?.hrefTemplate ?? "/wiki/${slug}",
      },
    },
  };
}

export type CodeImportPluginOptions = {
  rootDir?: string;
};

export function codeImport(options?: CodeImportPluginOptions): UnifastPlugin {
  return {
    name: "code-import",
    options: {
      codeImport: {
        enabled: true,
        rootDir: options?.rootDir,
      },
    },
  };
}

export function emoji(): UnifastPlugin {
  return { name: "emoji", options: { emoji: { enabled: true } } };
}

export function breaks(): UnifastPlugin {
  return { name: "breaks", options: { breaks: { enabled: true } } };
}

export function math(): UnifastPlugin {
  return { name: "math", options: { math: { enabled: true } } };
}

export type GithubAlertIconDef =
  | string
  | { svg?: string; importName?: string; importSource?: string };

export type GithubAlertPluginOptions = {
  icons?: "none" | "octicon" | Record<string, GithubAlertIconDef>;
};

export function githubAlert(options?: GithubAlertPluginOptions): UnifastPlugin {
  const iconsOpt = options?.icons;

  let icons: "none" | "octicon" | undefined;
  let customIcons:
    | Record<string, { svg?: string; importName?: string; importSource?: string }>
    | undefined;

  if (iconsOpt === "none") {
    icons = "none";
  } else if (iconsOpt === "octicon" || iconsOpt === undefined) {
    icons = "octicon";
  } else {
    customIcons = {};
    for (const [key, val] of Object.entries(iconsOpt)) {
      if (typeof val === "string") {
        customIcons[key] = { svg: val };
      } else {
        customIcons[key] = val;
      }
    }
  }

  return {
    name: "github-alert",
    options: {
      githubAlert: {
        enabled: true,
        icons,
        customIcons,
      },
    },
  };
}

export function sectionize(): UnifastPlugin {
  return { name: "sectionize", options: { sectionize: { enabled: true } } };
}

export function directive(): UnifastPlugin {
  return { name: "directive", options: { directive: { enabled: true } } };
}

export function definitionList(): UnifastPlugin {
  return { name: "definition-list", options: { definitionList: { enabled: true } } };
}

export function rubyAnnotation(): UnifastPlugin {
  return { name: "ruby-annotation", options: { rubyAnnotation: { enabled: true } } };
}

export function cjk(): UnifastPlugin {
  return { name: "cjk", options: { cjk: { enabled: true } } };
}

export function codeMeta(): UnifastPlugin {
  return { name: "code-meta", options: { codeMeta: { enabled: true } } };
}

export function figure(): UnifastPlugin {
  return { name: "figure", options: { figure: { enabled: true } } };
}

export function customHeadingId(): UnifastPlugin {
  return { name: "custom-heading-id", options: { customHeadingId: { enabled: true } } };
}

export type ReadingTimePluginOptions = {
  wordsPerMinute?: number;
  cjkCharsPerMinute?: number;
};

export function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin {
  return {
    name: "reading-time",
    options: {
      readingTime: {
        enabled: true,
        wordsPerMinute: options?.wordsPerMinute ?? 200,
        cjkCharsPerMinute: options?.cjkCharsPerMinute ?? 500,
      },
    },
  };
}

export type ExcerptPluginOptions = {
  separator?: string;
  fallbackParagraphs?: number;
  fallbackCharacters?: number;
};

export function excerpt(options?: ExcerptPluginOptions): UnifastPlugin {
  return {
    name: "excerpt",
    options: {
      excerpt: {
        enabled: true,
        separator: options?.separator ?? "<!-- more -->",
        fallbackParagraphs: options?.fallbackParagraphs ?? 1,
      },
    },
  };
}

export function abbr(): UnifastPlugin {
  return { name: "abbr", options: { abbr: { enabled: true } } };
}

export function commentRemoval(): UnifastPlugin {
  return { name: "comment-removal", options: { commentRemoval: { enabled: true } } };
}

export type ImgLazyLoadingPluginOptions = {
  skipFirst?: number;
};

export function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin {
  return {
    name: "img-lazy-loading",
    options: {
      imgLazyLoading: {
        enabled: true,
        skipFirst: options?.skipFirst ?? 0,
      },
    },
  };
}

export function accessibleEmoji(): UnifastPlugin {
  return { name: "accessible-emoji", options: { accessibleEmoji: { enabled: true } } };
}

export function addClasses(rules: Record<string, string>): UnifastPlugin {
  return {
    name: "add-classes",
    options: {
      addClass: {
        enabled: true,
        rules,
      },
    },
  };
}

export function minify(): UnifastPlugin {
  return { name: "minify", options: { minify: { enabled: true } } };
}
