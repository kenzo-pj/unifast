---
title: "¿Qué es unifast?"
description: "unifast es un compilador de Markdown y MDX de alto rendimiento con un núcleo en Rust. Pases integrados para GFM, sanitización, resaltado y TOC."
---

unifast es un compilador de Markdown y MDX de alto rendimiento con un núcleo en Rust. Cubre los casos de uso principales de remark/rehype implementando las funcionalidades directamente como pases integrados, no a través de la compatibilidad con plugins de JS.

### ¿Por qué unifast?

Las cadenas de herramientas tradicionales de Markdown como unified/remark/rehype son potentes, pero implican ciertos compromisos:

- **Sobrecarga de rendimiento** - Las múltiples transformaciones sobre el AST en JS se acumulan, especialmente a gran escala.
- **Coordinación de plugins** - Orden, compatibilidad y duplicación entre docenas de plugins.
- **Sin funcionalidades integradas** - Incluso tareas básicas como GFM o la sanitización requieren paquetes separados.

unifast adopta un enfoque diferente:

- **Núcleo en Rust** - El parsing, la transformación y la emisión suceden por completo en código nativo.
- **Pases integrados** - Las funcionalidades comunes (GFM, sanitización, resaltado, TOC) están integradas, no añadidas a posteriori.
- **Compilación única** - Una sola llamada compila Markdown a HTML con todas las funcionalidades aplicadas.

### Funcionalidades clave

| Funcionalidad | Descripción |
|---------|-------------|
| **CommonMark + GFM** | Tablas, listas de tareas, tachado, autoenlaces, notas al pie |
| **Frontmatter** | Extracción de metadatos YAML, TOML y JSON |
| **MDX** | Expresiones JSX e imports dentro de Markdown |
| **Diagnósticos** | Rangos de error precisos con mapeo de línea/columna |

### Pases integrados

Los plugins comunes de remark/rehype están reimplementados como pases nativos en Rust. Sin npm install, sin dolores de cabeza por el orden.

| Pase | Descripción |
|------|-------------|
| **Sanitización** | Lista blanca de HTML basada en esquemas con valores por defecto seguros |
| **Resaltado de sintaxis** | Motores intercambiables (syntect, Shiki) |
| **Tabla de contenidos** | Árbol de encabezados extraído automáticamente |

### Plataformas soportadas

unifast se ejecuta en múltiples plataformas a partir de un único núcleo en Rust:

- **`@unifast/node`** - Binding para Node.js a través de N-API (napi-rs). Destino principal.
- **`@unifast/core`** - Definiciones de tipos TypeScript compartidas entre todos los paquetes.
- **`unifast` (CLI)** - Interfaz de línea de comandos para scripts y CI.
- **WASM** - Soporte para navegadores y runtimes edge (destino secundario).

### No-objetivos

unifast **no** es un reemplazo directo de unified. No:

- Ejecuta plugins de remark/rehype existentes dentro del núcleo.
- Ofrece compatibilidad de API con el ecosistema unified.
- Depende de la resolución de módulos de Node en la ruta de compilación del núcleo.

En su lugar, apunta a la **completitud por caso de uso**, cubriendo lo que la mayoría de los proyectos necesitan sin la complejidad de ensamblar una cadena de plugins.
