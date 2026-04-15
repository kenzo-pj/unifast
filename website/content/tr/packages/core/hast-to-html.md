---
title: "hastToHtml()"
description: "Bir HAST kök düğümünü HTML string'ine serileştirir."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## İmza

```ts
function hastToHtml(hast: HastRoot): string
```

## Parametreler

### hast

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `type` | `"root"` | — | Düğüm türü tanımlayıcısı |
| `children` | `HastNode[]` | — | Ağacın alt düğümleri |

## Dönüş Değeri

`string` — Serileştirilmiş HTML string'i.

## Kullanım

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

## Örnekler

### Temel serileştirme

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

### Void elemanlar

Void elemanlar (`<br>`, `<img>`, `<hr>`, vb.) otomatik olarak kendi kendine kapatılır:

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

### compile() çıktısı ile

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Ham HTML geçişi

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

## Davranış

- **HTML kaçışı:** Metin içeriği kaçış karakterine dönüştürülür (`&`, `<`, `>`, `"`)
- **Void elemanlar:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` kendi kendine kapatılır
- **Öznitelikler:** Alfabetik olarak sıralanır; `className` dizileri boşluklarla birleştirilir ve `class` olarak render edilir; boolean `true` çıplak öznitelik olarak render edilir; `false`/`null`/`undefined` atlanır
- **Yorumlar:** `<!--value-->` olarak render edilir
- **Doctype:** `<!DOCTYPE html>` olarak render edilir
- **Raw düğümleri:** Kaçış yapılmadan olduğu gibi çıktılanır
