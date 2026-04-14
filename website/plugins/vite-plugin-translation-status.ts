import type { Plugin, ResolvedConfig } from "vite";
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";

import { SUPPORTED_LOCALES, DEFAULT_LOCALE, type LocaleCode } from "../src/i18n";

export type TranslationStatus = "translated" | "outdated" | "missing";

export interface TranslationStatusEntry {
  status: TranslationStatus;
  enLastModified?: number;
  localeLastModified?: number;
}

export type TranslationManifest = Partial<
  Record<LocaleCode, Record<string, TranslationStatusEntry>>
>;

const VIRTUAL_MODULE_ID = "virtual:translation-status";
const RESOLVED_VIRTUAL_MODULE_ID = "\0" + VIRTUAL_MODULE_ID;

function getGitTimestamp(filePath: string, cwd: string): number | null {
  try {
    const result = execFileSync("git", ["log", "-1", "--format=%ct", "--", filePath], {
      cwd,
      encoding: "utf-8",
    }).trim();
    return result ? parseInt(result, 10) : null;
  } catch {
    return null;
  }
}

function collectContentPaths(dir: string, prefix = ""): string[] {
  const paths: string[] = [];
  if (!fs.existsSync(dir)) return paths;

  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    if (entry.isDirectory()) {
      paths.push(...collectContentPaths(path.join(dir, entry.name), `${prefix}${entry.name}/`));
    } else if (entry.name.endsWith(".md") || entry.name.endsWith(".mdx")) {
      paths.push(`${prefix}${entry.name.replace(/\.(md|mdx)$/, "")}`);
    }
  }
  return paths;
}

function resolveContentFile(dir: string, slug: string): string | null {
  const md = path.join(dir, `${slug}.md`);
  const mdx = path.join(dir, `${slug}.mdx`);
  if (fs.existsSync(mdx)) return mdx;
  if (fs.existsSync(md)) return md;
  return null;
}

function buildManifest(contentDir: string, gitRoot: string): TranslationManifest {
  const enDir = path.join(contentDir, "en");
  const manifest: TranslationManifest = {};

  const contentPaths = collectContentPaths(enDir);

  for (const locale of SUPPORTED_LOCALES) {
    if (locale === DEFAULT_LOCALE) continue;
    const localeManifest: Record<string, TranslationStatusEntry> = {};
    const localeDir = path.join(contentDir, locale);

    for (const slug of contentPaths) {
      const enFile = resolveContentFile(enDir, slug);
      const localeFile = resolveContentFile(localeDir, slug);

      if (!enFile) continue;

      if (!localeFile) {
        localeManifest[slug] = { status: "missing" };
        continue;
      }

      const enRelative = path.relative(gitRoot, enFile);
      const localeRelative = path.relative(gitRoot, localeFile);
      const enTimestamp = getGitTimestamp(enRelative, gitRoot);
      const localeTimestamp = getGitTimestamp(localeRelative, gitRoot);

      if (enTimestamp && localeTimestamp && enTimestamp > localeTimestamp) {
        localeManifest[slug] = {
          status: "outdated",
          enLastModified: enTimestamp,
          localeLastModified: localeTimestamp,
        };
      } else {
        localeManifest[slug] = {
          status: "translated",
          enLastModified: enTimestamp ?? undefined,
          localeLastModified: localeTimestamp ?? undefined,
        };
      }
    }

    manifest[locale] = localeManifest;
  }

  return manifest;
}

export default function translationStatusPlugin(): Plugin {
  let contentDir: string;
  let gitRoot: string;

  return {
    name: "vite-plugin-translation-status",

    configResolved(config: ResolvedConfig) {
      const root = config.root;
      contentDir = path.resolve(root, "content");
      let dir = root;
      while (dir !== path.dirname(dir)) {
        if (fs.existsSync(path.join(dir, ".git"))) {
          gitRoot = dir;
          break;
        }
        dir = path.dirname(dir);
      }
      if (!gitRoot) gitRoot = root;
    },

    resolveId(id: string) {
      if (id === VIRTUAL_MODULE_ID) return RESOLVED_VIRTUAL_MODULE_ID;
    },

    load(id: string) {
      if (id === RESOLVED_VIRTUAL_MODULE_ID) {
        const manifest = buildManifest(contentDir, gitRoot);
        return `export default ${JSON.stringify(manifest)};`;
      }
    },
  };
}
