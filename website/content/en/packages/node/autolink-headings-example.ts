import md from "./autolink-headings-example.md?raw";
import { compile, autolinkHeadings } from "@unifast/node";

const result = compile(md, {
  plugins: [
    autolinkHeadings({
      behavior: "prepend",
    }),
  ],
});
