---
title: "accessibleEmoji()"
description: 'Racchiude i caratteri emoji in elementi <span role="img"> con attributi aria-label per l''accessibilità.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Firma

```ts
function accessibleEmoji(): UnifastPlugin
```

## Parametri

Nessuno.

## Utilizzo

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## Esempi

### Incapsulamento delle emoji con etichette aria

Ogni carattere emoji viene racchiuso in uno `<span>` con `role="img"` e un `aria-label` che descrive l'emoji per gli screen reader:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Testo senza emoji

Il testo semplice privo di caratteri emoji viene passato invariato:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
