---
title: "commentRemoval()"
description: "Menghapus komentar HTML dari output."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Signature

```ts
function commentRemoval(): UnifastPlugin
```

## Parameter

Tidak ada.

## Penggunaan

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// The HTML comment is stripped from the output
```

## Contoh

### Penghapusan komentar dasar

Semua node komentar HTML (`<!-- ... -->`) dihapus dari pohon output, termasuk komentar yang bersarang di dalam elemen blok seperti blockquote:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### HTML non-komentar dipertahankan

Hanya node komentar yang dihapus. HTML inline lainnya dibiarkan tidak tersentuh:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
