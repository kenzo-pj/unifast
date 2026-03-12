# @unifast/highlight

[highlight.js](https://highlightjs.org/) (via [lowlight](https://github.com/wooorm/lowlight)) syntax highlighting plugin for [unifast](https://kenzo-pj.github.io/unifast/).

## Install

```sh
npm install @unifast/highlight @unifast/node
```

## Usage

```ts
import { compile } from "@unifast/node";
import { highlight } from "@unifast/highlight";

const result = compile(markdown, {
  plugins: [highlight()],
});
```

Code blocks with a language class (e.g., ` ```js `) are automatically highlighted. Unknown languages are left untouched.

## API

### `highlight()`

Create a unifast plugin for lowlight-based syntax highlighting. Returns a `UnifastPlugin`.

The plugin:
- Disables unifast's built-in highlighting
- Transforms `<pre><code>` blocks in the HAST output
- Supports all languages included in highlight.js

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
