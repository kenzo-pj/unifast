---
title: "accessibleEmoji()"
description: 'Envuelve los caracteres emoji en elementos <span role="img"> con atributos aria-label para accesibilidad.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Firma

```ts
function accessibleEmoji(): UnifastPlugin
```

## Parámetros

Ninguno.

## Uso

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## Ejemplos

### Envolvido de emoji con etiquetas aria

Cada carácter emoji se envuelve en un `<span>` con `role="img"` y un `aria-label` que describe el emoji para los lectores de pantalla:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Texto sin emoji

El texto plano sin caracteres emoji pasa sin cambios:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
