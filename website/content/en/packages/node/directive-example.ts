import md from "./directive-example.md?raw";
import { compile, directive } from "@unifast/node";

const result = compile(md, {
  plugins: [directive()],
});
