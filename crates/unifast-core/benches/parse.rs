use unifast_core::parse::{parse_markdown, parse_mdx_input};

fn main() {
    divan::main();
}

#[divan::bench]
fn parse_markdown_simple(bencher: divan::Bencher<'_, '_>) {
    let input = "# Hello\n\nParagraph with **bold** and *italic*.\n\n- a\n- b\n- c\n";
    bencher.bench_local(|| parse_markdown(divan::black_box(input)));
}

#[divan::bench]
fn parse_gfm_table(bencher: divan::Bencher<'_, '_>) {
    let input = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |\n| 7 | 8 | 9 |\n";
    bencher.bench_local(|| parse_markdown(divan::black_box(input)));
}

#[divan::bench]
fn parse_mdx(bencher: divan::Bencher<'_, '_>) {
    let input = r#"import { Button } from './components'

export const meta = { title: "Hello" }

# Hello MDX

<Button variant="primary">Click me</Button>

Some text with {expression + 1} inline.
"#;
    bencher.bench_local(|| parse_mdx_input(divan::black_box(input)));
}

#[divan::bench]
fn parse_frontmatter(bencher: divan::Bencher<'_, '_>) {
    let input =
        "---\ntitle: Hello\nauthor: World\ntags:\n  - one\n  - two\n---\n\n# Content\n\nBody.\n";
    bencher.bench_local(|| parse_markdown(divan::black_box(input)));
}
