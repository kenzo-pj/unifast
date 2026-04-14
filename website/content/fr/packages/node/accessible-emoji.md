---
title: "accessibleEmoji()"
description: 'Enveloppe les caractères emoji dans des éléments <span role="img"> pourvus d''attributs aria-label pour l''accessibilité.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Signature

```ts
function accessibleEmoji(): UnifastPlugin
```

## Paramètres

Aucun.

## Utilisation

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## Exemples

### Encapsulation des emojis avec des aria labels

Chaque caractère emoji est enveloppé dans un `<span>` doté de `role="img"` et d'un `aria-label` décrivant l'emoji à destination des lecteurs d'écran :

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Texte sans emoji

Le texte brut sans caractères emoji est transmis tel quel :

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
