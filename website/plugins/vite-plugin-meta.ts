import fs from "node:fs";
import path from "node:path";

export const LOCALES = ["en", "ja"] as const;
export const DEFAULT_LOCALE = "en";
export const SITE_URL = "https://unifast.dev";

const SITE_NAME = "unifast";
const SITE_DESCRIPTION = "High-performance Markdown / MDX compiler built with Rust.";
const GITHUB_URL = "https://github.com/kenzo-pj/unifast";

function parseFrontmatter(raw: string): { title: string; description: string; body: string } {
  const match = raw.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/);
  if (!match) return { title: "", description: "", body: raw };
  const titleMatch = match[1].match(/title:\s*["']?(.+?)["']?\s*$/m);
  const descMatch = match[1].match(/description:\s*["']?(.+?)["']?\s*$/m);
  return { title: titleMatch?.[1] ?? "", description: descMatch?.[1] ?? "", body: match[2] };
}

function stripMdxSyntax(body: string): string {
  return body
    .replace(/^import\s.+$/gm, "")
    .replace(/<[A-Z][\w]*\b[^>]*\/>/g, "")
    .replace(/<[A-Z][\w]*\b[^>]*>[\s\S]*?<\/[A-Z][\w]*>/g, "")
    .replace(/\n{3,}/g, "\n\n")
    .trim();
}

export interface ContentEntry {
  route: string;
  title: string;
  description: string;
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
      const { title, description, body } = parseFrontmatter(raw);
      const cleanBody = entry.name.endsWith(".mdx") ? stripMdxSyntax(body) : body;
      entries.push({ route, title, description, body: cleanBody });
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

export function buildSitemapXsl(): string {
  return `<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="2.0"
  xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
  xmlns:sitemap="http://www.sitemaps.org/schemas/sitemap/0.9"
  xmlns:xhtml="http://www.w3.org/1999/xhtml">
  <xsl:output method="html" encoding="UTF-8" indent="yes"/>
  <xsl:template match="/">
    <html lang="en">
      <head>
        <title>Sitemap — unifast</title>
        <style>
          *{margin:0;padding:0;box-sizing:border-box}
          body{font-family:system-ui,-apple-system,sans-serif;color:#1a1a1a;background:#fff;padding:2rem}
          h1{font-size:1.5rem;font-weight:600;margin-bottom:.25rem}
          p.desc{color:#666;font-size:.875rem;margin-bottom:1.5rem}
          table{width:100%;border-collapse:collapse;font-size:.875rem}
          th{text-align:left;padding:.5rem .75rem;border-bottom:2px solid #e5e5e5;color:#666;font-weight:500}
          td{padding:.5rem .75rem;border-bottom:1px solid #f0f0f0}
          tr:hover td{background:#fafafa}
          a{color:#2563eb;text-decoration:none}
          a:hover{text-decoration:underline}
          .lang{display:inline-block;padding:.125rem .375rem;border-radius:.25rem;background:#f0f0f0;color:#666;font-size:.75rem;margin-right:.25rem}
          @media(prefers-color-scheme:dark){
            body{background:#171717;color:#e5e5e5}
            th{border-bottom-color:#333;color:#999}
            td{border-bottom-color:#262626}
            tr:hover td{background:#1e1e1e}
            a{color:#60a5fa}
            .lang{background:#262626;color:#999}
            p.desc{color:#999}
          }
        </style>
      </head>
      <body>
        <h1>Sitemap</h1>
        <p class="desc"><xsl:value-of select="count(sitemap:urlset/sitemap:url)"/> URLs</p>
        <table>
          <thead>
            <tr><th>URL</th><th>Languages</th><th>Last Modified</th></tr>
          </thead>
          <tbody>
            <xsl:for-each select="sitemap:urlset/sitemap:url">
              <tr>
                <td><a href="{sitemap:loc}"><xsl:value-of select="sitemap:loc"/></a></td>
                <td>
                  <xsl:for-each select="xhtml:link[@rel='alternate' and @hreflang!='x-default']">
                    <span class="lang"><xsl:value-of select="@hreflang"/></span>
                  </xsl:for-each>
                </td>
                <td><xsl:value-of select="sitemap:lastmod"/></td>
              </tr>
            </xsl:for-each>
          </tbody>
        </table>
      </body>
    </html>
  </xsl:template>
</xsl:stylesheet>
`;
}

export function buildSitemap(contentDir: string): string {
  const today = new Date().toISOString().split("T")[0];
  const enRoutes = collectRoutes(path.resolve(contentDir, "en"));
  const jaRoutes = new Set(collectRoutes(path.resolve(contentDir, "ja")));

  const urls: string[] = [];
  for (const route of enRoutes) {
    const enUrl = `${SITE_URL}${route === "/" ? "" : route}`;
    const jaUrl = `${SITE_URL}/ja${route === "/" ? "" : route}`;
    const hasJa = jaRoutes.has(route);

    let entry = `  <url>\n    <loc>${enUrl}</loc>\n    <lastmod>${today}</lastmod>`;
    entry += `\n    <xhtml:link rel="alternate" hreflang="en" href="${enUrl}" />`;
    if (hasJa) entry += `\n    <xhtml:link rel="alternate" hreflang="ja" href="${jaUrl}" />`;
    entry += `\n    <xhtml:link rel="alternate" hreflang="x-default" href="${enUrl}" />`;
    entry += `\n  </url>`;
    urls.push(entry);

    if (hasJa) {
      let jaEntry = `  <url>\n    <loc>${jaUrl}</loc>\n    <lastmod>${today}</lastmod>`;
      jaEntry += `\n    <xhtml:link rel="alternate" hreflang="en" href="${enUrl}" />`;
      jaEntry += `\n    <xhtml:link rel="alternate" hreflang="ja" href="${jaUrl}" />`;
      jaEntry += `\n    <xhtml:link rel="alternate" hreflang="x-default" href="${enUrl}" />`;
      jaEntry += `\n  </url>`;
      urls.push(jaEntry);
    }
  }

  return [
    `<?xml version="1.0" encoding="UTF-8"?>`,
    `<?xml-stylesheet type="text/xsl" href="sitemap.xsl"?>`,
    `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">`,
    urls.join("\n"),
    `</urlset>`,
    "",
  ].join("\n");
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

function extractDescription(body: string): string {
  const lines = body.split("\n");
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    if (trimmed.startsWith("#") || trimmed.startsWith("|") || trimmed.startsWith("```") || trimmed.startsWith("import ") || trimmed.startsWith("<")) continue;
    const plain = trimmed.replace(/\[([^\]]+)\]\([^)]+\)/g, "$1").replace(/[*_`]/g, "");
    if (plain.length > 20) return plain.length > 160 ? plain.slice(0, 157) + "..." : plain;
  }
  return SITE_DESCRIPTION;
}

function escapeHtml(str: string): string {
  return str.replace(/&/g, "&amp;").replace(/"/g, "&quot;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function buildBreadcrumb(route: string, title: string): string {
  const items: { name: string; url: string }[] = [{ name: "Home", url: SITE_URL }];
  if (route === "/") return "";

  const parts = route.replace(/^\/docs\//, "").split("/");
  let href = "/docs";
  for (let i = 0; i < parts.length - 1; i++) {
    href += `/${parts[i]}`;
    const name = parts[i].replace(/-/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
    items.push({ name, url: `${SITE_URL}${href}` });
  }
  items.push({ name: title, url: `${SITE_URL}${route}` });

  return JSON.stringify({
    "@context": "https://schema.org",
    "@type": "BreadcrumbList",
    "itemListElement": items.map((item, i) => ({
      "@type": "ListItem",
      "position": i + 1,
      "name": item.name,
      "item": item.url,
    })),
  });
}

export interface PageMeta {
  route: string;
  locale: string;
  title: string;
  description: string;
  noindex?: boolean;
}

export function buildHeadMeta(meta: PageMeta): string {
  const isHome = /^\/(ja\/?)?$/.test(meta.route);
  const pageTitle = isHome ? `${SITE_NAME} - ${SITE_DESCRIPTION}` : `${meta.title} | ${SITE_NAME}`;
  const desc = escapeHtml(meta.description || SITE_DESCRIPTION);
  const canonicalUrl = `${SITE_URL}${meta.route === "/" ? "" : meta.route}`;
  const locale = meta.locale;
  const altLocale = locale === "en" ? "ja" : "en";
  const altRoute = locale === "en"
    ? (meta.route === "/" ? "/ja" : `/ja${meta.route}`)
    : meta.route.replace(/^\/ja/, "") || "/";
  const ogLocale = locale === "en" ? "en_US" : "ja_JP";
  const ogType = isHome ? "website" : "article";

  const tags: string[] = [
    `<title>${escapeHtml(pageTitle)}</title>`,
    meta.noindex
      ? `<meta name="robots" content="noindex, nofollow" />`
      : `<meta name="robots" content="index, follow" />`,
    `<meta name="description" content="${desc}" />`,
    `<link rel="canonical" href="${canonicalUrl}" />`,
    `<link rel="alternate" hreflang="${locale}" href="${canonicalUrl}" />`,
    `<link rel="alternate" hreflang="${altLocale}" href="${SITE_URL}${altRoute === "/" ? "" : altRoute}" />`,
    `<link rel="alternate" hreflang="x-default" href="${SITE_URL}${locale === "en" ? (meta.route === "/" ? "" : meta.route) : altRoute === "/" ? "" : altRoute}" />`,
    `<meta property="og:title" content="${escapeHtml(pageTitle)}" />`,
    `<meta property="og:description" content="${desc}" />`,
    `<meta property="og:url" content="${canonicalUrl}" />`,
    `<meta property="og:site_name" content="${SITE_NAME}" />`,
    `<meta property="og:type" content="${ogType}" />`,
    `<meta property="og:locale" content="${ogLocale}" />`,
    `<meta property="og:locale:alternate" content="${altLocale === "en" ? "en_US" : "ja_JP"}" />`,
    `<meta name="twitter:card" content="summary" />`,
    `<meta name="twitter:title" content="${escapeHtml(pageTitle)}" />`,
    `<meta name="twitter:description" content="${desc}" />`,
  ];

  if (isHome) {
    tags.push(`<script type="application/ld+json">${JSON.stringify({
      "@context": "https://schema.org",
      "@type": "WebSite",
      "name": SITE_NAME,
      "url": SITE_URL,
      "description": SITE_DESCRIPTION,
      "inLanguage": [locale, altLocale],
      "potentialAction": {
        "@type": "SearchAction",
        "target": { "@type": "EntryPoint", "urlTemplate": `${SITE_URL}/?q={search_term_string}` },
        "query-input": "required name=search_term_string",
      },
    })}</script>`);

    tags.push(`<script type="application/ld+json">${JSON.stringify({
      "@context": "https://schema.org",
      "@type": "SoftwareSourceCode",
      "name": SITE_NAME,
      "description": SITE_DESCRIPTION,
      "url": SITE_URL,
      "codeRepository": GITHUB_URL,
      "programmingLanguage": ["Rust", "TypeScript"],
      "runtimePlatform": "Node.js",
      "license": "https://opensource.org/licenses/MIT",
    })}</script>`);
  }

  if (!isHome && meta.title) {
    tags.push(`<script type="application/ld+json">${JSON.stringify({
      "@context": "https://schema.org",
      "@type": "TechArticle",
      "headline": meta.title,
      "description": meta.description || SITE_DESCRIPTION,
      "url": canonicalUrl,
      "inLanguage": locale,
      "isPartOf": { "@type": "WebSite", "name": SITE_NAME, "url": SITE_URL },
      "publisher": { "@type": "Organization", "name": SITE_NAME, "url": SITE_URL },
    })}</script>`);

    const breadcrumb = buildBreadcrumb(meta.route, meta.title);
    if (breadcrumb) {
      tags.push(`<script type="application/ld+json">${breadcrumb}</script>`);
    }
  }

  return tags.join("\n    ");
}

export function collectAllEntries(contentDir: string): Map<string, ContentEntry> {
  const map = new Map<string, ContentEntry>();
  for (const locale of LOCALES) {
    const entries = collectContentEntries(path.resolve(contentDir, locale));
    for (const entry of entries) {
      const route = locale === DEFAULT_LOCALE ? entry.route : `/${locale}${entry.route}`;
      if (!entry.description) {
        entry.description = extractDescription(entry.body);
      }
      map.set(route, entry);
    }
  }
  return map;
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
        "/sitemap.xsl": () => ({ content: buildSitemapXsl(), type: "text/xsl" }),
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
