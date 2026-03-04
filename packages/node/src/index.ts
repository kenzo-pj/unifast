import type { CompileOptions, CompileResult } from "@unifast/core";
export type { CompileOptions, CompileResult } from "@unifast/core";
export { UnifastError, ParseError, CompileError } from "@unifast/core";

// Re-export the compile function
// When native binding is available, this delegates to the Rust core
// For now, provides the type-safe interface

/**
 * Compile Markdown or MDX input to HTML or other formats.
 *
 * @param input - The Markdown/MDX source text
 * @param options - Compilation options
 * @returns The compilation result including output, frontmatter, diagnostics, and stats
 */
export function compile(
  input: string,
  options?: CompileOptions,
): CompileResult {
  // Load native binding
  const { loadNativeBinding } = require("./native.js") as {
    loadNativeBinding: () => {
      compile: (
        input: string,
        options?: object,
      ) => {
        output: string;
        frontmatter: string;
        diagnostics: Array<{
          level: string;
          message: string;
          start?: number;
          end?: number;
        }>;
        stats: { parseMs: number; transformMs: number; emitMs: number };
      };
    };
  };

  const native = loadNativeBinding();
  const rawResult = native.compile(input, options);

  return {
    output: rawResult.output,
    frontmatter: JSON.parse(rawResult.frontmatter),
    diagnostics: rawResult.diagnostics.map((d) => ({
      level: d.level as "error" | "warn",
      message: d.message,
      start: d.start,
      end: d.end,
    })),
    stats: rawResult.stats,
  };
}
