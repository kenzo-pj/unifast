---
title: "excerpt()"
description: "Belge içeriğinden bir özet çıkarır."
---

```ts
import { excerpt } from "@unifast/node";
```

## İmza

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Parametreler

### options?

Özet çıkarma yapılandırması

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `separator` | `string` | `"<!-- more -->"` | Özeti geri kalanından ayıran yorum işareti |
| `fallbackParagraphs` | `number` | `1` | Ayırıcı bulunamadığında özet olarak kullanılacak baştaki paragraf sayısı |
| `fallbackCharacters` | `number` | `undefined` | Yedek özet için maksimum karakter uzunluğu (kelime sınırında kesilir) |

Hem `fallbackParagraphs` hem de `fallbackCharacters` ayarlandığında `fallbackParagraphs` önceliklidir.

## Dönüş Değeri

Plugin, derleme sonucuna bir `excerpt` özelliği ekler:

| Özellik | Tür | Açıklama |
|---------|-----|----------|
| `result.excerpt` | `string \| undefined` | Belgeden çıkarılan düz metin özeti |

## Kullanım

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is the introduction to my blog post.

<!-- more -->

The rest of the article continues here with more details.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "This is the introduction to my blog post."
```

## Örnekler

### Ayırıcı işareti ile

Özetin nerede bittiğini açıkça işaretlemek için Markdown'unuza bir `<!-- more -->` yorumu yerleştirin:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is a **bold** introduction with some content.

Here is a second paragraph still in the excerpt.

<!-- more -->

This content is not included in the excerpt.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "My Blog Post This is a bold introduction with some content. Here is a second paragraph still in the excerpt."
```

### İlk paragrafa geri dönüş

Ayırıcı işareti bulunamadığında, plugin ilk N paragrafı çıkarmaya geri döner:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is the first paragraph of my article.

This is the second paragraph with more details.

This is the third paragraph.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackParagraphs: 2,
    }),
  ],
});

console.log(result.excerpt);
// "This is the first paragraph of my article. This is the second paragraph with more details."
```

### Karakter sınırına geri dönüş

Özeti maksimum karakter uzunluğuna göre kısaltarak kelime sınırında kesin:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is a long article that goes on and on with lots of content.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackCharacters: 30,
    }),
  ],
});

console.log(result.excerpt);
// "This is a long article that"
```
