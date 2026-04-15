---
title: "Was ist unifast?"
description: "unifast ist ein hochperformanter Markdown- und MDX-Compiler mit einem Rust-Kern. Integrierte Passes für GFM, Sanitization, Hervorhebung und Inhaltsverzeichnis."
---

unifast ist ein hochperformanter Markdown- und MDX-Compiler mit einem Rust-Kern. Er deckt die gängigen Anwendungsfälle von remark/rehype ab, indem er Funktionen direkt als integrierte Passes implementiert – nicht über die Kompatibilität mit JS-Plugins.

### Warum unifast?

Klassische Markdown-Toolchains wie unified/remark/rehype sind mächtig, bringen jedoch Nachteile mit sich:

- **Performance-Overhead** – Mehrfache JS-AST-Transformationen summieren sich, besonders bei großem Umfang.
- **Plugin-Koordination** – Reihenfolge, Kompatibilität und Duplikate über Dutzende von Plugins hinweg.
- **Keine integrierten Funktionen** – Selbst grundlegende Aufgaben wie GFM oder Sanitization erfordern separate Pakete.

unifast verfolgt einen anderen Ansatz:

- **Rust-Kern** – Parsing, Transformation und Ausgabe finden vollständig in nativem Code statt.
- **Integrierte Passes** – Gängige Funktionen (GFM, Sanitization, Hervorhebung, Inhaltsverzeichnis) sind eingebaut, nicht nachträglich aufgesetzt.
- **Einzelne Kompilierung** – Ein einziger Aufruf kompiliert Markdown zu HTML mit allen angewendeten Funktionen.

### Wichtige Funktionen

| Funktion | Beschreibung |
|---------|-------------|
| **CommonMark + GFM** | Tabellen, Aufgabenlisten, Durchstreichung, Autolinks, Fußnoten |
| **Frontmatter** | Extraktion von Metadaten in YAML, TOML und JSON |
| **MDX** | JSX-Ausdrücke und Imports in Markdown |
| **Diagnostik** | Präzise Fehlerbereiche mit Zeilen-/Spaltenzuordnung |

### Integrierte Passes

Gängige remark/rehype-Plugins sind als native Rust-Passes neu implementiert. Kein npm install, keine Sorgen um die Reihenfolge.

| Pass | Beschreibung |
|------|-------------|
| **Sanitization** | Schema-basierte HTML-Allowlist mit sicheren Voreinstellungen |
| **Syntax-Hervorhebung** | Austauschbare Engines (syntect, Shiki) |
| **Inhaltsverzeichnis** | Automatisch extrahierter Überschriftenbaum |

### Plattform-Unterstützung

unifast läuft aus einem einzigen Rust-Kern auf mehreren Plattformen:

- **`@unifast/node`** – Node.js-Bindung über N-API (napi-rs). Primäres Ziel.
- **`@unifast/core`** – TypeScript-Typdefinitionen, die von allen Paketen gemeinsam genutzt werden.
- **`unifast` (CLI)** – Kommandozeilenschnittstelle für Skripte und CI.
- **WASM** – Unterstützung für Browser- und Edge-Laufzeiten (sekundäres Ziel).

### Nicht-Ziele

unifast ist **kein** Drop-in-Ersatz für unified. Es bietet nicht:

- Ausführung bestehender remark/rehype-JS-Plugins innerhalb des Kerns.
- API-Kompatibilität mit dem unified-Ökosystem.
- Abhängigkeit von Nodes Modulauflösung im Kern-Kompilierungspfad.

Stattdessen zielt es auf **Vollständigkeit der Anwendungsfälle** ab – und deckt das ab, was die meisten Projekte benötigen, ohne die Komplexität des Zusammenstellens einer Plugin-Pipeline.
