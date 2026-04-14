---
title: "visitHast()"
description: "Bir ziyaretçi fonksiyonu kullanarak bir HAST ağacında dolaşın ve dönüşüm uygulayın."
---

```ts
import { visitHast } from "@unifast/core";
```

## İmza

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## Parametreler

### node

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `type` | `string` | — | Düğüm türü (`"root"`, `"element"`, `"text"`, vb.) |
| `children` | `HastNode[]` | — | Alt düğümler (`"root"` ve `"element"` türleri için) |

### visitor

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | Her düğüm için çağrılan fonksiyon; orijinali değiştirmek için bir düğüm döndürün veya korumak için `void` döndürün |

## Dönüş Değeri

`HastNode` — Ziyaretçi fonksiyonu tarafından uygulanan dönüşümler içeren yeni bir ağaç.

## Kullanım

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Hello world" }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "text") {
    return { type: "text", value: node.value.toUpperCase() };
  }
});

console.log(result);
// "HELLO WORLD" metnine sahip ağaç
```

## Örnekler

### Tüm paragraflara bir sınıf ekleme

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "First paragraph." }],
    },
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Second paragraph." }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "p") {
    return {
      ...node,
      properties: { ...node.properties, className: ["prose"] },
    };
  }
});

// Her iki <p> elemanı artık className: ["prose"] değerine sahip
```

### Tüm görselleri kaldırma

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "See the photo: " },
        {
          type: "element",
          tagName: "img",
          properties: { src: "photo.jpg", alt: "A photo" },
          children: [],
        },
      ],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && (node.tagName === "p" || node.tagName === "div")) {
    return {
      ...node,
      children: node.children.filter(
        (child) => !(child.type === "element" && child.tagName === "img"),
      ),
    };
  }
});

// <img> elemanı ağaçtan kaldırıldı
```

### Tüm bağlantıları toplama

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "Visit " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://example.com" },
          children: [{ type: "text", value: "Example" }],
        },
        { type: "text", value: " and " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://docs.example.com" },
          children: [{ type: "text", value: "Docs" }],
        },
      ],
    },
  ],
};

const links: string[] = [];

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "a") {
    const href = node.properties.href;
    if (typeof href === "string") {
      links.push(href);
    }
  }
});

console.log(links);
// ["https://example.com", "https://docs.example.com"]
```

### Kod bloklarını bir kapsayıcıya sarma

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "pre",
      properties: {},
      children: [
        {
          type: "element",
          tagName: "code",
          properties: { className: ["language-js"] },
          children: [{ type: "text", value: "const x = 1;" }],
        },
      ],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    return {
      type: "element",
      tagName: "div",
      properties: { className: ["code-block"] },
      children: [node],
    } as HastNode;
  }
});

// <pre> artık <div class="code-block"> içine sarmalandı
```

## Davranış

- **Değiştirilemez:** Yeni bir ağaç döndürür; orijinal ağaç değiştirilmez
- **Yukarıdan aşağıya dolaşım:** Ziyaretçi, alt düğümler ziyaret edilmeden önce üst düğüm için çağrılır
- **Değiştirme:** Ziyaretçi bir düğüm döndürürse, alt düğümler dolaşılmadan önce geçerli düğümü değiştirir
- **İşlem yok:** Ziyaretçi `void` (veya `undefined`) döndürürse, orijinal düğüm korunur
- **Özyinelemeli:** `"root"` ve `"element"` düğümlerinin alt düğümleri özyinelemeli olarak ziyaret edilir
