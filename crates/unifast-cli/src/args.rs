use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "unifast", about = "High-performance Markdown/MDX compiler")]
pub struct Cli {
    /// Input file path (use - for stdin)
    pub input: PathBuf,

    /// Output format
    #[arg(long, default_value = "html")]
    pub format: String,

    /// Input kind (md or mdx)
    #[arg(long, default_value = "md")]
    pub input_kind: String,

    /// Enable GFM extensions
    #[arg(long, default_value = "true")]
    pub gfm: bool,

    /// Enable syntax highlighting
    #[arg(long)]
    pub highlight: bool,

    /// Raw HTML policy (disallow, allowDangerous, parseAndSanitize)
    #[arg(long, default_value = "disallow")]
    pub raw_html: String,

    /// Enable sanitization
    #[arg(long, default_value = "true")]
    pub sanitize: bool,

    /// Output file (default: stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Print AST instead of rendered output
    #[arg(long)]
    pub ast: bool,

    /// Print frontmatter as JSON
    #[arg(long)]
    pub frontmatter: bool,

    /// Print diagnostics
    #[arg(long)]
    pub diagnostics: bool,

    /// Print stats
    #[arg(long)]
    pub stats: bool,
}
