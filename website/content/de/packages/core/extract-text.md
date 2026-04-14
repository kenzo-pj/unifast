---
title: "extractText()"
description: "Extrahiert rekursiv den gesamten Textinhalt aus einem HAST-Knoten."
---

```ts
import { extractText } from "@unifast/core";
```

## Signatur

```ts
function extractText(node: HastNode): string
```

## Parameter

### node

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `type` | `string` | — | Der Knotentyp (`"root"`, `"element"`, `"text"` usw.) |
| `children` | `HastNode[]` | — | Kindknoten (für die Typen `"root"` und `"element"`) |
| `value` | `string` | — | Textinhalt (für den Typ `"text"`) |

## Rückgabewert

`string` – Der gesamte Textinhalt des Knotens und seiner Nachfahren, aneinandergehängt.

## Verwendung

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const element: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "Hello " },
    {
      type: "element",
      tagName: "strong",
      properties: {},
      children: [{ type: "text", value: "world" }],
    },
  ],
};

const text = extractText(element);

console.log(text);
// Hello world
```

## Beispiele

### Aus einem einfachen Element extrahieren

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h1",
  properties: { id: "title" },
  children: [{ type: "text", value: "Getting Started" }],
};

console.log(extractText(heading));
// Getting Started
```

### Aus verschachtelten Elementen extrahieren

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const paragraph: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "This is " },
    {
      type: "element",
      tagName: "em",
      properties: {},
      children: [
        { type: "text", value: "deeply " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "nested" }],
        },
      ],
    },
    { type: "text", value: " content." },
  ],
};

console.log(extractText(paragraph));
// This is deeply nested content.
```

### Leeres Element

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const emptyDiv: HastElement = {
  type: "element",
  tagName: "div",
  properties: {},
  children: [],
};

console.log(extractText(emptyDiv));
// (empty string)
```

### Slugs für Überschriften erzeugen

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h2",
  properties: {},
  children: [
    { type: "text", value: "API " },
    {
      type: "element",
      tagName: "code",
      properties: {},
      children: [{ type: "text", value: "Reference" }],
    },
  ],
};

const slug = extractText(heading).toLowerCase().replace(/\s+/g, "-");

console.log(slug);
// api-reference
```
