---
title: "imgLazyLoading()"
description: 'Ertelenmiş yükleme için görsellere loading="lazy" özniteliği ekler.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## İmza

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Parametreler

### options?

Geç yükleme davranışı yapılandırması

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `skipFirst` | `number` | `0` | Atlanacak görsel sayısı (örn. hero görselini atla) |

Plugin, diğer elemanların içinde iç içe geçmiş görseller dahil eşleşen `<img>` elemanlarına hem `loading="lazy"` hem de `decoding="async"` özniteliklerini ekler.

## Kullanım

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const md = `
![Photo 1](photo1.jpg)

![Photo 2](photo2.jpg)

![Photo 3](photo3.jpg)
`;

const result = compile(md, {
  plugins: [imgLazyLoading()],
});

// Tüm görseller loading="lazy" ve decoding="async" alır:
// <img src="photo1.jpg" alt="Photo 1" loading="lazy" decoding="async">
// <img src="photo2.jpg" alt="Photo 2" loading="lazy" decoding="async">
// <img src="photo3.jpg" alt="Photo 3" loading="lazy" decoding="async">
```

## Örnekler

### İlk görseli atla (hero görsel kalıbı)

Bir sayfadaki ilk görsel genellikle hemen yüklenmesi gereken bir hero veya banner görselidir. Geç yüklemeden hariç tutmak için `skipFirst` kullanın:

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const md = `
![Hero banner](hero.jpg)

Some introductory content...

![Diagram](diagram.jpg)

More content...

![Screenshot](screenshot.jpg)
`;

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 1,
    }),
  ],
});

// İlk görsel hemen yüklenir (loading özniteliği yok):
// <img src="hero.jpg" alt="Hero banner">
//
// Kalan görseller geç yüklenir:
// <img src="diagram.jpg" alt="Diagram" loading="lazy" decoding="async">
// <img src="screenshot.jpg" alt="Screenshot" loading="lazy" decoding="async">
```

### Ekranın üst kısmındaki birden fazla görseli atla

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 3, // ilk 3 görseli atla
    }),
  ],
});
```
