import type { UnifastPlugin } from "./plugin";

export type CompileOptions = {
  inputKind?: "md" | "mdx";
  outputKind?: "html" | "hast" | "mdast" | "mdxJs";

  gfm?: {
    tables?: boolean;
    taskList?: boolean;
    strikethrough?: boolean;
    footnotes?: boolean;
    autolink?: boolean;
  };

  frontmatter?: { yaml?: boolean; toml?: boolean; json?: boolean };

  rawHtml?: "disallow" | "allowDangerous" | "parseAndSanitize";

  sanitize?: { enabled?: boolean; schema?: SanitizeSchema };

  highlight?: { enabled?: boolean; engine?: "none" | "syntect" | "treeSitter" };

  lineNumbers?: { enabled?: boolean };

  slug?: { mode?: "github" | "unicode" };

  toc?: { enabled?: boolean; maxDepth?: number };

  externalLinks?: { enabled?: boolean; rel?: string; target?: string };

  autolinkHeadings?: {
    enabled?: boolean;
    behavior?: "prepend" | "append" | "wrap";
  };

  sectionize?: { enabled?: boolean };

  breaks?: { enabled?: boolean };

  smartypants?: {
    enabled?: boolean;
    quotes?: boolean;
    dashes?: boolean;
    ellipses?: boolean;
  };

  emoji?: { enabled?: boolean };

  githubAlert?: {
    enabled?: boolean;
    icons?: "none" | "octicon";
    customIcons?: Record<
      string,
      string | { svg?: string; importName?: string; importSource?: string }
    >;
  };

  math?: { enabled?: boolean };

  directive?: { enabled?: boolean };

  wikiLink?: { enabled?: boolean; hrefTemplate?: string };

  definitionList?: { enabled?: boolean };

  rubyAnnotation?: { enabled?: boolean };

  cjk?: { enabled?: boolean };

  codeImport?: { enabled?: boolean; rootDir?: string };

  codeMeta?: { enabled?: boolean };
  figure?: { enabled?: boolean };
  customHeadingId?: { enabled?: boolean };
  readingTime?: { enabled?: boolean; wordsPerMinute?: number; cjkCharsPerMinute?: number };
  excerpt?: {
    enabled?: boolean;
    separator?: string;
    fallbackParagraphs?: number;
    fallbackCharacters?: number;
  };
  abbr?: { enabled?: boolean };
  commentRemoval?: { enabled?: boolean };
  imgLazyLoading?: { enabled?: boolean; skipFirst?: number };
  accessibleEmoji?: { enabled?: boolean };
  addClasses?: { enabled?: boolean; rules?: { selector: string; classes: string }[] };
  htmlCleanup?: { removeEmptyNodes?: boolean; minifyWhitespace?: boolean };
  minify?: { enabled?: boolean };

  diagnostics?: { format?: "compact" | "verbose" };

  plugins?: UnifastPlugin[];
};

export type TocEntry = {
  depth: number;
  text: string;
  slug: string;
};

export type SanitizeSchema = {
  allowedTags?: string[];
  allowedAttributes?: Record<string, string[]>;
  allowedProtocols?: Record<string, string[]>;
};
