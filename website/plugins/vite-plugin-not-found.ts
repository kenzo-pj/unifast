import fs from "node:fs";
import path from "node:path";
import type { Plugin, ResolvedConfig, ViteDevServer, PreviewServer } from "vite";
import type { IncomingMessage, ServerResponse } from "node:http";

const LOCALES = ["en", "ja"];
const DEFAULT_LOCALE = "en";

function collectRoutes(contentDir: string): Set<string> {
  const routes = new Set<string>();

  for (const locale of LOCALES) {
    const localeDir = path.join(contentDir, locale);
    if (!fs.existsSync(localeDir)) continue;
    walkDir(localeDir, "", (slug) => {
      if (locale === DEFAULT_LOCALE) {
        routes.add(slug === "index" ? "/" : `/docs/${slug}`);
      } else {
        routes.add(slug === "index" ? `/${locale}` : `/${locale}/docs/${slug}`);
      }
    });
  }

  return routes;
}

function walkDir(dir: string, prefix: string, cb: (slug: string) => void) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      walkDir(path.join(dir, entry.name), `${prefix}${entry.name}/`, cb);
    } else if (entry.name.endsWith(".md") || entry.name.endsWith(".mdx")) {
      const slug = entry.name.replace(/\.(md|mdx)$/, "");
      cb(prefix ? `${prefix}${slug}` : slug);
    }
  }
}

export default function notFoundPlugin(): Plugin {
  let knownRoutes: Set<string>;
  let contentDir: string;

  return {
    name: "vite-plugin-not-found",
    configResolved(config: ResolvedConfig) {
      contentDir = path.resolve(config.root, "content");
      knownRoutes = collectRoutes(contentDir);
    },
    configureServer(server: ViteDevServer) {
      server.middlewares.use((req: IncomingMessage, _res: ServerResponse, next: () => void) => {
        knownRoutes = collectRoutes(contentDir);
        const url = req.url?.split("?")[0]?.replace(/\/$/, "") || "/";
        const normalized = url === "" ? "/" : url;

        if (
          req.headers.accept?.includes("text/html") &&
          !knownRoutes.has(normalized)
        ) {
          const origWriteHead = _res.writeHead.bind(_res);
          _res.writeHead = function (_statusCode: number, ...args: any[]) {
            return origWriteHead(404, ...args);
          } as typeof _res.writeHead;
        }

        next();
      });
    },
    configurePreviewServer(server: PreviewServer) {
      server.middlewares.use((req: IncomingMessage, _res: ServerResponse, next: () => void) => {
        const url = req.url?.split("?")[0]?.replace(/\/$/, "") || "/";
        const normalized = url === "" ? "/" : url;

        if (
          req.headers.accept?.includes("text/html") &&
          !knownRoutes.has(normalized)
        ) {
          const origWriteHead = _res.writeHead.bind(_res);
          _res.writeHead = function (_statusCode: number, ...args: any[]) {
            return origWriteHead(404, ...args);
          } as typeof _res.writeHead;
        }

        next();
      });
    },
  };
}
