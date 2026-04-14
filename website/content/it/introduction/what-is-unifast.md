---
title: "Cos'è unifast?"
description: "unifast è un compilatore Markdown e MDX ad alte prestazioni con core Rust. Pass integrati per GFM, sanitizzazione, evidenziazione e TOC."
---

unifast è un compilatore Markdown e MDX ad alte prestazioni con core Rust. Copre i principali casi d'uso di remark/rehype implementando le funzionalità direttamente come pass integrati, senza passare per la compatibilità con i plugin JS.

### Perché unifast?

Le toolchain Markdown tradizionali come unified/remark/rehype sono potenti, ma presentano alcuni compromessi:

- **Overhead sulle prestazioni** - Le ripetute trasformazioni dell'AST in JS si sommano, soprattutto su larga scala.
- **Coordinamento dei plugin** - Ordinamento, compatibilità e duplicazione tra decine di plugin.
- **Nessuna funzionalità integrata** - Persino attività di base come GFM o sanitizzazione richiedono pacchetti separati.

unifast adotta un approccio diverso:

- **Core Rust** - Parsing, trasformazione ed emissione avvengono tutti in codice nativo.
- **Pass integrati** - Le funzionalità più comuni (GFM, sanitizzazione, evidenziazione, TOC) sono integrate, non aggiunte a posteriori.
- **Compilazione unica** - Una sola chiamata compila il Markdown in HTML applicando tutte le funzionalità.

### Caratteristiche principali

| Funzionalità | Descrizione |
|--------------|-------------|
| **CommonMark + GFM** | Tabelle, elenchi di attività, strikethrough, autolink, note a piè di pagina |
| **Frontmatter** | Estrazione di metadati YAML, TOML e JSON |
| **MDX** | Espressioni JSX e import all'interno del Markdown |
| **Diagnostica** | Intervalli di errore precisi con mappatura riga/colonna |

### Pass integrati

I plugin remark/rehype più comuni sono reimplementati come pass Rust nativi. Niente npm install, niente grattacapi di ordinamento.

| Pass | Descrizione |
|------|-------------|
| **Sanitizzazione** | Allowlist HTML basata su schema con impostazioni predefinite sicure |
| **Evidenziazione della sintassi** | Motori intercambiabili (syntect, Shiki) |
| **Tabella dei contenuti** | Albero dei titoli estratto automaticamente |

### Supporto delle piattaforme

unifast gira su più piattaforme da un unico core Rust:

- **`@unifast/node`** - Binding Node.js tramite N-API (napi-rs). Target principale.
- **`@unifast/core`** - Definizioni dei tipi TypeScript condivise tra tutti i pacchetti.
- **`unifast` (CLI)** - Interfaccia a riga di comando per script e CI.
- **WASM** - Supporto per browser e runtime edge (target secondario).

### Obiettivi esclusi

unifast **non** è un sostituto drop-in di unified. In particolare:

- Non esegue i plugin JS esistenti di remark/rehype all'interno del core.
- Non offre compatibilità API con l'ecosistema unified.
- Non dipende dalla risoluzione dei moduli di Node nel percorso di compilazione del core.

Punta invece alla **completezza sui casi d'uso**: coprire ciò di cui la maggior parte dei progetti ha bisogno senza la complessità di assemblare una pipeline di plugin.
