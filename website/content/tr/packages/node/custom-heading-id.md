---
title: "customHeadingId()"
description: "{#custom-id} sözdizimini kullanarak başlıklara özel ID'ler atar."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## İmza

```ts
function customHeadingId(): UnifastPlugin
```

## Parametreler

Yok.

## Kullanım

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// Başlık, otomatik oluşturulan slug yerine id="intro" alır
```

## Örnekler

### Özel ID

Belirli bir `id` özniteliği atamak için bir başlığın sonunda `{#custom-id}` sözdizimini kullanın. Süslü parantez bloğu render edilen metinden kaldırılır:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Özel ID olmayan başlık

`{#...}` sözdizimi olmayan başlıklar değiştirilmeden bırakılır. Başlık metninden üretilen varsayılan slug'ı kullanırlar:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Sınıflar ve keyfi öznitelikler

Süslü parantez sözdizimi `.class` ve `key=value` notasyonlarını da destekler:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
