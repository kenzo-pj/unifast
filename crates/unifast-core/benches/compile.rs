use std::hint::black_box;
use std::time::Instant;
use unifast_core::api::compile::compile;
use unifast_core::api::options::{
    CompileOptions, FrontmatterOptions, GfmOptions, HighlightEngine, HighlightOptions,
};

fn bench_compile_simple(iterations: u32) {
    let input = "# Hello World\n\nThis is a paragraph with **bold** and *italic* text.\n\n- Item 1\n- Item 2\n- Item 3\n";
    let opts = CompileOptions::default();
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(compile(black_box(input), black_box(&opts)));
    }
    let elapsed = start.elapsed();
    println!(
        "compile_simple: {} iterations in {:?} ({:.2} µs/iter)",
        iterations,
        elapsed,
        elapsed.as_micros() as f64 / f64::from(iterations)
    );
}

fn bench_compile_complex(iterations: u32) {
    let input = r#"---
title: Benchmark Document
author: unifast
---

# Main Heading

This is a paragraph with **bold**, *italic*, ~~strikethrough~~, and `inline code`.

## Code Block

```rust
fn main() {
    println!("Hello, world!");
    let x = 42;
    for i in 0..x {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}
```

## Table

| Column A | Column B | Column C |
|----------|:--------:|---------:|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |

## Lists

1. First item
   - Nested bullet
   - Another nested
2. Second item
3. Third item

## Links and Images

[Link text](https://example.com "Title")

![Alt text](image.png)

> Blockquote with **formatting** inside.
>
> Multiple paragraphs.

---

Footnote reference[^1].

[^1]: This is a footnote.

- [x] Task done
- [ ] Task pending
"#;

    let opts = CompileOptions {
        gfm: GfmOptions::default(),
        frontmatter: FrontmatterOptions {
            yaml: true,
            ..Default::default()
        },
        highlight: HighlightOptions {
            enabled: true,
            engine: HighlightEngine::Syntect,
        },
        ..Default::default()
    };

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(compile(black_box(input), black_box(&opts)));
    }
    let elapsed = start.elapsed();
    println!(
        "compile_complex: {} iterations in {:?} ({:.2} µs/iter)",
        iterations,
        elapsed,
        elapsed.as_micros() as f64 / f64::from(iterations)
    );
}

fn bench_compile_large(iterations: u32) {
    let mut input = String::with_capacity(50_000);
    for i in 0..100 {
        input.push_str(&format!("## Section {i}\n\n"));
        input.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit. ");
        input.push_str("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\n");
        input.push_str("- Item A with **bold**\n");
        input.push_str("- Item B with *italic*\n");
        input.push_str("- Item C with `code`\n\n");
        if i % 10 == 0 {
            input.push_str("```js\nconst x = 42;\nconsole.log(x);\n```\n\n");
        }
    }

    let opts = CompileOptions {
        gfm: GfmOptions::default(),
        highlight: HighlightOptions {
            enabled: true,
            engine: HighlightEngine::Syntect,
        },
        ..Default::default()
    };

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(compile(black_box(&input), black_box(&opts)));
    }
    let elapsed = start.elapsed();
    println!(
        "compile_large (100 sections): {} iterations in {:?} ({:.2} µs/iter)",
        iterations,
        elapsed,
        elapsed.as_micros() as f64 / f64::from(iterations)
    );
}

fn main() {
    println!("=== unifast compile benchmarks ===\n");
    bench_compile_simple(10_000);
    bench_compile_complex(5_000);
    bench_compile_large(100);
    println!("\nDone.");
}
