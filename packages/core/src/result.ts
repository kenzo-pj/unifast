import type { TocEntry } from "./options";

export type CompileResult = {
  output: string | object;
  sourcemap?: string;
  frontmatter: Record<string, unknown>;
  diagnostics: Array<{
    level: "error" | "warn";
    message: string;
    start?: number;
    end?: number;
    line?: number;
    column?: number;
  }>;
  stats: { parseMs: number; transformMs: number; emitMs: number };
  toc: TocEntry[];
  readingTime?: { minutes: number; words: number };
  excerpt?: string;
};
