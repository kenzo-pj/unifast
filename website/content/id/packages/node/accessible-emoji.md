---
title: "accessibleEmoji()"
description: 'Membungkus karakter emoji dalam elemen <span role="img"> dengan atribut aria-label untuk aksesibilitas.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Signature

```ts
function accessibleEmoji(): UnifastPlugin
```

## Parameter

Tidak ada.

## Penggunaan

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## Contoh

### Pembungkusan emoji dengan label aria

Setiap karakter emoji dibungkus dalam sebuah `<span>` dengan `role="img"` dan sebuah `aria-label` yang mendeskripsikan emoji tersebut untuk pembaca layar:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Teks tanpa emoji

Teks polos tanpa karakter emoji dilewatkan tanpa perubahan:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
