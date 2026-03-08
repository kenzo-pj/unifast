import md from "./emoji-example.md?raw";
import { compile, emoji } from "@unifast/node";

const result = compile(md, {
  plugins: [emoji()],
});
