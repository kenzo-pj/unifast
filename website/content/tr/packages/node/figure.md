---
title: "figure()"
description: "Alt metni olan görselleri <figure> ve <figcaption> elemanları ile sarmalar."
---

```ts
import { figure } from "@unifast/node";
```

## İmza

```ts
function figure(): UnifastPlugin
```

## Parametreler

Yok.

## Kullanım

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// Görsel, <figcaption> içeren bir <figure> ile sarmalanır
```

## Örnekler

### Temel figure sarmalama

Bir görselin alt metni olduğunda, `figure()` onu bir `<figure>` elemanıyla sarmalar ve alt metnini içeren bir `<figcaption>` ekler:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Alt metni olmayan görsel

Alt metni olmayan görseller sarmalanmaz, çünkü görüntülenecek anlamlı bir altyazı yoktur:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
