---
title: "hastToHtml()"
description: "Serialisiert einen HAST-Wurzelknoten in einen HTML-String."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Signatur

```ts
function hastToHtml(hast: HastRoot): string
```

## Parameter

### hast

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Bezeichner des Knotentyps |
| `children` | `HastNode[]` | — | Kindknoten des Baums |

## Rückgabewert

`string` – Der serialisierte HTML-String.

## Verwendung

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

## Beispiele

### Grundlegende Serialisierung

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

### Void-Elemente

Void-Elemente (`<br>`, `<img>`, `<hr>` usw.) werden automatisch selbstschließend ausgegeben:

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

### Mit compile()-Ausgabe

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Roh-HTML durchreichen

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

## Verhalten

- **HTML-Maskierung:** Textinhalte werden maskiert (`&`, `<`, `>`, `"`)
- **Void-Elemente:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` werden selbstschließend ausgegeben
- **Attribute:** Alphabetisch sortiert; `className`-Arrays werden mit Leerzeichen verbunden und als `class` gerendert; der boolesche Wert `true` wird als bloßes Attribut gerendert; `false`/`null`/`undefined` werden ausgelassen
- **Kommentare:** Werden als `<!--value-->` gerendert
- **Doctype:** Wird als `<!DOCTYPE html>` gerendert
- **Raw-Knoten:** Werden unverändert und ohne Maskierung ausgegeben
