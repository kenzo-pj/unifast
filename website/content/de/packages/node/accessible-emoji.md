---
title: "accessibleEmoji()"
description: 'Schließt Emoji-Zeichen zur besseren Barrierefreiheit in <span role="img">-Elemente mit aria-label-Attributen ein.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Signatur

```ts
function accessibleEmoji(): UnifastPlugin
```

## Parameter

Keine.

## Verwendung

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## Beispiele

### Einschließen von Emojis mit aria-Labels

Jedes Emoji-Zeichen wird in ein `<span>` mit `role="img"` und einem `aria-label` eingeschlossen, das das Emoji für Screenreader beschreibt:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Text ohne Emoji

Reiner Text ohne Emoji-Zeichen wird unverändert durchgereicht:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
