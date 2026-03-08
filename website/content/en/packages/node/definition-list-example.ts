import md from "./definition-list-example.md?raw";
import { compile, definitionList } from "@unifast/node";

const result = compile(md, {
  plugins: [definitionList()],
});
