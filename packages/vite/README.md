# @unifast/vite

[Vite](https://vite.dev/) plugin for [unifast](https://kenzo-pj.github.io/unifast/) — import `.md` and `.mdx` files as JS modules.

## Install

```sh
npm install @unifast/vite @unifast/node vite
```

## Usage

```ts
// vite.config.ts
import { defineConfig } from "vite";
import unifast from "@unifast/vite";

export default defineConfig({
  plugins: [unifast()],
});
```

Then import Markdown files directly:

```ts
import post from "./content/hello.md";

console.log(post.html);         // Compiled HTML string
console.log(post.frontmatter);  // { title: "Hello", ... }
console.log(post.toc);          // [{ depth: 1, text: "...", slug: "..." }]
```

### With Options

```ts
import unifast from "@unifast/vite";
import { gfm, frontmatter } from "@unifast/node";

export default defineConfig({
  plugins: [
    unifast({
      md: {
        plugins: [gfm(), frontmatter()],
      },
    }),
  ],
});
```

## API

### `unifastPlugin(options?)`

Default export. Returns a Vite plugin.

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `md` | `CompileOptions` | Options for `.md` files |
| `mdx` | `CompileOptions` | Options for `.mdx` files |

**Module exports from imported `.md`/`.mdx` files:**

| Export | Type | Description |
|--------|------|-------------|
| `html` | `string` | Compiled HTML |
| `frontmatter` | `Record<string, unknown>` | Extracted metadata |
| `toc` | `TocEntry[]` | Table of contents |
| `default` | `{ html, frontmatter, toc }` | All exports as default |

Supports HMR — edits to Markdown files trigger hot updates.

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
