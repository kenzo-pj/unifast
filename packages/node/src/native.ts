import type { CompileOptions, CompileResult } from "@unifast/core";

// Native binding interface
// In production, this loads the napi-rs compiled binary
// For now, this is a placeholder that will be replaced by the actual native module

interface NativeBinding {
  compile(input: string, options?: object): {
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
  };
}

let nativeBinding: NativeBinding | null = null;

export function loadNativeBinding(): NativeBinding {
  if (nativeBinding) return nativeBinding;

  try {
    // Try to load the native addon
    // The actual path depends on the platform and build
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    nativeBinding = require("../native/unifast.node") as NativeBinding;
  } catch {
    throw new Error(
      "Failed to load unifast native binding. " +
        "Make sure the native addon is built. " +
        "Run `pnpm build` in the project root.",
    );
  }

  return nativeBinding!;
}
