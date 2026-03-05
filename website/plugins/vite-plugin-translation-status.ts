import type { Plugin } from "vite";
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";

export type TranslationStatus = "translated" | "outdated" | "missing";

export interface TranslationStatusEntry {
  status: TranslationStatus;
  enLastModified?: number;
  jaLastModified?: number;
}

export type TranslationManifest = Record<string, TranslationStatusEntry>;

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

function buildManifest(contentDir: string, gitRoot: string): TranslationManifest {
  const enDir = path.join(contentDir, "en");
  const jaDir = path.join(contentDir, "ja");
  const manifest: TranslationManifest = {};

  const contentPaths = collectContentPaths(enDir);

  for (const slug of contentPaths) {
    const enFile = path.join(enDir, `${slug}.md`);
    const jaFile = path.join(jaDir, `${slug}.md`);

    const enRelative = path.relative(gitRoot, enFile);
    const jaRelative = path.relative(gitRoot, jaFile);

    if (!fs.existsSync(jaFile)) {
      manifest[slug] = { status: "missing" };
      continue;
    }

    const enTimestamp = getGitTimestamp(enRelative, gitRoot);
    const jaTimestamp = getGitTimestamp(jaRelative, gitRoot);

    if (enTimestamp && jaTimestamp && enTimestamp > jaTimestamp) {
      manifest[slug] = { status: "outdated", enLastModified: enTimestamp, jaLastModified: jaTimestamp };
    } else {
      manifest[slug] = { status: "translated", enLastModified: enTimestamp ?? undefined, jaLastModified: jaTimestamp ?? undefined };
    }
  }

  return manifest;
}

export default function translationStatusPlugin(): Plugin {
  let contentDir: string;
  let gitRoot: string;

  return {
    name: "vite-plugin-translation-status",

    configResolved(config) {
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

    resolveId(id) {
      if (id === VIRTUAL_MODULE_ID) return RESOLVED_VIRTUAL_MODULE_ID;
    },

    load(id) {
      if (id === RESOLVED_VIRTUAL_MODULE_ID) {
        const manifest = buildManifest(contentDir, gitRoot);
        return `export default ${JSON.stringify(manifest)};`;
      }
    },
  };
}
