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

export function githubAlert(): UnifastPlugin {
  return { name: "github-alert", options: { githubAlert: { enabled: true } } };
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
