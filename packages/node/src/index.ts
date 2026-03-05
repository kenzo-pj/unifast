import type { CompileOptions, CompileResult, HastRoot, HastNode, HastElement } from "@unifast/core";
import { hastToHtml } from "@unifast/core";
import { loadNativeBinding } from "./native.js";

export type { CompileOptions, CompileResult, UnifastPlugin, TocEntry } from "@unifast/core";
export type { HastRoot, HastElement, HastText, HastNode } from "@unifast/core";
export { hastToHtml } from "@unifast/core";
export { UnifastError, ParseError, CompileError } from "@unifast/core";

function deepMerge(target: Record<string, unknown>, source: Record<string, unknown>): Record<string, unknown> {
  const result = { ...target };
  for (const key of Object.keys(source)) {
    const srcVal = source[key];
    const tgtVal = result[key];
    if (
      srcVal && typeof srcVal === "object" && !Array.isArray(srcVal) &&
      tgtVal && typeof tgtVal === "object" && !Array.isArray(tgtVal)
    ) {
      result[key] = deepMerge(tgtVal as Record<string, unknown>, srcVal as Record<string, unknown>);
    } else {
      result[key] = srcVal;
    }
  }
  return result;
}

function applyDataLineAttributes(node: HastNode): void {
  if (node.type === "element") {
    if (node.tagName === "pre") {
      const code = node.children.find(
        (c): c is HastElement => c.type === "element" && c.tagName === "code",
      );
      if (code) {
        let lineNum = 1;
        for (const child of code.children) {
          if (child.type === "element" && child.tagName === "span") {
            const cls = child.properties.className ?? child.properties.class;
            const hasLine = Array.isArray(cls)
              ? cls.includes("line")
              : typeof cls === "string" && cls.split(" ").includes("line");
            if (hasLine) {
              child.properties["data-line"] = String(lineNum++);
            }
          }
        }
      }
    } else {
      for (const child of node.children) {
        applyDataLineAttributes(child);
      }
    }
  } else if (node.type === "root") {
    for (const child of node.children) {
      applyDataLineAttributes(child);
    }
  }
}

/**
 * Compile Markdown or MDX input to HTML or other formats.
 */
export function compile(
  input: string,
  options?: CompileOptions,
): CompileResult {
  const native = loadNativeBinding();

  // 1. Separate plugins from options
  const plugins = options?.plugins ?? [];
  const hastTransforms = plugins
    .filter((p) => p.hastTransform)
    .map((p) => p.hastTransform!);
  const mdxJsTransforms = plugins
    .filter((p) => p.mdxJsTransform)
    .map((p) => p.mdxJsTransform!);

  // 2. Merge plugin options into compile options
  let mergedOpts: Record<string, unknown> = { ...options };
  delete mergedOpts.plugins;
  for (const plugin of plugins) {
    if (plugin.options) {
      mergedOpts = deepMerge(mergedOpts, plugin.options as Record<string, unknown>);
    }
  }

  // 3. If HAST transforms exist and output is not mdxJs, force HAST output
  const hasHastTransforms = hastTransforms.length > 0;
  const userRequestedOutputKind = options?.outputKind;
  if (hasHastTransforms && userRequestedOutputKind !== "hast" && userRequestedOutputKind !== "mdxJs") {
    mergedOpts.outputKind = "hast";
  }

  // 4. Call Rust native binding
  const rawResult = native.compile(input, mergedOpts);

  // 5. Apply plugin transforms
  let output = rawResult.output;
  if (hasHastTransforms && userRequestedOutputKind !== "mdxJs") {
    let hast: HastRoot = JSON.parse(output);
    for (const transform of hastTransforms) {
      hast = transform(hast);
    }
    // 5b. Re-apply data-line after plugin transforms (shiki replaces code blocks)
    if (options?.lineNumbers?.enabled) {
      for (const child of hast.children) {
        applyDataLineAttributes(child);
      }
    }
    if (userRequestedOutputKind === "hast") {
      output = JSON.stringify(hast);
    } else {
      output = hastToHtml(hast);
    }
  }
  if (mdxJsTransforms.length > 0 && (userRequestedOutputKind === "mdxJs" || mergedOpts.outputKind === "mdxJs")) {
    for (const transform of mdxJsTransforms) {
      output = transform(output);
    }
  }

  return {
    output,
    frontmatter: JSON.parse(rawResult.frontmatter),
    diagnostics: rawResult.diagnostics.map((d) => ({
      level: d.level as "error" | "warn",
      message: d.message,
      start: d.start,
      end: d.end,
    })),
    stats: rawResult.stats,
    toc: rawResult.toc ?? [],
  };
}
