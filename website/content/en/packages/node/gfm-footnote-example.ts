import md from "./gfm-footnote-example.md?raw";
import { compile, gfm } from "@unifast/node";

const result = compile(md, {
  plugins: [gfm()],
});
