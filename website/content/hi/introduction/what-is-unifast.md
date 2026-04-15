---
title: "unifast क्या है?"
description: "unifast एक उच्च-प्रदर्शन Markdown और MDX कंपाइलर है जिसका कोर Rust में बना है। GFM, sanitization, highlighting, और TOC के लिए अंतर्निहित passes।"
---

unifast एक उच्च-प्रदर्शन Markdown और MDX कंपाइलर है जिसका कोर Rust में बना है। यह remark/rehype के मुख्यधारा उपयोग-मामलों को कवर करता है — features को सीधे अंतर्निहित passes के रूप में लागू करके, न कि JS plugin compatibility के माध्यम से।

### unifast क्यों?

unified/remark/rehype जैसी पारंपरिक Markdown toolchains शक्तिशाली हैं, लेकिन उनके साथ कुछ trade-offs भी आते हैं:

- **Performance overhead** - कई JS AST transformations जुड़ते जाते हैं, खासकर बड़े पैमाने पर।
- **Plugin coordination** - दर्जनों plugins के बीच ordering, compatibility, और duplication।
- **No built-in features** - GFM या sanitization जैसे बुनियादी कार्यों के लिए भी अलग-अलग packages चाहिए।

unifast एक अलग दृष्टिकोण अपनाता है:

- **Rust core** - Parsing, transformation, और emission सब कुछ native code में होता है।
- **Built-in passes** - आम features (GFM, sanitization, highlighting, TOC) अंतर्निहित हैं, बाद में जोड़े नहीं गए।
- **Single compilation** - एक ही call सभी features लागू करके Markdown को HTML में कंपाइल करता है।

### मुख्य विशेषताएँ

| विशेषता | विवरण |
|---------|-------------|
| **CommonMark + GFM** | Tables, task lists, strikethrough, autolinks, footnotes |
| **Frontmatter** | YAML, TOML, और JSON metadata निकालना |
| **MDX** | Markdown में JSX expressions और imports |
| **Diagnostics** | line/column mapping के साथ सटीक error spans |

### अंतर्निहित Passes

आम remark/rehype plugins को native Rust passes के रूप में फिर से लागू किया गया है। न कोई npm install, न ordering की झंझट।

| Pass | विवरण |
|------|-------------|
| **Sanitization** | सुरक्षित defaults के साथ schema-based HTML allowlist |
| **Syntax Highlighting** | Pluggable engines (syntect, Shiki) |
| **Table of Contents** | स्वचालित रूप से निकाला गया heading tree |

### प्लेटफ़ॉर्म सपोर्ट

unifast एक ही Rust core से कई प्लेटफ़ॉर्म्स पर चलता है:

- **`@unifast/node`** - N-API (napi-rs) के माध्यम से Node.js binding। प्राथमिक target।
- **`@unifast/core`** - सभी packages में साझा की गई TypeScript type definitions।
- **`unifast` (CLI)** - scripts और CI के लिए command-line interface।
- **WASM** - Browser और edge runtime सपोर्ट (द्वितीयक target)।

### Non-goals

unifast unified के लिए **drop-in replacement नहीं** है। यह निम्न नहीं करता:

- कोर के अंदर मौजूदा remark/rehype JS plugins execute करना।
- unified ecosystem के साथ API compatibility प्रदान करना।
- कोर compilation path में Node की module resolution पर निर्भर रहना।

इसके बजाय, यह **use-case completeness** पर लक्ष्य करता है — plugin pipeline को इकट्ठा करने की जटिलता के बिना, अधिकांश projects को जो चाहिए वह कवर करता है।
