---
title: "minify()"
description: "Minifica a saída HTML removendo whitespace desnecessário."
---

```ts
import { minify } from "@unifast/node";
```

## Assinatura

```ts
function minify(): UnifastPlugin
```

## Parâmetros

Nenhum.

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

## Exemplos

### Minificação básica

O plugin `minify()` colapsa caracteres consecutivos de whitespace em espaços únicos, remove comentários HTML, retira nós de texto contendo apenas whitespace entre elementos block e remove atributos `class` e `style` vazios:

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

### Conteúdo preformatado é preservado

Whitespace dentro de blocos `<pre>` e `<code>` é deixado intacto, então a formatação de código nunca é quebrada:

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
