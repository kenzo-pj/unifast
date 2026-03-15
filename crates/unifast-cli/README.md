# unifast-cli

[![Crates.io](https://img.shields.io/crates/v/unifast-cli)](https://crates.io/crates/unifast-cli)

Command-line interface for [unifast](https://unifast.dev) — a high-performance Markdown/MDX compiler.

## Install

```sh
cargo install unifast-cli
```

## Usage

```sh
# Compile Markdown to HTML
unifast input.md

# Read from stdin
cat input.md | unifast -

# Write to file
unifast input.md -o output.html

# With GFM and syntax highlighting
unifast input.md --gfm --highlight

# Output as AST
unifast input.md --format mdast

# Extract frontmatter
unifast input.md --frontmatter
```

## Options

| Flag | Default | Description |
|------|---------|-------------|
| `--format <fmt>` | `html` | Output format: `html`, `hast`, `mdast`, `mdx-js` |
| `--input-kind <kind>` | `md` | Input format: `md`, `mdx` |
| `--gfm` | `true` | Enable GFM (tables, task lists, strikethrough, etc.) |
| `--highlight` | `false` | Enable syntax highlighting (Syntect) |
| `--raw-html <policy>` | `disallow` | `disallow`, `allowDangerous`, `parseAndSanitize` |
| `--sanitize` | `true` | Enable HTML sanitization |
| `-o, --output <path>` | stdout | Write output to file |
| `--ast` | | Print MDAST (alias for `--format mdast`) |
| `--frontmatter` | | Print extracted frontmatter as JSON to stderr |
| `--diagnostics` | | Print compilation diagnostics to stderr |
| `--stats` | | Print performance timing to stderr |

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
