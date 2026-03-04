import type { CompileOptions, CompileResult } from "@unifast/core";

export type MdxCompileOptions = CompileOptions & {
  inputKind: "mdx";
  outputKind?: "html" | "mdxJs";
};

/**
 * Compile MDX input with pre-configured MDX defaults.
 * This is a convenience wrapper around the core compile function.
 */
export function compileMdx(
  input: string,
  options?: Partial<MdxCompileOptions>,
): CompileResult {
  // This would delegate to @unifast/node's compile with inputKind: "mdx"
  // For now, re-export the type interface
  throw new Error(
    "compileMdx requires @unifast/node to be installed and built",
  );
}

export type { CompileOptions, CompileResult } from "@unifast/core";
