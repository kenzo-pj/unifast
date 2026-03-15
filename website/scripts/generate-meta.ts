import fs from "node:fs";
import path from "node:path";
import { buildSitemap, buildSitemapXsl, buildLlmsTxt, buildLlmsFullTxt } from "../plugins/vite-plugin-meta";

const contentDir = path.resolve(import.meta.dirname, "../content");
const distDir = path.resolve(import.meta.dirname, "../dist");

fs.writeFileSync(path.join(distDir, "sitemap.xml"), buildSitemap(contentDir));
fs.writeFileSync(path.join(distDir, "sitemap.xsl"), buildSitemapXsl());
fs.writeFileSync(path.join(distDir, "llms.txt"), buildLlmsTxt(contentDir));
fs.writeFileSync(path.join(distDir, "llms-full.txt"), buildLlmsFullTxt(contentDir));

console.log("[generate-meta] sitemap.xml, sitemap.xsl, llms.txt, llms-full.txt written to dist/");
