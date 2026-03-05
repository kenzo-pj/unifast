import path from "node:path";
import url from "node:url";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { tanstackRouter } from "@tanstack/router-plugin/vite";
import unifastPlugin from "@unifast/plugin-vite";
import { syntect } from "@unifast/plugin-syntect";
import translationStatusPlugin from "./plugins/vite-plugin-translation-status";
import notFoundPlugin from "./plugins/vite-plugin-not-found";
import metaPlugin from "./plugins/vite-plugin-meta";
import packageInstallHighlightPlugin from "./plugins/vite-plugin-package-install-highlight";

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));

function pagefindDevPlugin() {
  return {
    name: "pagefind-dev-stub",
    enforce: "pre" as const,
    resolveId(id: string) {
      if (id === "/pagefind/pagefind.js") return "\0pagefind-stub";
    },
    load(id: string) {
      if (id === "\0pagefind-stub") {
        return "export async function init() {} export async function search() { return { results: [] }; }";
      }
    },
  };
}

const compileOptions = {
  toc: { enabled: true, maxDepth: 3 },
  lineNumbers: { enabled: true },
  plugins: [syntect()],
};

export default defineConfig(({ isSsrBuild }) => ({
  plugins: [
    pagefindDevPlugin(),
    metaPlugin(),
    notFoundPlugin(),
    tanstackRouter({ target: "react", autoCodeSplitting: true }),
    unifastPlugin({
      md: compileOptions,
      mdx: compileOptions,
    }),
    packageInstallHighlightPlugin(),
    translationStatusPlugin(),
    react({
      babel: {
        plugins: [["babel-plugin-react-compiler", {}]],
      },
    }),
  ],
  resolve: {
    alias: { "~": path.resolve(__dirname, "src") },
  },
  build: isSsrBuild
    ? {
        ssr: true,
        outDir: "dist/server",
        copyPublicDir: false,
        rollupOptions: {
          input: path.resolve(__dirname, "src/entry-server.tsx"),
          output: { entryFileNames: "[name].js" },
          external: ["/pagefind/pagefind.js"],
        },
      }
    : {
        outDir: "dist/client",
        rollupOptions: {
          input: path.resolve(__dirname, "index.html"),
          external: ["/pagefind/pagefind.js"],
          output: {
            manualChunks(id) {
              if (id.includes("/node_modules/react-dom/") || id.includes("/node_modules/react/")) {
                return "vendor-react";
              }
              if (id.includes("/node_modules/@tanstack/react-router/")) {
                return "vendor-router";
              }
            },
          },
        },
      },
}));
