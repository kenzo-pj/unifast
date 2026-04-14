---
title: "Concepts clés"
description: "Comprenez le pipeline de compilation multi-étapes d'unifast, ses passes intégrées, ainsi que le fonctionnement des transformations MdAst et HAst."
---

unifast compile le Markdown au travers d'un pipeline multi-étapes. Comprendre ces étapes vous aide à configurer le compilateur et à choisir les bons plugins.

### Pipeline de compilation

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Chaque étape transforme le document via une représentation intermédiaire (IR).

### Représentations intermédiaires

| IR | Nom | Rôle |
|----|------|---------|
| **IR0** | MdAst | Structure Markdown - titres, paragraphes, listes, blocs de code |
| **IR1** | HAst | Structure HTML - éléments, attributs, nœuds texte |
| **IR2** | JsAst | MDX uniquement - imports ESM et expressions JSX |

Le parser produit du **MdAst**, qui est ensuite abaissé en **HAst** pour l'émission HTML. Les entrées MDX produisent en plus des nœuds **JsAst** pour la sortie JavaScript.

### Passes

Les passes sont des transformations appliquées à l'AST à des phases précises. unifast dispose de passes intégrées pour les tâches courantes :

**Passes MdAst** (avant l'abaissement vers HTML) :

- **Normalize** - Structure cohérente pour les passes suivantes
- **Slug** - Génération d'identifiants de titres à partir du contenu textuel
- **TOC** - Extraction de la table des matières à partir des titres
- **Definition Resolution** - Résolution des définitions de références de liens et d'images

**Passes HAst** (après l'abaissement vers HTML) :

- **Sanitize** - Suppression des éléments et attributs HTML non autorisés
- **Highlight** - Application de la coloration syntaxique aux blocs de code
- **Cleanup** - Suppression des nœuds et espaces superflus

Les passes sont ordonnées par phase - vous n'avez pas à vous soucier de l'ordre d'exécution.

### Plugins

Les plugins sont des paquets TypeScript qui configurent les passes intégrées. Ils n'exécutent pas de JavaScript arbitraire pendant la compilation - à la place, ils définissent des options qui contrôlent la façon dont le cœur Rust traite votre document.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Chaque plugin renvoie un objet de configuration qui est fusionné dans les options de compilation avant l'exécution du cœur Rust. Cela permet de garder entièrement le chemin critique en code natif.

### Formats de sortie

Le compilateur prend en charge plusieurs formats de sortie via l'option `outputKind` :

| Format | Description |
|--------|-------------|
| `"html"` | Chaîne HTML (par défaut) |
| `"hast"` | HAst JSON - l'AST HTML pour un rendu personnalisé |
| `"mdast"` | MdAst JSON - l'AST Markdown pour l'analyse |
| `"mdx-js"` | Chaîne de module JavaScript (MDX uniquement) |

### Diagnostics

Le compilateur signale les problèmes via le tableau `diagnostics` présent dans le résultat. Chaque diagnostic comporte un niveau de gravité, un message et, éventuellement, une étendue source permettant de localiser précisément l'erreur.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
