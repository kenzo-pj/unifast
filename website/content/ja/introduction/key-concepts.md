---
title: "主要な概念"
description: "unifast のマルチステージなコンパイルパイプライン、組み込みパス、そして MdAst と HAst の変換のしくみを理解します。"
---

unifast は、マルチステージなパイプラインを通じて Markdown をコンパイルします。これらのステージを理解することで、コンパイラの設定や適切なプラグインの選択がしやすくなります。

### コンパイルパイプライン

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

各ステージでは、中間表現 (IR) を通じてドキュメントが変換されます。

### 中間表現

| IR | 名前 | 目的 |
|----|------|---------|
| **IR0** | MdAst | Markdown の構造 - 見出し、段落、リスト、コードブロック |
| **IR1** | HAst | HTML の構造 - 要素、属性、テキストノード |
| **IR2** | JsAst | MDX のみ - ESM の import と JSX 式 |

パーサーは **MdAst** を生成し、それを **HAst** に降ろして HTML を出力します。MDX 入力の場合は、JavaScript 出力用に **JsAst** ノードも追加で生成されます。

### パス

パスは、特定のフェーズで AST に適用される変換です。unifast には、よく使われるタスクのための組み込みパスが用意されています。

**MdAst パス** (HTML への降ろし込み前)：

- **Normalize** - 後段のパスのために構造を一貫させる
- **Slug** - テキストから見出し ID を生成する
- **TOC** - 見出しから目次を抽出する
- **Definition Resolution** - リンク/画像の参照定義を解決する

**HAst パス** (HTML への降ろし込み後)：

- **Sanitize** - 許可されていない HTML 要素や属性を除去する
- **Highlight** - コードブロックにシンタックスハイライトを適用する
- **Cleanup** - 不要なノードや空白を除去する

パスはフェーズごとに順序付けられているため、実行順序を気にする必要はありません。

### プラグイン

プラグインは、組み込みパスを設定するための TypeScript パッケージです。コンパイル中に任意の JavaScript を実行するわけではなく、Rust コアがドキュメントをどのように処理するかを制御するオプションを設定します。

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

各プラグインは設定オブジェクトを返し、Rust コアが実行される前にコンパイルオプションへマージされます。これによりホットパスを完全にネイティブコード内に保てます。

### 出力フォーマット

コンパイラは `outputKind` オプションを通じて複数の出力フォーマットをサポートしています。

| フォーマット | 説明 |
|--------|-------------|
| `"html"` | HTML 文字列 (デフォルト) |
| `"hast"` | HAst JSON - カスタムレンダリング用の HTML AST |
| `"mdast"` | MdAst JSON - 解析用の Markdown AST |
| `"mdx-js"` | JavaScript モジュール文字列 (MDX のみ) |

### 診断

コンパイラは、結果の `diagnostics` 配列を通じて問題を報告します。各診断には重大度レベル、メッセージ、正確なエラー位置を示すソース範囲 (任意) が含まれます。

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
