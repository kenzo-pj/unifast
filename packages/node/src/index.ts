import type { CompileOptions, CompileResult, HastRoot, HastNode, HastElement } from "@unifast/core";
import { hastToHtml } from "@unifast/core";
import deepmerge from "deepmerge";

import { loadNativeBinding } from "./native";

export type { CompileOptions, CompileResult, UnifastPlugin, TocEntry } from "@unifast/core";
export type { HastRoot, HastElement, HastText, HastNode } from "@unifast/core";
export { hastToHtml } from "@unifast/core";
export { UnifastError, ParseError, CompileError } from "@unifast/core";
export {
  gfm,
  frontmatter,
  sanitize,
  syntect,
  treeSitter,
  toc,
  externalLinks,
  autolinkHeadings,
  smartypants,
  wikiLink,
  codeImport,
  emoji,
  breaks,
  math,
  githubAlert,
  sectionize,
  directive,
  definitionList,
  rubyAnnotation,
  cjk,
} from "@unifast/core";
export type {
  GfmPluginOptions,
  FrontmatterPluginOptions,
  SanitizePluginOptions,
  SyntectPluginOptions,
  TreeSitterPluginOptions,
  TocPluginOptions,
  ExternalLinksPluginOptions,
  AutolinkHeadingsPluginOptions,
  SmartypantsPluginOptions,
  WikiLinkPluginOptions,
  CodeImportPluginOptions,
} from "@unifast/core";

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

export function compile(input: string, options?: CompileOptions): CompileResult {
  const native = loadNativeBinding();

  const plugins = options?.plugins ?? [];
  const hastTransforms = plugins
    .filter((p): p is typeof p & { hastTransform: NonNullable<typeof p.hastTransform> } =>
      Boolean(p.hastTransform),
    )
    .map((p) => p.hastTransform);
  const mdxJsTransforms = plugins
    .filter((p): p is typeof p & { mdxJsTransform: NonNullable<typeof p.mdxJsTransform> } =>
      Boolean(p.mdxJsTransform),
    )
    .map((p) => p.mdxJsTransform);

  let mergedOpts: Record<string, unknown> = { ...options };
  delete mergedOpts.plugins;
  for (const plugin of plugins) {
    if (plugin.options) {
      mergedOpts = deepmerge(mergedOpts, plugin.options as Record<string, unknown>);
    }
  }

  const hasHastTransforms = hastTransforms.length > 0;
  const userRequestedOutputKind = options?.outputKind;
  if (
    hasHastTransforms &&
    userRequestedOutputKind !== "hast" &&
    userRequestedOutputKind !== "mdxJs"
  ) {
    mergedOpts.outputKind = "hast";
  }

  const rawResult = native.compile(input, mergedOpts);

  let { output } = rawResult;
  if (hasHastTransforms && userRequestedOutputKind !== "mdxJs") {
    let hast: HastRoot;
    try {
      hast = JSON.parse(output) as HastRoot;
    } catch {
      throw new Error("Failed to parse HAST output from native binding");
    }
    for (const transform of hastTransforms) {
      hast = transform(hast);
    }
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
  if (
    mdxJsTransforms.length > 0 &&
    (userRequestedOutputKind === "mdxJs" || mergedOpts.outputKind === "mdxJs")
  ) {
    for (const transform of mdxJsTransforms) {
      output = transform(output);
    }
  }

  let frontmatter: Record<string, unknown>;
  try {
    frontmatter = JSON.parse(rawResult.frontmatter) as Record<string, unknown>;
  } catch {
    frontmatter = {};
  }

  return {
    output,
    frontmatter,
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
