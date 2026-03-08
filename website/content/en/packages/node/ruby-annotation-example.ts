import md from "./ruby-annotation-example.md?raw";
import { compile, rubyAnnotation } from "@unifast/node";

const result = compile(md, {
  plugins: [rubyAnnotation()],
});
