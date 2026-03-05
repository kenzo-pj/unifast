import fs from "node:fs";
import path from "node:path";

const LOCALES = ["en", "ja"] as const;
const DEFAULT_LOCALE = "en";
const SITE_URL = "https://unifast.dev";

function parseFrontmatter(raw: string): { title: string; body: string } {
  const match = raw.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/);
  if (!match) return { title: "", body: raw };
  const titleMatch = match[1].match(/title:\s*["']?(.+?)["']?\s*$/m);
  return { title: titleMatch?.[1] ?? "", body: match[2] };
}

function stripMdxSyntax(body: string): string {
  return body
    .replace(/^import\s.+$/gm, "")
    .replace(/<[A-Z][\w]*\b[^>]*\/>/g, "")
    .replace(/<[A-Z][\w]*\b[^>]*>[\s\S]*?<\/[A-Z][\w]*>/g, "")
    .replace(/\n{3,}/g, "\n\n")
    .trim();
}

interface ContentEntry {
  route: string;
  title: string;
  body: string;
}

function collectContentEntries(dir: string, prefix = ""): ContentEntry[] {
  const entries: ContentEntry[] = [];
  if (!fs.existsSync(dir)) return entries;

  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      entries.push(
        ...collectContentEntries(path.join(dir, entry.name), `${prefix}/${entry.name}`),
      );
    } else if (entry.name.endsWith(".md") || entry.name.endsWith(".mdx")) {
      const slug = entry.name.replace(/\.(md|mdx)$/, "");
      const route =
        slug === "index" && prefix === "" ? "/" :
        slug === "index" ? `/docs${prefix}` :
        `/docs${prefix}/${slug}`;
      const raw = fs.readFileSync(path.join(dir, entry.name), "utf-8");
      const { title, body } = parseFrontmatter(raw);
      const cleanBody = entry.name.endsWith(".mdx") ? stripMdxSyntax(body) : body;
      entries.push({ route, title, body: cleanBody });
    }
  }
  return entries;
}

function collectRoutes(dir: string, prefix = ""): string[] {
  const routes: string[] = [];
  if (!fs.existsSync(dir)) return routes;

  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      routes.push(...collectRoutes(path.join(dir, entry.name), `${prefix}/${entry.name}`));
    } else if (entry.name.endsWith(".md") || entry.name.endsWith(".mdx")) {
      const slug = entry.name.replace(/\.(md|mdx)$/, "");
      if (slug === "index" && prefix === "") routes.push("/");
      else if (slug === "index") routes.push(`/docs${prefix}`);
      else routes.push(`/docs${prefix}/${slug}`);
    }
  }
  return routes;
}

export function buildSitemap(contentDir: string): string {
  const allRoutes: string[] = [];
  for (const locale of LOCALES) {
    const routes = collectRoutes(path.resolve(contentDir, locale));
    for (const route of routes) {
      allRoutes.push(locale === DEFAULT_LOCALE ? route : `/${locale}${route}`);
    }
  }
  const urls = allRoutes
    .map((r) => `  <url>\n    <loc>${SITE_URL}${r === "/" ? "" : r}</loc>\n  </url>`)
    .join("\n");
  return `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`;
}

export function buildLlmsTxt(contentDir: string): string {
  const entries = getSortedEntries(contentDir);
  const intro = entries.find((e) => e.route === "/");
  const firstLine = intro?.body.split("\n").find((l) => l.trim().length > 0) ?? "";

  let out = `# unifast\n\n> ${firstLine}\n\n## Docs\n\n`;
  for (const entry of entries) {
    out += `- [${entry.title}](${SITE_URL}${entry.route === "/" ? "" : entry.route})\n`;
  }
  return out;
}

export function buildLlmsFullTxt(contentDir: string): string {
  const entries = getSortedEntries(contentDir);
  const sections = entries.map((e) => `# ${e.title}\n\n${e.body.trim()}`);
  return sections.join("\n\n---\n\n") + "\n";
}

function getSortedEntries(contentDir: string): ContentEntry[] {
  const entries = collectContentEntries(path.resolve(contentDir, DEFAULT_LOCALE));
  entries.sort((a, b) =>
    a.route === "/" ? -1 : b.route === "/" ? 1 : a.route.localeCompare(b.route),
  );
  return entries;
}

export default function metaPlugin() {
  let contentDir: string;

  return {
    name: "unifast-meta",
    configResolved(config: { root: string }) {
      contentDir = path.resolve(config.root, "content");
    },
    configureServer(server: { middlewares: { use: (fn: Function) => void } }) {
      const handlers: Record<string, () => { content: string; type: string }> = {
        "/sitemap.xml": () => ({ content: buildSitemap(contentDir), type: "application/xml" }),
        "/llms.txt": () => ({ content: buildLlmsTxt(contentDir), type: "text/plain" }),
        "/llms-full.txt": () => ({ content: buildLlmsFullTxt(contentDir), type: "text/plain" }),
      };

      server.middlewares.use((req: any, res: any, next: any) => {
        const handler = handlers[req.url];
        if (!handler) return next();
        const { content, type } = handler();
        res.setHeader("Content-Type", `${type}; charset=utf-8`);
        res.end(content);
      });
    },
  };
}
