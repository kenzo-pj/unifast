import md from "./github-alert-example.md?raw";
import { compile, githubAlert } from "@unifast/node";

const result = compile(md, {
  plugins: [githubAlert()],
});
