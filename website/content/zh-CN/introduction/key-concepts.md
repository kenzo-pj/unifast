---
title: "核心概念"
description: "了解 unifast 的多阶段编译管线、内置处理阶段以及 MdAst 和 HAst 转换的工作方式。"
---

unifast 通过一条多阶段管线来编译 Markdown。理解这些阶段有助于你配置编译器并选择合适的插件。

### 编译管线

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

每个阶段都会通过一层中间表示（IR）对文档进行转换。

### 中间表示

| IR | 名称 | 用途 |
|----|------|---------|
| **IR0** | MdAst | Markdown 结构 —— 标题、段落、列表、代码块 |
| **IR1** | HAst | HTML 结构 —— 元素、属性、文本节点 |
| **IR2** | JsAst | 仅 MDX 使用 —— ESM 导入和 JSX 表达式 |

解析器首先生成 **MdAst**，再降级为 **HAst** 以便输出 HTML。MDX 输入还会额外生成用于 JavaScript 输出的 **JsAst** 节点。

### 处理阶段

处理阶段是在 AST 的特定阶段上执行的转换。unifast 为常见任务内置了以下处理阶段：

**MdAst 阶段**（降级为 HTML 之前）：

- **Normalize** —— 为下游阶段提供一致的结构
- **Slug** —— 根据文本内容生成标题 ID
- **TOC** —— 从标题中提取目录
- **Definition Resolution** —— 解析链接/图像的引用定义

**HAst 阶段**（降级为 HTML 之后）：

- **Sanitize** —— 移除不允许的 HTML 元素和属性
- **Highlight** —— 为代码块应用语法高亮
- **Cleanup** —— 清理多余的节点和空白字符

处理阶段按照所属阶段自动排序 —— 你无需关心它们的执行顺序。

### 插件

插件是用于配置内置处理阶段的 TypeScript 包。它们不会在编译期间执行任意 JavaScript 代码 —— 而是通过设置选项来控制 Rust 核心处理文档的方式。

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

每个插件都会返回一个配置对象，在 Rust 核心运行之前被合并到编译选项中。这样就能保证热路径完全运行在原生代码中。

### 输出格式

编译器通过 `outputKind` 选项支持多种输出格式：

| 格式 | 描述 |
|--------|-------------|
| `"html"` | HTML 字符串（默认） |
| `"hast"` | HAst JSON —— 用于自定义渲染的 HTML AST |
| `"mdast"` | MdAst JSON —— 用于分析的 Markdown AST |
| `"mdx-js"` | JavaScript 模块字符串（仅 MDX） |

### 诊断信息

编译器会通过结果中的 `diagnostics` 数组报告各类问题。每条诊断都包含严重级别、消息，以及可选的源码位置范围，用于精确定位错误。

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
