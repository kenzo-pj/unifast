---
title: "O que é o unifast?"
description: "O unifast é um compilador de Markdown e MDX de alta performance com core em Rust. Passes integrados para GFM, sanitização, highlighting e TOC."
---

O unifast é um compilador de Markdown e MDX de alta performance com core em Rust. Ele cobre os principais casos de uso do remark/rehype implementando funcionalidades diretamente como passes integrados - e não por meio de compatibilidade com plugins JS.

### Por que o unifast?

Toolchains tradicionais de Markdown como unified/remark/rehype são poderosas, mas vêm com trade-offs:

- **Sobrecarga de performance** - Múltiplas transformações de AST em JS se acumulam, especialmente em larga escala.
- **Coordenação de plugins** - Ordenação, compatibilidade e duplicação entre dezenas de plugins.
- **Sem funcionalidades integradas** - Até tarefas básicas como GFM ou sanitização exigem pacotes separados.

O unifast adota uma abordagem diferente:

- **Core em Rust** - Parsing, transformação e emissão acontecem todos em código nativo.
- **Passes integrados** - Funcionalidades comuns (GFM, sanitização, highlighting, TOC) são integradas, não adicionadas por cima.
- **Compilação única** - Uma chamada compila Markdown para HTML com todas as funcionalidades aplicadas.

### Principais Funcionalidades

| Funcionalidade | Descrição |
|---------|-------------|
| **CommonMark + GFM** | Tabelas, task lists, strikethrough, autolinks, footnotes |
| **Frontmatter** | Extração de metadados em YAML, TOML e JSON |
| **MDX** | Expressões JSX e imports em Markdown |
| **Diagnósticos** | Spans de erro precisos com mapeamento de linha/coluna |

### Passes Integrados

Plugins comuns do remark/rehype são reimplementados como passes nativos em Rust. Sem npm install, sem dores de cabeça com ordenação.

| Pass | Descrição |
|------|-------------|
| **Sanitização** | Allowlist de HTML baseada em schema com padrões seguros |
| **Syntax Highlighting** | Engines plugáveis (syntect, Shiki) |
| **Table of Contents** | Árvore de headings extraída automaticamente |

### Suporte a Plataformas

O unifast roda em múltiplas plataformas a partir de um único core em Rust:

- **`@unifast/node`** - Binding para Node.js via N-API (napi-rs). Target principal.
- **`@unifast/core`** - Definições de tipos TypeScript compartilhadas entre todos os pacotes.
- **`unifast` (CLI)** - Interface de linha de comando para scripts e CI.
- **WASM** - Suporte a runtimes de browser e edge (target secundário).

### Não-objetivos

O unifast **não** é um substituto drop-in para o unified. Ele não:

- Executa plugins JS existentes do remark/rehype dentro do core.
- Fornece compatibilidade de API com o ecossistema unified.
- Depende da resolução de módulos do Node no caminho de compilação do core.

Em vez disso, ele foca em **completude de casos de uso** - cobrindo o que a maioria dos projetos precisa sem a complexidade de montar uma pipeline de plugins.
