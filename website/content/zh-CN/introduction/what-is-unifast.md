---
title: "什么是 unifast？"
description: "unifast 是一款基于 Rust 核心的高性能 Markdown 与 MDX 编译器，内置 GFM、净化、高亮、目录等处理阶段。"
---

unifast 是一款基于 Rust 核心的高性能 Markdown 与 MDX 编译器。它通过内置处理阶段直接实现了 remark/rehype 的主流使用场景，而不是通过 JS 插件兼容层来实现。

### 为什么选择 unifast？

unified/remark/rehype 等传统 Markdown 工具链虽然功能强大，但也存在一些折衷：

- **性能开销** —— 多次 JS AST 转换会不断累积，在大规模场景下尤为明显。
- **插件协作** —— 数十个插件之间的执行顺序、兼容性和功能重叠都需要协调。
- **缺少内置能力** —— 即便是 GFM 或净化这类基础功能，也需要额外安装独立的包。

unifast 采用了不同的思路：

- **Rust 核心** —— 解析、转换和输出全部在原生代码中完成。
- **内置处理阶段** —— 常用特性（GFM、净化、高亮、目录）作为内置能力直接提供，而非外挂叠加。
- **一次编译** —— 一次函数调用即可完成从 Markdown 到 HTML 的全部特性处理。

### 核心特性

| 特性 | 描述 |
|---------|-------------|
| **CommonMark + GFM** | 表格、任务列表、删除线、自动链接、脚注 |
| **Frontmatter** | 支持 YAML、TOML 和 JSON 元数据提取 |
| **MDX** | 在 Markdown 中使用 JSX 表达式和 import |
| **诊断信息** | 精确的错误范围，带有行列位置映射 |

### 内置处理阶段

常用的 remark/rehype 插件被重新实现为原生 Rust 处理阶段。无需 npm install，也不用为执行顺序发愁。

| 处理阶段 | 描述 |
|------|-------------|
| **净化** | 基于 schema 的 HTML 允许列表，提供安全默认值 |
| **语法高亮** | 可插拔的引擎（syntect、Shiki） |
| **目录** | 自动提取标题生成目录树 |

### 平台支持

unifast 基于单一的 Rust 核心，可运行在多个平台上：

- **`@unifast/node`** —— 通过 N-API（napi-rs）提供的 Node.js 绑定。主要目标平台。
- **`@unifast/core`** —— 在所有包之间共享的 TypeScript 类型定义。
- **`unifast`（CLI）** —— 面向脚本和 CI 的命令行接口。
- **WASM** —— 浏览器和边缘运行时支持（次要目标平台）。

### 非目标

unifast **并非** unified 的直接替代品。它不提供以下能力：

- 在核心中执行现有的 remark/rehype JS 插件。
- 与 unified 生态系统保持 API 兼容。
- 在核心编译路径中依赖 Node 的模块解析机制。

与之相对，unifast 追求的是**使用场景的完备性** —— 覆盖绝大多数项目的实际需求，而不必费心组装一条插件流水线。
