---
title: "cjk()"
description: "CJK-friendly text processing that removes unwanted newlines between CJK characters."
---

```ts
import { cjk } from "@unifast/node";
```

### Signature

```ts
function cjk(): UnifastPlugin
```

### Parameters

None.

### Returns

`UnifastPlugin`

## Usage

### Basic usage

In standard Markdown, a single newline between lines is treated as a space. This works well for English, but creates unwanted spaces between CJK characters. The `cjk()` plugin removes these extra spaces.

```ts
import { compile, cjk } from "@unifast/node";

const md = `
今日は天気が
とても良いです。
`;

const result = compile(md, { plugins: [cjk()] });
```

**Without** the plugin, the output contains a space:

```html
<p>今日は天気が とても良いです。</p>
```

**With** the plugin, the unwanted space is removed:

```html
<p>今日は天気がとても良いです。</p>
```

### Chinese text

```ts
import { compile, cjk } from "@unifast/node";

const md = `
这是一段
中文文本。
编写较长的段落时，
换行不应引入空格。
`;

const result = compile(md, { plugins: [cjk()] });
// Output: <p>这是一段中文文本。编写较长的段落时，换行不应引入空格。</p>
```

### Mixed CJK and Latin text

```ts
import { compile, cjk } from "@unifast/node";

const md = `
unifastは高速な
Markdownコンパイラです。
Rustで書かれています。
`;

const result = compile(md, { plugins: [cjk()] });
// Newlines between CJK characters are removed,
// while spaces between Latin words are preserved.
```

### Combined with other plugins

```ts
import { compile, cjk, gfm, frontmatter } from "@unifast/node";

const result = compile(md, {
  plugins: [cjk(), gfm(), frontmatter()],
});
```
