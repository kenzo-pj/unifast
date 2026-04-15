---
title: "customHeadingId()"
description: "Menetapkan ID kustom pada heading menggunakan sintaks {#custom-id}."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Signature

```ts
function customHeadingId(): UnifastPlugin
```

## Parameter

Tidak ada.

## Penggunaan

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## Contoh

### ID kustom

Gunakan sintaks `{#custom-id}` di akhir heading untuk menetapkan atribut `id` yang spesifik. Blok kurung kurawal dihapus dari teks yang dirender:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Heading tanpa ID kustom

Heading tanpa sintaks `{#...}` dibiarkan tidak berubah. Mereka menggunakan slug default yang dihasilkan dari teks heading:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Class dan atribut sembarang

Sintaks kurung kurawal juga mendukung notasi `.class` dan `key=value`:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
