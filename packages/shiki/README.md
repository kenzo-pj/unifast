# @unifast/shiki

[Shiki](https://shiki.style/) syntax highlighting plugin for [unifast](https://kenzo-pj.github.io/unifast/).

Transforms code blocks in the HAST output using Shiki's TextMate grammar-based highlighter with support for dual themes (light/dark).

## Install

```sh
npm install @unifast/shiki @unifast/node
```

## Usage

```ts
import { compile } from "@unifast/node";
import { createShikiPlugin } from "@unifast/shiki";

const shiki = await createShikiPlugin({
  themes: {
    light: "github-light",
    dark: "github-dark",
  },
});

const result = compile(markdown, {
  plugins: [shiki],
});
```

## API

### `createShikiPlugin(options?)`

Create a unifast plugin for Shiki syntax highlighting. Returns a `Promise<UnifastPlugin>`.

**Options:**

| Option         | Type                                                | Description                          |
| -------------- | --------------------------------------------------- | ------------------------------------ |
| `themes`       | `BundledTheme \| BundledTheme[] \| { light, dark }` | Theme(s) to use                      |
| `defaultTheme` | `BundledTheme`                                      | Fallback theme                       |
| `defaultColor` | `string \| false`                                   | Default color for highlighted tokens |
| `langs`        | `BundledLanguage[]`                                 | Languages to load                    |

### `createShikiTransformer(options?)`

Lower-level API that returns a transformer with `transform(hast)` and `transformMdxJs(js)` methods.

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
