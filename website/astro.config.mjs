import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import unifastPlugin from "@unifast/vite";
import { compile, frontmatter, treeSitter, externalLinks, autolinkHeadings, githubAlert, emoji, smartypants, breaks, cjk, directive, definitionList, rubyAnnotation, wikiLink, sectionize, math, gfm } from "@unifast/node";
import packageInstallHighlightPlugin from "./plugins/vite-plugin-package-install-highlight";
import examplePlugin from "./plugins/vite-plugin-example";
import translationStatusPlugin from "./plugins/vite-plugin-translation-status";
import metaPlugin from "./plugins/vite-plugin-meta";

const compileOptions = {
  toc: { enabled: true, maxDepth: 3 },
  lineNumbers: { enabled: true },
  plugins: [
    frontmatter(),
    treeSitter(),
    externalLinks({ target: "_blank" }),
    autolinkHeadings({ behavior: "prepend" }),
    githubAlert(),
    emoji(),
    smartypants(),
    breaks(),
    cjk(),
  ],
};

const exampleCompileOptions = {
  ...compileOptions,
  plugins: [
    ...compileOptions.plugins,
    directive(),
    definitionList(),
    rubyAnnotation(),
    wikiLink(),
    sectionize(),
    math(),
    gfm(),
  ],
};

export default defineConfig({
  site: "https://unifast.dev",
  base: "/",
  output: "static",
  integrations: [
    react({
      include: ["**/components/**", "**/hooks/**"],
    }),
  ],
  vite: {
    plugins: [
      unifastPlugin({ md: compileOptions, mdx: compileOptions }),
      packageInstallHighlightPlugin({ compile, plugins: [treeSitter()] }),
      examplePlugin({ compile, compileOptions: exampleCompileOptions }),
      translationStatusPlugin(),
      metaPlugin(),
    ],
    resolve: {
      alias: { "~": new URL("./src", import.meta.url).pathname },
    },
    ssr: {
      external: ["@unifast/node"],
    },
    css: {
      transformer: "lightningcss",
      lightningcss: {
        targets: { chrome: 110 << 16, firefox: 115 << 16, safari: 16 << 16 },
      },
    },
  },
});
