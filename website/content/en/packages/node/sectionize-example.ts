import md from "./sectionize-example.md?raw";
import { compile, sectionize } from "@unifast/node";

const result = compile(md, {
  plugins: [sectionize()],
});
