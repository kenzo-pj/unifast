---
title: "Kernkonzepte"
description: "Verstehen Sie die mehrstufige Kompilierungspipeline von unifast, die integrierten Passes sowie die Funktionsweise der MdAst- und HAst-Transformationen."
---

unifast kompiliert Markdown über eine mehrstufige Pipeline. Das Verständnis dieser Stufen hilft Ihnen, den Compiler zu konfigurieren und die richtigen Plugins auszuwählen.

### Kompilierungspipeline

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Jede Stufe transformiert das Dokument über eine Zwischendarstellung (IR).

### Zwischendarstellungen

| IR | Name | Zweck |
|----|------|---------|
| **IR0** | MdAst | Markdown-Struktur – Überschriften, Absätze, Listen, Code-Blöcke |
| **IR1** | HAst | HTML-Struktur – Elemente, Attribute, Textknoten |
| **IR2** | JsAst | Nur MDX – ESM-Imports und JSX-Ausdrücke |

Der Parser erzeugt **MdAst**, der anschließend für die HTML-Ausgabe zu **HAst** heruntergebrochen wird. MDX-Eingaben erzeugen zusätzlich **JsAst**-Knoten für die JavaScript-Ausgabe.

### Passes

Passes sind Transformationen, die in bestimmten Phasen auf den AST angewendet werden. unifast verfügt über integrierte Passes für gängige Aufgaben:

**MdAst-Passes** (vor dem Herunterbrechen zu HTML):

- **Normalize** – Konsistente Struktur für nachgelagerte Passes
- **Slug** – Erzeugen von Überschriften-IDs aus dem Textinhalt
- **TOC** – Extrahieren eines Inhaltsverzeichnisses aus den Überschriften
- **Definition Resolution** – Auflösen von Link-/Bildreferenz-Definitionen

**HAst-Passes** (nach dem Herunterbrechen zu HTML):

- **Sanitize** – Entfernen nicht erlaubter HTML-Elemente und -Attribute
- **Highlight** – Anwenden der Syntax-Hervorhebung auf Code-Blöcke
- **Cleanup** – Entfernen unnötiger Knoten und Leerzeichen

Passes werden nach Phase geordnet – Sie müssen sich keine Gedanken über die Ausführungsreihenfolge machen.

### Plugins

Plugins sind TypeScript-Pakete, die integrierte Passes konfigurieren. Sie führen während der Kompilierung kein beliebiges JavaScript aus – stattdessen setzen sie Optionen, die steuern, wie der Rust-Kern Ihr Dokument verarbeitet.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Jedes Plugin gibt ein Konfigurationsobjekt zurück, das vor dem Ausführen des Rust-Kerns in die Kompilierungsoptionen eingemischt wird. Dadurch bleibt der Hot Path vollständig in nativem Code.

### Ausgabeformate

Der Compiler unterstützt über die Option `outputKind` mehrere Ausgabeformate:

| Format | Beschreibung |
|--------|-------------|
| `"html"` | HTML-String (Standard) |
| `"hast"` | HAst-JSON – der HTML-AST für benutzerdefiniertes Rendering |
| `"mdast"` | MdAst-JSON – der Markdown-AST für die Analyse |
| `"mdx-js"` | JavaScript-Modul-String (nur MDX) |

### Diagnostik

Der Compiler meldet Probleme über das `diagnostics`-Array im Ergebnis. Jede Diagnose enthält einen Schweregrad, eine Nachricht und einen optionalen Quellbereich zur präzisen Fehlerlokalisierung.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
