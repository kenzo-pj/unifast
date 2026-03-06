import fs from "node:fs";
import path from "node:path";
import url from "node:url";
import { minify } from "html-minifier-terser";
import { buildSitemap, buildSitemapXsl, buildLlmsTxt, buildLlmsFullTxt, collectAllEntries, buildHeadMeta } from "../plugins/vite-plugin-meta";

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const distClient = path.resolve(root, "dist/client");
const contentDir = path.resolve(root, "content");

const LOCALES = ["en", "ja"] as const;
const DEFAULT_LOCALE = "en";
const BASE_PATH = process.env.CI ? "/unifast" : "";

async function minifyHtml(html: string): Promise<string> {
  return minify(html, {
    collapseWhitespace: true,
    removeComments: true,
    removeRedundantAttributes: true,
    removeEmptyAttributes: true,
    minifyCSS: true,
    minifyJS: true,
  });
}

function injectModulePreloads(html: string): string {
  const assetDir = path.resolve(distClient, "assets");
  if (!fs.existsSync(assetDir)) return html;

  const vendorChunks = fs.readdirSync(assetDir)
    .filter((f) => f.startsWith("vendor-") && f.endsWith(".js"))
    .map((f) => `${BASE_PATH}/assets/${f}`);

  if (vendorChunks.length === 0) return html;

  const preloadTags = vendorChunks
    .map((href) => `<link rel="modulepreload" href="${href}">`)
    .join("\n    ");

  return html.replace("</head>", `    ${preloadTags}\n  </head>`);
}

async function collectRoutes(dir: string, prefix = ""): Promise<string[]> {
  const routes: string[] = [];
  if (!fs.existsSync(dir)) return routes;
  const entries = fs.readdirSync(dir, { withFileTypes: true });

  for (const entry of entries) {
    if (entry.isDirectory()) {
      const sub = await collectRoutes(
        path.join(dir, entry.name),
        `${prefix}/${entry.name}`,
      );
      routes.push(...sub);
    } else if (entry.name.endsWith(".md") || entry.name.endsWith(".mdx")) {
      const slug = entry.name.replace(/\.(md|mdx)$/, "");
      if (slug === "index" && prefix === "") {
        routes.push("/");
      } else if (slug === "index") {
        routes.push(`/docs${prefix}/`);
      } else {
        routes.push(`/docs${prefix}/${slug}/`);
      }
    }
  }

  return routes;
}

async function prerender() {
  const { render } = await import(
    path.resolve(root, "dist/server/entry-server.js")
  );
  let template = fs.readFileSync(
    path.resolve(distClient, "index.html"),
    "utf-8",
  );

  template = injectModulePreloads(template);
  template = template.replace('href="/sitemap.xml"', `href="${BASE_PATH}/sitemap.xml"`);

  const allRoutes: string[] = [];

  for (const locale of LOCALES) {
    const localeContentDir = path.resolve(contentDir, locale);
    const baseRoutes = await collectRoutes(localeContentDir);

    for (const route of baseRoutes) {
      if (locale === DEFAULT_LOCALE) {
        allRoutes.push(route);
      } else {
        allRoutes.push(`/${locale}${route}`);
      }
    }
  }

  const contentEntries = collectAllEntries(contentDir);

  console.log(`Prerendering ${allRoutes.length} routes...`);

  for (const route of allRoutes) {
    const result = await render(`${BASE_PATH}${route}`);
    const lang = route.startsWith("/ja") ? "ja" : "en";
    const entry = contentEntries.get(route);
    const headMeta = buildHeadMeta({
      route,
      locale: lang,
      title: entry?.title ?? "unifast",
      description: entry?.description ?? "",
    });
    const raw = template
      .replace('<html lang="en">', `<html lang="${lang}">`)
      .replace("<title>unifast</title>", "")
      .replace("<!--head-meta-->", headMeta)
      .replace(
        '<div id="root"></div>',
        `<div id="root">${result.html}</div>`,
      );
    const html = await minifyHtml(raw);

    const filePath =
      route === "/"
        ? path.resolve(distClient, "index.html")
        : path.resolve(distClient, `${route.slice(1)}/index.html`);

    fs.mkdirSync(path.dirname(filePath), { recursive: true });
    fs.writeFileSync(filePath, html);
    console.log(`  ${route} -> ${path.relative(root, filePath)}`);
  }

  const notFoundResult = await render(`${BASE_PATH}/this-page-does-not-exist`);
  const notFoundMeta = buildHeadMeta({
    route: "/404",
    locale: "en",
    title: "Page Not Found",
    description: "The page you are looking for does not exist.",
    noindex: true,
  });
  const notFoundRaw = template
    .replace("<title>unifast</title>", "")
    .replace("<!--head-meta-->", notFoundMeta)
    .replace(
      '<div id="root"></div>',
      `<div id="root">${notFoundResult.html}</div>`,
    );
  const notFoundHtml = await minifyHtml(notFoundRaw);
  const notFoundPath = path.resolve(distClient, "404.html");
  fs.writeFileSync(notFoundPath, notFoundHtml);
  console.log(`  404 -> ${path.relative(root, notFoundPath)}`);

  fs.writeFileSync(path.resolve(distClient, "sitemap.xml"), buildSitemap(contentDir));
  fs.writeFileSync(path.resolve(distClient, "sitemap.xsl"), buildSitemapXsl());
  console.log("  Generated sitemap.xml + sitemap.xsl");
  fs.writeFileSync(path.resolve(distClient, "llms.txt"), buildLlmsTxt(contentDir));
  console.log("  Generated llms.txt");
  fs.writeFileSync(path.resolve(distClient, "llms-full.txt"), buildLlmsFullTxt(contentDir));
  console.log("  Generated llms-full.txt");

  console.log("Done.");
}

prerender().catch((err) => {
  console.error(err);
  process.exit(1);
});
