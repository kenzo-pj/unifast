---
title: "customHeadingId()"
description: "Establece IDs personalizados en los encabezados usando la sintaxis {#custom-id}."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Firma

```ts
function customHeadingId(): UnifastPlugin
```

## Parámetros

Ninguno.

## Uso

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## Ejemplos

### ID personalizado

Usa la sintaxis `{#custom-id}` al final de un encabezado para asignar un atributo `id` específico. El bloque entre llaves se elimina del texto renderizado:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Encabezado sin ID personalizado

Los encabezados sin la sintaxis `{#...}` se dejan sin cambios. Usan el slug por defecto generado a partir del texto del encabezado:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Clases y atributos arbitrarios

La sintaxis de llaves también soporta la notación `.class` y `key=value`:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
