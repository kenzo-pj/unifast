---
title: "escapeHtml()"
description: "文字列内の HTML 特殊文字をエスケープします。"
---

```ts
import { escapeHtml } from "@unifast/core";
```

## シグネチャ

```ts
function escapeHtml(str: string): string
```

## パラメータ

### str

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `str` | `string` | — | エスケープする文字を含む文字列 |

## 戻り値

`string` — 入力文字列内の `&`、`<`、`>`、`"` が対応する HTML エンティティに置き換えられた文字列です。

## 使い方

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## 使用例

### 基本的なエスケープ

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### ユーザー生成コンテンツのエスケープ

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### 安全な HTML 属性を組み立てる

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## 動作

- **`&`** は `&amp;` に置き換えられる
- **`<`** は `&lt;` に置き換えられる
- **`>`** は `&gt;` に置き換えられる
- **`"`** は `&quot;` に置き換えられる
- その他の文字はそのまま残る
