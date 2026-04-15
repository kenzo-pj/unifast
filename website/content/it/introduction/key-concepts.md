---
title: "Concetti fondamentali"
description: "Scopri la pipeline di compilazione multi-fase di unifast, i pass integrati e il funzionamento delle trasformazioni MdAst e HAst."
---

unifast compila il Markdown attraverso una pipeline multi-fase. Comprendere queste fasi aiuta a configurare il compilatore e a scegliere i plugin più adatti.

### Pipeline di compilazione

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Ogni fase trasforma il documento attraverso una rappresentazione intermedia (IR).

### Rappresentazioni intermedie

| IR | Nome | Scopo |
|----|------|-------|
| **IR0** | MdAst | Struttura Markdown - titoli, paragrafi, elenchi, blocchi di codice |
| **IR1** | HAst | Struttura HTML - elementi, attributi, nodi di testo |
| **IR2** | JsAst | Solo MDX - import ESM ed espressioni JSX |

Il parser produce **MdAst**, che viene poi ridotto a **HAst** per l'emissione HTML. Gli input MDX producono in aggiunta nodi **JsAst** per l'output JavaScript.

### Pass

I pass sono trasformazioni applicate all'AST in fasi specifiche. unifast include pass integrati per le attività più comuni:

**Pass MdAst** (prima della riduzione a HTML):

- **Normalize** - Struttura coerente per i pass successivi
- **Slug** - Genera gli ID dei titoli a partire dal testo
- **TOC** - Estrae la tabella dei contenuti dai titoli
- **Definition Resolution** - Risolve le definizioni di riferimento per link e immagini

**Pass HAst** (dopo la riduzione a HTML):

- **Sanitize** - Rimuove elementi e attributi HTML non consentiti
- **Highlight** - Applica l'evidenziazione della sintassi ai blocchi di codice
- **Cleanup** - Rimuove nodi superflui e spazi bianchi

I pass sono ordinati per fase: non è necessario preoccuparsi dell'ordine di esecuzione.

### Plugin

I plugin sono pacchetti TypeScript che configurano i pass integrati. Non eseguono JavaScript arbitrario durante la compilazione: si limitano a impostare opzioni che controllano il modo in cui il core Rust elabora il documento.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Ogni plugin restituisce un oggetto di configurazione che viene unito alle opzioni di compilazione prima dell'esecuzione del core Rust. Questo permette di mantenere il percorso critico interamente in codice nativo.

### Formati di output

Il compilatore supporta diversi formati di output tramite l'opzione `outputKind`:

| Formato | Descrizione |
|---------|-------------|
| `"html"` | Stringa HTML (predefinito) |
| `"hast"` | JSON HAst - l'AST HTML per il rendering personalizzato |
| `"mdast"` | JSON MdAst - l'AST Markdown per l'analisi |
| `"mdx-js"` | Stringa modulo JavaScript (solo MDX) |

### Diagnostica

Il compilatore segnala i problemi tramite l'array `diagnostics` nel risultato. Ogni diagnostica include un livello di gravità, un messaggio e, opzionalmente, un intervallo sorgente per individuare con precisione la posizione dell'errore.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
