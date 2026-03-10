import { createRequire } from "node:module";

interface NativeBinding {
  compile(
    input: string,
    options?: object,
  ): {
    output: string;
    frontmatter: string;
    diagnostics: Array<{
      level: string;
      message: string;
      start?: number;
      end?: number;
    }>;
    stats: {
      parseMs: number;
      transformMs: number;
      emitMs: number;
    };
    toc: Array<{
      depth: number;
      text: string;
      slug: string;
    }>;
    readingTime?: {
      words: number;
      minutes: number;
    };
    excerpt?: string;
  };
  stringifyHast(json: string): string;
}

let nativeBinding: NativeBinding | null = null;

export function loadNativeBinding(): NativeBinding {
  if (nativeBinding) return nativeBinding;

  try {
    const require = createRequire(import.meta.url);
    nativeBinding = require("../native/unifast.node") as NativeBinding;
  } catch {
    throw new Error(
      "Failed to load unifast native binding. " +
        "Make sure the native addon is built. " +
        "Run `cargo build -p unifast-bindings-node --release` first.",
    );
  }

  return nativeBinding!;
}
