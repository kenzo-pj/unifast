import md from "./smartypants-example.md?raw";
import { compile, smartypants } from "@unifast/node";

const result = compile(md, {
  plugins: [
    smartypants({
      quotes: true,
      dashes: true,
      ellipses: true,
    }),
  ],
});
