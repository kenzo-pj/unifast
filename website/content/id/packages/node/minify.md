---
title: "minify()"
description: "Mem-minify output HTML dengan menghapus whitespace yang tidak perlu."
---

```ts
import { minify } from "@unifast/node";
```

## Signature

```ts
function minify(): UnifastPlugin
```

## Parameter

Tidak ada.

## Penggunaan

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## Contoh

### Minifikasi dasar

Plugin `minify()` menggabungkan karakter whitespace berurutan menjadi spasi tunggal, menghapus komentar HTML, menghapus text node yang hanya berisi whitespace di antara elemen blok, dan menghapus atribut `class` dan `style` yang kosong:

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### Konten preformatted dipertahankan

Whitespace di dalam blok `<pre>` dan `<code>` dibiarkan tidak tersentuh, sehingga formatting kode tidak pernah rusak:

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// Whitespace inside the <pre><code> block is preserved exactly as written
```
