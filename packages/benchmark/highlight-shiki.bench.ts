import { readFileSync } from "node:fs";
import { join } from "node:path";
import { bench, describe } from "vitest";
import { compile } from "@unifast/node";
import { createShikiPlugin } from "@unifast/shiki";
import { createShikiProcessor } from "./setup/unified.js";

const fixturesDir = join(import.meta.dirname, "fixtures");
const readme = readFileSync(join(fixturesDir, "readme.md"), "utf-8");

const shikiPlugin = await createShikiPlugin({
  themes: "github-dark",
  langs: ["javascript", "typescript", "rust", "bash", "json", "html", "css", "python", "yaml", "toml"],
});
const unifiedShiki = await createShikiProcessor();

describe("shiki: unifast vs unified (readme)", () => {
  bench("unifast (plugin-shiki)", () => { compile(readme, { plugins: [shikiPlugin] }); });
  bench("unified (@shikijs/rehype)", async () => { await unifiedShiki.process(readme); });
});
