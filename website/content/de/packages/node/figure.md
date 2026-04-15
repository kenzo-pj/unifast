---
title: "figure()"
description: "Schließt Bilder mit Alt-Text in <figure>- und <figcaption>-Elemente ein."
---

```ts
import { figure } from "@unifast/node";
```

## Signatur

```ts
function figure(): UnifastPlugin
```

## Parameter

Keine.

## Verwendung

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## Beispiele

### Grundlegendes Einschließen in figure

Wenn ein Bild einen Alt-Text besitzt, schließt `figure()` es in ein `<figure>`-Element ein und fügt ein `<figcaption>` mit dem Alt-Text hinzu:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Bild ohne Alt-Text

Bilder ohne Alt-Text werden nicht eingeschlossen, da keine aussagekräftige Bildunterschrift angezeigt werden kann:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
