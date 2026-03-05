import type { TocEntry } from "./options.js";

export type CompileResult = {
  output: string | object;
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
};
