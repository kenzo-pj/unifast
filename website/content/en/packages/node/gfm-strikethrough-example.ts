import md from "./gfm-strikethrough-example.md?raw";
import { compile, gfm } from "@unifast/node";

const result = compile(md, {
  plugins: [gfm()],
});
