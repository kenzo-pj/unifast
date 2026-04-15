import { createRequire } from "node:module";

import { familySync, MUSL } from "detect-libc";

interface NativeBinding {
  compile(
    input: string,
    options?: object,
  ): {
    output: string;
    sourcemap?: string;
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

function resolvePlatformBindingName(): string {
  const { platform, arch } = process;

  if (platform === "darwin") {
    return `unifast.darwin-${arch}.node`;
  }
  if (platform === "win32") {
    return `unifast.win32-${arch}.node`;
  }
  if (platform === "linux") {
    const family = familySync() === MUSL ? "musl" : "gnu";
    return `unifast.linux-${arch}-${family}.node`;
  }

  throw new Error(`unifast: unsupported platform ${platform}-${arch}`);
}

export function loadNativeBinding(): NativeBinding {
  if (nativeBinding) return nativeBinding;

  const require = createRequire(import.meta.url);
  const platformName = resolvePlatformBindingName();
  const candidates = [`../native/${platformName}`, "../native/unifast.node"];

  const errors: string[] = [];
  for (const candidate of candidates) {
    try {
      nativeBinding = require(candidate) as NativeBinding;
      return nativeBinding;
    } catch (error) {
      errors.push(`  ${candidate}: ${(error as Error).message}`);
    }
  }

  throw new Error(
    `Failed to load unifast native binding. Tried:\n${errors.join("\n")}\n` +
      "If you are developing locally, run `cargo build -p unifast-bindings-node --release` " +
      "and ensure the output is placed at packages/node/native/unifast.node.",
  );
}
