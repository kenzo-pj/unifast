---
title: "imgLazyLoading()"
description: 'deferred loading के लिए images में loading="lazy" attribute जोड़ें।'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Signature

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Parameters

### options?

Lazy loading व्यवहार के लिए Configuration

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | skip करने के लिए images की संख्या (जैसे hero image को skip करें) |

Plugin matched `<img>` elements पर `loading="lazy"` और `decoding="async"` दोनों attributes जोड़ता है, जिसमें अन्य elements के अंदर nested images भी शामिल हैं।

## उपयोग

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

// सभी images को loading="lazy" और decoding="async" मिलते हैं:
// <img src="photo1.jpg" alt="Photo 1" loading="lazy" decoding="async">
// <img src="photo2.jpg" alt="Photo 2" loading="lazy" decoding="async">
// <img src="photo3.jpg" alt="Photo 3" loading="lazy" decoding="async">
```

## उदाहरण

### पहली image skip करें (hero image pattern)

एक page पर पहली image अक्सर एक hero या banner image होती है जिसे eagerly load होना चाहिए। इसे lazy loading से बाहर करने के लिए `skipFirst` का उपयोग करें:

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

// पहली image eagerly load होती है (कोई loading attribute नहीं):
// <img src="hero.jpg" alt="Hero banner">
//
// बची हुई images lazy loaded हैं:
// <img src="diagram.jpg" alt="Diagram" loading="lazy" decoding="async">
// <img src="screenshot.jpg" alt="Screenshot" loading="lazy" decoding="async">
```

### कई above-the-fold images skip करें

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 3, // पहली 3 images skip करें
    }),
  ],
});
```
