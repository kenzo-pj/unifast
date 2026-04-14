---
title: "मुख्य अवधारणाएँ"
description: "unifast की multi-stage compilation pipeline, अंतर्निहित passes, और MdAst और HAst transformations कैसे काम करते हैं — यह समझें।"
---

unifast Markdown को एक multi-stage pipeline के माध्यम से कंपाइल करता है। इन stages को समझने से आपको कंपाइलर को configure करने और सही plugins चुनने में मदद मिलेगी।

### Compilation Pipeline

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

प्रत्येक stage document को एक intermediate representation (IR) के माध्यम से रूपांतरित करता है।

### Intermediate Representations

| IR | नाम | उद्देश्य |
|----|------|---------|
| **IR0** | MdAst | Markdown structure - headings, paragraphs, lists, code blocks |
| **IR1** | HAst | HTML structure - elements, attributes, text nodes |
| **IR2** | JsAst | केवल MDX - ESM imports और JSX expressions |

Parser **MdAst** उत्पन्न करता है, जिसे फिर HTML emission के लिए **HAst** में lower किया जाता है। MDX inputs अतिरिक्त रूप से JavaScript output के लिए **JsAst** nodes भी उत्पन्न करते हैं।

### Passes

Passes वे transformations हैं जो AST पर विशिष्ट phases में लागू होते हैं। unifast में आम कार्यों के लिए अंतर्निहित passes हैं:

**MdAst passes** (HTML में lower करने से पहले):

- **Normalize** - downstream passes के लिए सुसंगत structure
- **Slug** - text content से heading IDs उत्पन्न करना
- **TOC** - headings से table of contents निकालना
- **Definition Resolution** - link/image reference definitions को resolve करना

**HAst passes** (HTML में lower करने के बाद):

- **Sanitize** - अनुमत न होने वाले HTML elements और attributes हटाना
- **Highlight** - code blocks पर syntax highlighting लागू करना
- **Cleanup** - अनावश्यक nodes और whitespace हटाना

Passes phase के अनुसार ordered होते हैं — आपको execution order की चिंता करने की ज़रूरत नहीं है।

### Plugins

Plugins TypeScript packages हैं जो अंतर्निहित passes को configure करते हैं। ये compilation के दौरान कोई arbitrary JavaScript नहीं चलाते — इसके बजाय, ये options set करते हैं जो नियंत्रित करते हैं कि Rust core आपके document को कैसे process करे।

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

प्रत्येक plugin एक configuration object return करता है जिसे Rust core चलने से पहले compile options में merge किया जाता है। यह hot path को पूरी तरह native code में रखता है।

### Output Formats

कंपाइलर `outputKind` option के माध्यम से कई output formats को सपोर्ट करता है:

| Format | विवरण |
|--------|-------------|
| `"html"` | HTML string (default) |
| `"hast"` | HAst JSON - custom rendering के लिए HTML AST |
| `"mdast"` | MdAst JSON - विश्लेषण के लिए Markdown AST |
| `"mdx-js"` | JavaScript module string (केवल MDX) |

### Diagnostics

कंपाइलर result में `diagnostics` array के माध्यम से issues रिपोर्ट करता है। प्रत्येक diagnostic में एक severity level, message, और सटीक error स्थान के लिए optional source span शामिल होता है।

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
