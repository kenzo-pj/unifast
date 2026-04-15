---
title: "commentRemoval()"
description: "HTML yorumlarını çıktıdan kaldırır."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## İmza

```ts
function commentRemoval(): UnifastPlugin
```

## Parametreler

Yok.

## Kullanım

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// HTML yorumu çıktıdan kaldırılır
```

## Örnekler

### Temel yorum kaldırma

Tüm HTML yorum düğümleri (`<!-- ... -->`), blockquote gibi blok elemanlarının içinde iç içe geçmiş yorumlar dahil çıktı ağacından kaldırılır:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### Yorum olmayan HTML korunur

Yalnızca yorum düğümleri kaldırılır. Diğer satır içi HTML dokunulmadan bırakılır:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
