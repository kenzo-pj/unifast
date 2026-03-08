import md from "./math-example.md?raw";
import { compile, math } from "@unifast/node";

const result = compile(md, {
  plugins: [math()],
});
