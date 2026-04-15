---
title: "hastToHtml()"
description: "Men-serialize sebuah node root HAST menjadi string HTML."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Signature

```ts
function hastToHtml(hast: HastRoot): string
```

## Parameter

### hast

| Properti | Tipe | Default | Deskripsi |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Identifier tipe node |
| `children` | `HastNode[]` | — | Node anak dari pohon |

## Return

`string` — String HTML hasil serialisasi.

## Penggunaan

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "h1",
      properties: { id: "hello", className: ["title", "main"] },
      children: [
        { type: "text", value: "Hello " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "world" }],
        },
      ],
    },
  ],
};

const html = hastToHtml(hast);

console.log(html);
// <h1 class="title main" id="hello">Hello <strong>world</strong></h1>
```

## Contoh

### Serialisasi dasar

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "This is " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "bold" }],
        },
        { type: "text", value: " text." },
      ],
    },
  ],
};

console.log(hastToHtml(hast));
// <p>This is <strong>bold</strong> text.</p>
```

### Void element

Void element (`<br>`, `<img>`, `<hr>`, dll.) ditutup secara otomatis (self-closed):

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "img",
      properties: { src: "photo.jpg", alt: "A photo" },
      children: [],
    },
  ],
};

console.log(hastToHtml(hast));
// <img alt="A photo" src="photo.jpg" />
```

### Dengan output compile()

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Passthrough HTML mentah

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    { type: "raw", value: "<div class=\"custom\">Raw HTML</div>" },
  ],
};

console.log(hastToHtml(hast));
// <div class="custom">Raw HTML</div>
```

## Perilaku

- **Escape HTML:** Konten teks di-escape (`&`, `<`, `>`, `"`)
- **Void element:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` ditutup sendiri (self-closed)
- **Atribut:** Diurutkan secara alfabetis; array `className` digabungkan dengan spasi dan dirender sebagai `class`; boolean `true` dirender sebagai atribut tanpa nilai; `false`/`null`/`undefined` dihilangkan
- **Komentar:** Dirender sebagai `<!--value-->`
- **Doctype:** Dirender sebagai `<!DOCTYPE html>`
- **Node mentah:** Dikeluarkan apa adanya tanpa di-escape
