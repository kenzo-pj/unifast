import md from "./cjk-example.md?raw";
import { compile, cjk } from "@unifast/node";

const result = compile(md, {
  plugins: [cjk()],
});
