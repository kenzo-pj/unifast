---
title: "minify()"
description: "Gereksiz boşlukları kaldırarak HTML çıktısını küçültür."
---

```ts
import { minify } from "@unifast/node";
```

## İmza

```ts
function minify(): UnifastPlugin
```

## Parametreler

Yok.

## Kullanım

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Boşluklar daraltılır ve gereksiz düğümler kaldırılır
```

## Örnekler

### Temel küçültme

`minify()` plugin'i ardışık boşluk karakterlerini tek boşluklara daraltır, HTML yorumlarını kaldırır, blok elemanları arasındaki yalnızca boşluk içeren metin düğümlerini temizler ve boş `class` ile `style` özniteliklerini kaldırır:

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### Önceden biçimlendirilmiş içerik korunur

`<pre>` ve `<code>` blokları içindeki boşluklar dokunulmadan bırakılır, böylece kod biçimlendirmesi asla bozulmaz:

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// <pre><code> bloğu içindeki boşluklar tam olarak yazıldığı gibi korunur
```
