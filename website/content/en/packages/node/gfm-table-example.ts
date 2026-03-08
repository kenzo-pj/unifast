import md from "./gfm-table-example.md?raw";
import { compile, gfm } from "@unifast/node";

const result = compile(md, {
  plugins: [gfm()],
});
