---
title: "accessibleEmoji()"
description: 'Erişilebilirlik için emoji karakterlerini aria-label öznitelikli <span role="img"> elemanlarıyla sarmalar.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## İmza

```ts
function accessibleEmoji(): UnifastPlugin
```

## Parametreler

Yok.

## Kullanım

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji, <span role="img" aria-label="rocket"> ile sarmalanır
```

## Örnekler

### Aria etiketleriyle emoji sarmalama

Her emoji karakteri, ekran okuyucular için emoji'yi tanımlayan `role="img"` ve `aria-label` özniteliklerine sahip bir `<span>` ile sarmalanır:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Emoji içermeyen metin

Emoji karakterleri olmayan düz metin değiştirilmeden geçer:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
