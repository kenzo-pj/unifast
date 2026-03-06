mod args;

use args::Cli;
use clap::Parser;
use std::fs;
use std::io::{self, Read};
use unifast_core::api::compile::compile;
use unifast_core::api::options::{
    CompileOptions, GfmOptions, HighlightEngine, HighlightOptions, InputKind, OutputKind,
    RawHtmlPolicy, SanitizeOptions,
};
use unifast_core::api::result::Output;

fn main() {
    let cli = Cli::parse();

    let input = if cli.input.to_str() == Some("-") {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("Failed to read stdin");
        buf
    } else {
        fs::read_to_string(&cli.input).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", cli.input.display(), e);
            std::process::exit(1);
        })
    };

    let opts = CompileOptions {
        input_kind: match cli.input_kind.as_str() {
            "mdx" => InputKind::Mdx,
            _ => InputKind::Markdown,
        },
        output_kind: match cli.format.as_str() {
            "hast" => OutputKind::Hast,
            "mdast" => OutputKind::Mdast,
            "mdxJs" | "mdx-js" => OutputKind::MdxJs,
            _ => OutputKind::Html,
        },
        gfm: if cli.gfm {
            GfmOptions::default()
        } else {
            GfmOptions {
                tables: false,
                task_list: false,
                strikethrough: false,
                footnotes: false,
                autolink: false,
            }
        },
        highlight: HighlightOptions {
            enabled: cli.highlight,
            engine: if cli.highlight {
                HighlightEngine::Syntect
            } else {
                HighlightEngine::None
            },
        },
        raw_html: match cli.raw_html.as_str() {
            "allowDangerous" => RawHtmlPolicy::AllowDangerous,
            "parseAndSanitize" => RawHtmlPolicy::ParseAndSanitize,
            _ => RawHtmlPolicy::Disallow,
        },
        sanitize: SanitizeOptions {
            enabled: cli.sanitize,
            schema: None,
        },
        ..Default::default()
    };

    let result = compile(&input, &opts);

    let output_str = match &result.output {
        Output::Html(html) => html.clone(),
        Output::MdxJs { code, .. } => code.clone(),
        Output::Hast(root) => format!("{root:#?}"),
        Output::Mdast(doc) => format!("{doc:#?}"),
    };

    if let Some(ref path) = cli.output {
        fs::write(path, &output_str).expect("Failed to write output");
    } else {
        print!("{output_str}");
    }

    if cli.frontmatter {
        eprintln!("\n--- Frontmatter ---");
        eprintln!(
            "{}",
            serde_json::to_string_pretty(&result.frontmatter).unwrap_or_default()
        );
    }

    if cli.diagnostics && !result.diagnostics.is_empty() {
        let line_index = unifast_core::util::line_index::LineIndex::new(&input);
        for d in &result.diagnostics {
            let _pos = line_index.line_col(d.span.start);
            let _len = (d.span.end - d.span.start).max(1) as usize;
            let severity = if d.level == unifast_core::diagnostics::diagnostic::DiagLevel::Error {
                miette::Severity::Error
            } else {
                miette::Severity::Warning
            };
            let report = miette::miette!(
                severity = severity,
                labels = vec![miette::LabeledSpan::at(
                    d.span.start as usize..d.span.end as usize,
                    &d.message
                )],
                "{}",
                d.message
            )
            .with_source_code(miette::NamedSource::new(
                cli.input.display().to_string(),
                input.clone(),
            ));
            eprintln!("{report:?}");
        }
    }

    if cli.stats {
        eprintln!("\n--- Stats ---");
        eprintln!("Parse:     {:.2}ms", result.stats.parse_ms);
        eprintln!("Transform: {:.2}ms", result.stats.transform_ms);
        eprintln!("Emit:      {:.2}ms", result.stats.emit_ms);
    }
}
