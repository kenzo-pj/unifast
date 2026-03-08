import md from "./external-links-example.md?raw";
import { compile, externalLinks } from "@unifast/node";

const result = compile(md, {
  plugins: [
    externalLinks({
      rel: "nofollow noopener noreferrer",
      target: "_blank",
    }),
  ],
});
