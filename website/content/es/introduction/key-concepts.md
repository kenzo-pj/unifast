---
title: "Conceptos clave"
description: "Comprende la pipeline de compilación multi-etapa de unifast, los pases integrados y cómo funcionan las transformaciones sobre MdAst y HAst."
---

unifast compila Markdown a través de una pipeline multi-etapa. Entender estas etapas te ayudará a configurar el compilador y a elegir los plugins adecuados.

### Pipeline de compilación

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Cada etapa transforma el documento a través de una representación intermedia (IR).

### Representaciones intermedias

| IR | Nombre | Propósito |
|----|------|---------|
| **IR0** | MdAst | Estructura Markdown: encabezados, párrafos, listas, bloques de código |
| **IR1** | HAst | Estructura HTML: elementos, atributos, nodos de texto |
| **IR2** | JsAst | Solo MDX: imports ESM y expresiones JSX |

El parser produce **MdAst**, que luego se transforma a **HAst** para la emisión de HTML. Las entradas MDX producen además nodos **JsAst** para la salida JavaScript.

### Pases

Los pases son transformaciones aplicadas al AST en fases específicas. unifast incluye pases integrados para tareas comunes:

**Pases sobre MdAst** (antes de transformar a HTML):

- **Normalize** - Estructura consistente para los pases posteriores
- **Slug** - Genera IDs de encabezado a partir del contenido de texto
- **TOC** - Extrae la tabla de contenidos a partir de los encabezados
- **Definition Resolution** - Resuelve las definiciones de referencia de enlaces/imágenes

**Pases sobre HAst** (después de transformar a HTML):

- **Sanitize** - Elimina los elementos y atributos HTML no permitidos
- **Highlight** - Aplica resaltado de sintaxis a los bloques de código
- **Cleanup** - Elimina nodos y espacios en blanco innecesarios

Los pases están ordenados por fase: no tienes que preocuparte por el orden de ejecución.

### Plugins

Los plugins son paquetes TypeScript que configuran los pases integrados. No ejecutan JavaScript arbitrario durante la compilación: en su lugar, establecen opciones que controlan cómo el núcleo en Rust procesa tu documento.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Cada plugin retorna un objeto de configuración que se combina con las opciones de compilación antes de que se ejecute el núcleo en Rust. Esto mantiene el camino crítico completamente en código nativo.

### Formatos de salida

El compilador admite múltiples formatos de salida a través de la opción `outputKind`:

| Formato | Descripción |
|--------|-------------|
| `"html"` | Cadena HTML (por defecto) |
| `"hast"` | JSON del HAst, el AST HTML para renderizado personalizado |
| `"mdast"` | JSON del MdAst, el AST Markdown para análisis |
| `"mdx-js"` | Cadena de módulo JavaScript (solo MDX) |

### Diagnósticos

El compilador reporta los problemas a través del array `diagnostics` en el resultado. Cada diagnóstico incluye un nivel de severidad, un mensaje y un rango de origen opcional para una localización precisa del error.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
