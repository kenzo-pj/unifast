import md from "./breaks-example.md?raw";
import { compile, breaks } from "@unifast/node";

const result = compile(md, {
  plugins: [breaks()],
});
