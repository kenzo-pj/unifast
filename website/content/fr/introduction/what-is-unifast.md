---
title: "Qu'est-ce qu'unifast ?"
description: "unifast est un compilateur Markdown et MDX haute performance avec un cœur Rust. Passes intégrées pour GFM, assainissement, coloration syntaxique et table des matières."
---

unifast est un compilateur Markdown et MDX haute performance doté d'un cœur Rust. Il couvre les cas d'usage courants de remark/rehype en implémentant ces fonctionnalités directement sous forme de passes intégrées, sans recourir à une compatibilité avec les plugins JS.

### Pourquoi unifast ?

Les chaînes d'outils Markdown traditionnelles comme unified/remark/rehype sont puissantes, mais elles comportent leurs compromis :

- **Surcoût de performance** - Les multiples transformations d'AST JS s'additionnent, particulièrement à grande échelle.
- **Coordination des plugins** - Ordre d'exécution, compatibilité et redondance entre des dizaines de plugins.
- **Aucune fonctionnalité intégrée** - Même des tâches basiques comme GFM ou l'assainissement exigent des paquets séparés.

unifast adopte une approche différente :

- **Cœur Rust** - L'analyse, la transformation et l'émission s'effectuent toutes en code natif.
- **Passes intégrées** - Les fonctionnalités courantes (GFM, assainissement, coloration syntaxique, TOC) sont intégrées, pas ajoutées après coup.
- **Compilation unique** - Un seul appel compile le Markdown en HTML avec toutes les fonctionnalités appliquées.

### Fonctionnalités clés

| Fonctionnalité | Description |
|---------|-------------|
| **CommonMark + GFM** | Tableaux, listes de tâches, texte barré, liens automatiques, notes de bas de page |
| **Frontmatter** | Extraction de métadonnées YAML, TOML et JSON |
| **MDX** | Expressions JSX et imports dans le Markdown |
| **Diagnostics** | Étendues d'erreurs précises avec correspondance ligne/colonne |

### Passes intégrées

Les plugins remark/rehype les plus courants sont réimplémentés sous forme de passes Rust natives. Aucun npm install, aucune prise de tête liée à l'ordre d'exécution.

| Passe | Description |
|------|-------------|
| **Assainissement** | Liste blanche HTML fondée sur un schéma, avec des valeurs par défaut sûres |
| **Coloration syntaxique** | Moteurs interchangeables (syntect, Shiki) |
| **Table des matières** | Arborescence de titres extraite automatiquement |

### Prise en charge des plateformes

unifast s'exécute sur plusieurs plateformes à partir d'un unique cœur Rust :

- **`@unifast/node`** - Liaison Node.js via N-API (napi-rs). Cible principale.
- **`@unifast/core`** - Définitions de types TypeScript partagées par tous les paquets.
- **`unifast` (CLI)** - Interface en ligne de commande pour scripts et CI.
- **WASM** - Prise en charge des navigateurs et des runtimes edge (cible secondaire).

### Objectifs écartés

unifast n'est **pas** un remplacement immédiat d'unified. Il ne :

- Exécute pas les plugins JS remark/rehype existants au sein du cœur.
- Fournit pas de compatibilité d'API avec l'écosystème unified.
- Dépend pas de la résolution de modules de Node dans le chemin de compilation du cœur.

À la place, il vise la **complétude des cas d'usage** - en couvrant ce dont la majorité des projets ont besoin, sans la complexité liée à l'assemblage d'une chaîne de plugins.
