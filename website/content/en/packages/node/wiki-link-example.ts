import md from "./wiki-link-example.md?raw";
import { compile, wikiLink } from "@unifast/node";

const result = compile(md, {
  plugins: [
    wikiLink({
      hrefTemplate: "/docs/${slug}",
    }),
  ],
});
