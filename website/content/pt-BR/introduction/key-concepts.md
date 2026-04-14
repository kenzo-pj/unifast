---
title: "Conceitos Principais"
description: "Entenda a pipeline de compilação multi-estágio do unifast, os passes integrados e como funcionam as transformações de MdAst e HAst."
---

O unifast compila Markdown através de uma pipeline multi-estágio. Entender esses estágios te ajuda a configurar o compilador e a escolher os plugins certos.

### Pipeline de Compilação

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Cada estágio transforma o documento através de uma representação intermediária (IR).

### Representações Intermediárias

| IR | Nome | Propósito |
|----|------|---------|
| **IR0** | MdAst | Estrutura Markdown - headings, parágrafos, listas, code blocks |
| **IR1** | HAst | Estrutura HTML - elementos, atributos, nós de texto |
| **IR2** | JsAst | Apenas MDX - imports ESM e expressões JSX |

O parser produz **MdAst**, que é então rebaixado para **HAst** para a emissão de HTML. Entradas MDX produzem adicionalmente nós **JsAst** para saída JavaScript.

### Passes

Passes são transformações aplicadas ao AST em fases específicas. O unifast tem passes integrados para tarefas comuns:

**Passes MdAst** (antes do rebaixamento para HTML):

- **Normalize** - Estrutura consistente para os passes posteriores
- **Slug** - Gera IDs de headings a partir do conteúdo textual
- **TOC** - Extrai a table of contents a partir dos headings
- **Definition Resolution** - Resolve definições de referências de links/imagens

**Passes HAst** (após o rebaixamento para HTML):

- **Sanitize** - Remove elementos e atributos HTML não permitidos
- **Highlight** - Aplica syntax highlighting aos code blocks
- **Cleanup** - Remove nós e whitespace desnecessários

Os passes são ordenados por fase - você não precisa se preocupar com a ordem de execução.

### Plugins

Plugins são pacotes TypeScript que configuram os passes integrados. Eles não executam JavaScript arbitrário durante a compilação - em vez disso, definem opções que controlam como o core em Rust processa seu documento.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Cada plugin retorna um objeto de configuração que é mesclado nas opções de compilação antes do core em Rust rodar. Isso mantém o hot path inteiramente em código nativo.

### Formatos de Saída

O compilador suporta múltiplos formatos de saída via a opção `outputKind`:

| Formato | Descrição |
|--------|-------------|
| `"html"` | String HTML (padrão) |
| `"hast"` | JSON HAst - o AST HTML para renderização customizada |
| `"mdast"` | JSON MdAst - o AST Markdown para análise |
| `"mdx-js"` | String de módulo JavaScript (apenas MDX) |

### Diagnósticos

O compilador reporta problemas via o array `diagnostics` no resultado. Cada diagnóstico inclui um nível de severidade, mensagem e um span de origem opcional para localização precisa do erro.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
