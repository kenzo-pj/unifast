import {
  type HastRoot,
  type HastNode,
  type HastElement,
  extractLang as extractLangFromCode,
  extractText,
  findCodeChild,
} from "@unifast/core";
import { createHighlighter, type BundledLanguage, type BundledTheme } from "shiki";

export type ShikiTransformerOptions = {
  themes?: BundledTheme | BundledTheme[] | { light: BundledTheme; dark: BundledTheme };
  defaultTheme?: BundledTheme;
  defaultColor?: string | false;
  langs?: BundledLanguage[];
};

export type ShikiTransformer = {
  transform(hast: HastRoot): HastRoot;
  transformMdxJs(js: string): string;
};

type ResolvedThemeConfig =
  | { kind: "single"; theme: BundledTheme; allThemes: BundledTheme[] }
  | {
      kind: "dual";
      light: BundledTheme;
      dark: BundledTheme;
      allThemes: BundledTheme[];
      defaultColor: string | false;
    };

function resolveThemeConfig(opts: ShikiTransformerOptions): ResolvedThemeConfig {
  const { themes } = opts;

  if (themes && typeof themes === "object" && !Array.isArray(themes) && "light" in themes) {
    return {
      kind: "dual",
      light: themes.light,
      dark: themes.dark,
      allThemes: [themes.light, themes.dark],
      defaultColor: opts.defaultColor ?? false,
    };
  }

  if (Array.isArray(themes)) {
    const defaultTheme = opts.defaultTheme ?? themes[0] ?? "github-dark";
    return { kind: "single", theme: defaultTheme, allThemes: themes };
  }

  if (typeof themes === "string") {
    return { kind: "single", theme: themes, allThemes: [themes] };
  }

  const fallback = opts.defaultTheme ?? "github-dark";
  return { kind: "single", theme: fallback, allThemes: [fallback] };
}

export async function createShikiTransformer(
  options: ShikiTransformerOptions = {},
): Promise<ShikiTransformer> {
  const themeConfig = resolveThemeConfig(options);
  const langs = options.langs ?? [];

  const highlighter = await createHighlighter({ themes: themeConfig.allThemes, langs });
  const loadedLangs = new Set(highlighter.getLoadedLanguages());

  function highlight(code: string, lang: string): string {
    if (themeConfig.kind === "dual") {
      return highlighter.codeToHtml(code, {
        lang,
        themes: { light: themeConfig.light, dark: themeConfig.dark },
        defaultColor: themeConfig.defaultColor,
      });
    }
    return highlighter.codeToHtml(code, { lang, theme: themeConfig.theme });
  }

  function highlightToHast(code: string, lang: string) {
    if (themeConfig.kind === "dual") {
      return highlighter.codeToHast(code, {
        lang: lang as BundledLanguage,
        themes: { light: themeConfig.light, dark: themeConfig.dark },
        defaultColor: themeConfig.defaultColor,
      });
    }
    return highlighter.codeToHast(code, {
      lang: lang as BundledLanguage,
      theme: themeConfig.theme,
    });
  }

  function extractLang(element: HastElement): string | null {
    const code = findCodeChild(element);
    if (!code) return null;
    return extractLangFromCode(code);
  }

  function transformNode(node: HastNode): HastNode {
    if (node.type === "element") {
      if (node.tagName === "pre") {
        const lang = extractLang(node);
        if (lang && loadedLangs.has(lang)) {
          const code = extractText(node);
          const result = highlightToHast(code, lang);
          const pre = result.children[0];
          if (pre && typeof pre === "object" && "type" in pre) {
            return pre as unknown as HastNode;
          }
        }
      }
      return {
        ...node,
        children: node.children.map(transformNode),
      };
    }
    if (node.type === "root") {
      return {
        ...node,
        children: node.children.map(transformNode),
      };
    }
    return node;
  }

  function transformMdxJs(js: string): string {
    const pattern =
      /_jsx\("pre",\s*\{\s*children:\s*_jsx\("code",\s*\{(?:\s*children:\s*("(?:[^"\\]|\\.)*"),\s*className:\s*"language-([\w+-]+)"|\s*className:\s*"language-([\w+-]+)",\s*children:\s*("(?:[^"\\]|\\.)*"))\s*\}\)\s*\}\)/g;
    const replacements: Array<{ start: number; end: number; replacement: string }> = [];

    let m: RegExpExecArray | null;
    while ((m = pattern.exec(js)) !== null) {
      const lang = m[2] ?? m[3];
      const codeStr = JSON.parse(m[1] ?? m[4]) as string;
      if (!loadedLangs.has(lang)) continue;
      try {
        const highlighted = highlight(codeStr, lang);
        const replacement = `_jsx("div", { dangerouslySetInnerHTML: { __html: ${JSON.stringify(highlighted)} } })`;
        replacements.push({ start: m.index, end: m.index + m[0].length, replacement });
      } catch {}
    }

    let result = js;
    for (let i = replacements.length - 1; i >= 0; i--) {
      const r = replacements[i];
      result = result.slice(0, r.start) + r.replacement + result.slice(r.end);
    }
    return result;
  }

  return {
    transform(hast: HastRoot): HastRoot {
      return {
        ...hast,
        children: hast.children.map(transformNode),
      };
    },
    transformMdxJs,
  };
}
