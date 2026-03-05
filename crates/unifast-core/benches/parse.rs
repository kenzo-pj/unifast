use std::hint::black_box;
use std::time::Instant;
use unifast_core::parse::{parse_markdown, parse_mdx_input};

fn bench_parse_markdown_simple(iterations: u32) {
    let input = "# Hello\n\nParagraph with **bold** and *italic*.\n\n- a\n- b\n- c\n";
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(parse_markdown(black_box(input)));
    }
    let elapsed = start.elapsed();
    println!(
        "parse_markdown_simple: {} iterations in {:?} ({:.2} µs/iter)",
        iterations,
        elapsed,
        elapsed.as_micros() as f64 / f64::from(iterations)
    );
}

fn bench_parse_gfm_table(iterations: u32) {
    let input = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |\n| 7 | 8 | 9 |\n";
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(parse_markdown(black_box(input)));
    }
    let elapsed = start.elapsed();
    println!(
        "parse_gfm_table: {} iterations in {:?} ({:.2} µs/iter)",
        iterations,
        elapsed,
        elapsed.as_micros() as f64 / f64::from(iterations)
    );
}

fn bench_parse_mdx(iterations: u32) {
    let input = r#"import { Button } from './components'

export const meta = { title: "Hello" }

# Hello MDX

<Button variant="primary">Click me</Button>

Some text with {expression + 1} inline.
"#;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(parse_mdx_input(black_box(input)));
    }
    let elapsed = start.elapsed();
    println!(
        "parse_mdx: {} iterations in {:?} ({:.2} µs/iter)",
        iterations,
        elapsed,
        elapsed.as_micros() as f64 / f64::from(iterations)
    );
}

fn bench_parse_frontmatter(iterations: u32) {
    let input =
        "---\ntitle: Hello\nauthor: World\ntags:\n  - one\n  - two\n---\n\n# Content\n\nBody.\n";
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(parse_markdown(black_box(input)));
    }
    let elapsed = start.elapsed();
    println!(
        "parse_frontmatter: {} iterations in {:?} ({:.2} µs/iter)",
        iterations,
        elapsed,
        elapsed.as_micros() as f64 / f64::from(iterations)
    );
}

fn main() {
    println!("=== unifast parse benchmarks ===\n");
    bench_parse_markdown_simple(10_000);
    bench_parse_gfm_table(10_000);
    bench_parse_mdx(10_000);
    bench_parse_frontmatter(10_000);
    println!("\nDone.");
}
