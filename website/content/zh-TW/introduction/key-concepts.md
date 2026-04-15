---
title: "核心概念"
description: "了解 unifast 的多階段編譯管線、內建處理階段，以及 MdAst 與 HAst 轉換的運作方式。"
---

unifast 透過多階段管線來編譯 Markdown。理解這些階段有助於您設定編譯器並挑選合適的外掛。

### 編譯管線

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

每一個階段都會透過中介表示法（IR）對文件進行轉換。

### 中介表示法

| IR | 名稱 | 用途 |
|----|------|---------|
| **IR0** | MdAst | Markdown 結構──標題、段落、清單、程式碼區塊 |
| **IR1** | HAst | HTML 結構──元素、屬性、文字節點 |
| **IR2** | JsAst | 僅限 MDX──ESM import 與 JSX 運算式 |

解析器會產生 **MdAst**，接著下降為 **HAst** 以輸出 HTML。MDX 輸入另外還會產生 **JsAst** 節點，用來輸出 JavaScript。

### 處理階段（Passes）

處理階段是在特定階段套用於 AST 的轉換。unifast 針對常見任務提供以下內建處理階段：

**MdAst 處理階段**（下降為 HTML 之前）：

- **Normalize** - 為後續處理階段建立一致的結構
- **Slug** - 從文字內容產生標題 ID
- **TOC** - 從標題擷取目錄
- **Definition Resolution** - 解析連結與圖片的參照定義

**HAst 處理階段**（下降為 HTML 之後）：

- **Sanitize** - 移除不允許的 HTML 元素與屬性
- **Highlight** - 為程式碼區塊套用語法高亮
- **Cleanup** - 移除不必要的節點與空白

處理階段會依據階段排序，您無需擔心執行順序。

### 外掛

外掛是用來設定內建處理階段的 TypeScript 套件。它們並不會在編譯過程中執行任意 JavaScript，而是設定選項以控制 Rust 核心處理文件的方式。

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

每個外掛會傳回一個設定物件，該物件會在 Rust 核心執行之前合併至 compile 選項。如此一來，熱路徑便能完全留在原生程式碼中。

### 輸出格式

編譯器透過 `outputKind` 選項支援多種輸出格式：

| 格式 | 說明 |
|--------|-------------|
| `"html"` | HTML 字串（預設） |
| `"hast"` | HAst JSON──用於自訂渲染的 HTML AST |
| `"mdast"` | MdAst JSON──用於分析的 Markdown AST |
| `"mdx-js"` | JavaScript 模組字串（僅限 MDX） |

### 診斷訊息

編譯器會透過結果中的 `diagnostics` 陣列回報問題。每筆診斷訊息都包含嚴重度層級、訊息內容，以及選用的來源範圍，可用於精確定位錯誤位置。

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
