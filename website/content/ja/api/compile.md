---
title: compile()
---

## compile(input, options?)

MarkdownまたはMDXをHTMLにコンパイルします。

### パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `input` | `string` | MarkdownまたはMDXのソース |
| `options` | `CompileOptions` | コンパイルオプション |

### 戻り値

`CompileResult` の内容:

- `output` - コンパイル済みのHTML文字列
- `frontmatter` - パースされたYAML/TOML/JSONメタデータ
- `diagnostics` - 警告とエラー
- `stats` - タイミング情報
