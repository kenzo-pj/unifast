---
title: "excerpt()"
description: "从文档内容中提取摘要。"
---

```ts
import { excerpt } from "@unifast/node";
```

## 签名

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## 参数

### options?

摘要提取的配置

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | 用于分隔摘要和正文的注释标记 |
| `fallbackParagraphs` | `number` | `1` | 在没有找到分隔符时，作为摘要的开头段落数量 |
| `fallbackCharacters` | `number` | `undefined` | 回退摘要的最大字符长度（在单词边界截断） |

当 `fallbackParagraphs` 和 `fallbackCharacters` 同时设置时，以 `fallbackParagraphs` 为准。

## 返回值

该插件会在编译结果中添加 `excerpt` 属性：

| 属性 | 类型 | 描述 |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | 从文档中提取出的纯文本摘要 |

## 用法

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is the introduction to my blog post.

<!-- more -->

The rest of the article continues here with more details.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "This is the introduction to my blog post."
```

## 示例

### 使用分隔标记

在 Markdown 中放置 `<!-- more -->` 注释，即可显式地标记摘要的结束位置：

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is a **bold** introduction with some content.

Here is a second paragraph still in the excerpt.

<!-- more -->

This content is not included in the excerpt.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "My Blog Post This is a bold introduction with some content. Here is a second paragraph still in the excerpt."
```

### 回退到前几段

当未找到分隔标记时，该插件会回退为提取前 N 段：

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is the first paragraph of my article.

This is the second paragraph with more details.

This is the third paragraph.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackParagraphs: 2,
    }),
  ],
});

console.log(result.excerpt);
// "This is the first paragraph of my article. This is the second paragraph with more details."
```

### 回退到字符数限制

将摘要截断到指定的最大字符数，并在单词边界处断开：

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is a long article that goes on and on with lots of content.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackCharacters: 30,
    }),
  ],
});

console.log(result.excerpt);
// "This is a long article that"
```
