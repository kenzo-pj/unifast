---
title: "minify()"
description: "Minifica la salida HTML eliminando los espacios en blanco innecesarios."
---

```ts
import { minify } from "@unifast/node";
```

## Firma

```ts
function minify(): UnifastPlugin
```

## Parámetros

Ninguno.

## Uso

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## Ejemplos

### Minificación básica

El plugin `minify()` colapsa los caracteres de espacio en blanco consecutivos en espacios simples, elimina los comentarios HTML, descarta los nodos de texto compuestos solo de espacios entre elementos en bloque y elimina los atributos `class` y `style` vacíos:

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

### El contenido preformateado se conserva

Los espacios en blanco dentro de bloques `<pre>` y `<code>` se dejan intactos, por lo que el formato del código nunca se rompe:

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// Whitespace inside the <pre><code> block is preserved exactly as written
```
