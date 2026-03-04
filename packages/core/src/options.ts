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

  highlight?: { enabled?: boolean; engine?: "none" | "builtin" };

  slug?: { mode?: "github" | "unicode" };

  toc?: { enabled?: boolean; maxDepth?: number };

  diagnostics?: { format?: "compact" | "verbose" };

  cache?: { enabled?: boolean; dir?: string };

  plugins?: Array<{ name: string; options?: unknown }>;
};

export type SanitizeSchema = {
  allowedTags?: string[];
  allowedAttributes?: Record<string, string[]>;
  allowedProtocols?: Record<string, string[]>;
};
