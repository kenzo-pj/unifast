---
title: "accessibleEmoji()"
description: 'Envolve caracteres emoji em elementos <span role="img"> com atributos aria-label para acessibilidade.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Assinatura

```ts
function accessibleEmoji(): UnifastPlugin
```

## Parâmetros

Nenhum.

## Uso

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## Exemplos

### Envolvendo emojis com aria labels

Cada caractere emoji é envolvido em um `<span>` com `role="img"` e um `aria-label` descrevendo o emoji para leitores de tela:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Texto sem emoji

Texto puro sem caracteres emoji passa inalterado:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
