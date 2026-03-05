use crate::ast::common::NodeIdGen;
use crate::ast::hast::nodes::*;
use std::sync::LazyLock;
use syntect::html::ClassStyle;
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);

pub trait HighlightEngine: Send + Sync {
    fn name(&self) -> &str;
    fn highlight(&self, code: &str, language: &str) -> Option<String>;
}

pub struct SyntectHighlighter;

impl Default for SyntectHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntectHighlighter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn find_syntax(language: &str) -> Option<&'static syntect::parsing::SyntaxReference> {
        SYNTAX_SET
            .find_syntax_by_token(language)
            .or_else(|| SYNTAX_SET.find_syntax_by_name(language))
            .or_else(|| {
                let alias = match language {
                    "ts" | "typescript" | "tsx" => Some("javascript"),
                    "sh" | "zsh" | "fish" => Some("bash"),
                    "yml" => Some("yaml"),
                    "dockerfile" => Some("Dockerfile"),
                    _ => None,
                };
                alias.and_then(|a| {
                    SYNTAX_SET
                        .find_syntax_by_token(a)
                        .or_else(|| SYNTAX_SET.find_syntax_by_name(a))
                })
            })
    }
}

impl HighlightEngine for SyntectHighlighter {
    fn name(&self) -> &'static str {
        "syntect"
    }

    fn highlight(&self, code: &str, language: &str) -> Option<String> {
        let syntax = Self::find_syntax(language)?;
        let mut generator = syntect::html::ClassedHTMLGenerator::new_with_class_style(
            syntax,
            &SYNTAX_SET,
            ClassStyle::SpacedPrefixed { prefix: "sy-" },
        );
        for line in syntect::util::LinesWithEndings::from(code) {
            generator
                .parse_html_for_line_which_includes_newline(line)
                .ok()?;
        }
        Some(generator.finalize())
    }
}

use std::collections::HashMap;
use tree_sitter_highlight::{
    Highlight, HighlightConfiguration, Highlighter as TsHighlighter, HtmlRenderer,
};

const TS_HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "function",
    "function.builtin",
    "keyword",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

const TS_ATTRS: &[&str] = &[
    "class=\"ts-attribute\"",
    "class=\"ts-comment\"",
    "class=\"ts-constant\"",
    "class=\"ts-constant-builtin\"",
    "class=\"ts-constructor\"",
    "class=\"ts-embedded\"",
    "class=\"ts-function\"",
    "class=\"ts-function-builtin\"",
    "class=\"ts-keyword\"",
    "class=\"ts-number\"",
    "class=\"ts-operator\"",
    "class=\"ts-property\"",
    "class=\"ts-punctuation\"",
    "class=\"ts-punctuation-bracket\"",
    "class=\"ts-punctuation-delimiter\"",
    "class=\"ts-punctuation-special\"",
    "class=\"ts-string\"",
    "class=\"ts-string-special\"",
    "class=\"ts-tag\"",
    "class=\"ts-type\"",
    "class=\"ts-type-builtin\"",
    "class=\"ts-variable\"",
    "class=\"ts-variable-builtin\"",
    "class=\"ts-variable-parameter\"",
];

fn init_ts_configs() -> HashMap<&'static str, HighlightConfiguration> {
    let mut map = HashMap::new();

    macro_rules! register_lang {
        ($name:expr, $lang:expr, $query:expr) => {
            if let Ok(mut config) = HighlightConfiguration::new($lang.into(), $name, $query, "", "")
            {
                config.configure(TS_HIGHLIGHT_NAMES);
                map.insert($name, config);
            }
        };
    }

    register_lang!(
        "typescript",
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        tree_sitter_typescript::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "tsx",
        tree_sitter_typescript::LANGUAGE_TSX,
        tree_sitter_typescript::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "html",
        tree_sitter_html::LANGUAGE,
        tree_sitter_html::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "css",
        tree_sitter_css::LANGUAGE,
        tree_sitter_css::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "python",
        tree_sitter_python::LANGUAGE,
        tree_sitter_python::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "rust",
        tree_sitter_rust::LANGUAGE,
        tree_sitter_rust::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "json",
        tree_sitter_json::LANGUAGE,
        tree_sitter_json::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "yaml",
        tree_sitter_yaml::LANGUAGE,
        tree_sitter_yaml::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "go",
        tree_sitter_go::LANGUAGE,
        tree_sitter_go::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "ruby",
        tree_sitter_ruby::LANGUAGE,
        tree_sitter_ruby::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "toml",
        tree_sitter_toml_ng::LANGUAGE,
        tree_sitter_toml_ng::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "swift",
        tree_sitter_swift::LANGUAGE,
        tree_sitter_swift::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "php",
        tree_sitter_php::LANGUAGE_PHP,
        tree_sitter_php::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "java",
        tree_sitter_java::LANGUAGE,
        tree_sitter_java::HIGHLIGHTS_QUERY
    );

    register_lang!(
        "lua",
        tree_sitter_lua::LANGUAGE,
        tree_sitter_lua::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "scala",
        tree_sitter_scala::LANGUAGE,
        tree_sitter_scala::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "zig",
        tree_sitter_zig::LANGUAGE,
        tree_sitter_zig::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "elixir",
        tree_sitter_elixir::LANGUAGE,
        tree_sitter_elixir::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "ocaml",
        tree_sitter_ocaml::LANGUAGE_OCAML,
        tree_sitter_ocaml::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "r",
        tree_sitter_r::LANGUAGE,
        tree_sitter_r::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "make",
        tree_sitter_make::LANGUAGE,
        tree_sitter_make::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "nix",
        tree_sitter_nix::LANGUAGE,
        tree_sitter_nix::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "regex",
        tree_sitter_regex::LANGUAGE,
        tree_sitter_regex::HIGHLIGHTS_QUERY
    );
    register_lang!(
        "erlang",
        tree_sitter_erlang::LANGUAGE,
        tree_sitter_erlang::HIGHLIGHTS_QUERY
    );

    register_lang!(
        "javascript",
        tree_sitter_javascript::LANGUAGE,
        include_str!("../../../queries/javascript.scm")
    );
    register_lang!(
        "bash",
        tree_sitter_bash::LANGUAGE,
        include_str!("../../../queries/bash.scm")
    );
    register_lang!(
        "c",
        tree_sitter_c::LANGUAGE,
        include_str!("../../../queries/c.scm")
    );
    {
        let combined_cpp_query = format!(
            "{}\n{}",
            include_str!("../../../queries/c.scm"),
            include_str!("../../../queries/cpp.scm")
        );
        if let Ok(mut config) = HighlightConfiguration::new(
            tree_sitter_cpp::LANGUAGE.into(),
            "cpp",
            &combined_cpp_query,
            "",
            "",
        ) {
            config.configure(TS_HIGHLIGHT_NAMES);
            map.insert("cpp", config);
        }
    }
    register_lang!(
        "cmake",
        tree_sitter_cmake::LANGUAGE,
        include_str!("../../../queries/cmake.scm")
    );

    map
}

static TS_CONFIGS: LazyLock<HashMap<&'static str, HighlightConfiguration>> =
    LazyLock::new(init_ts_configs);

fn find_ts_config(language: &str) -> Option<&'static HighlightConfiguration> {
    let canonical = match language {
        "js" => "javascript",
        "jsx" => "tsx",
        "ts" => "typescript",
        "tsx" => "tsx",
        "py" => "python",
        "rs" => "rust",
        "yml" => "yaml",
        "rb" => "ruby",
        "sh" | "zsh" | "fish" | "shell" => "bash",
        "c++" | "cxx" | "cc" => "cpp",
        "ex" | "exs" => "elixir",
        "ml" | "mli" => "ocaml",
        "sc" => "scala",
        "kt" | "kts" => "kotlin",
        "makefile" | "Makefile" | "mk" => "make",
        "nixos" => "nix",
        "erl" | "hrl" => "erlang",
        "Dockerfile" => "dockerfile",
        "CMakeLists.txt" | "CMakeLists" => "cmake",
        lang => lang,
    };
    TS_CONFIGS.get(canonical)
}

pub struct TreeSitterHighlighter;

impl Default for TreeSitterHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeSitterHighlighter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl HighlightEngine for TreeSitterHighlighter {
    fn name(&self) -> &'static str {
        "tree-sitter"
    }

    fn highlight(&self, code: &str, language: &str) -> Option<String> {
        let config = find_ts_config(language)?;
        let mut highlighter = TsHighlighter::new();
        let events = highlighter
            .highlight(config, code.as_bytes(), None, |injection_lang| {
                find_ts_config(injection_lang)
            })
            .ok()?;
        let mut renderer = HtmlRenderer::new();
        renderer
            .render(
                events,
                code.as_bytes(),
                &|highlight: Highlight, output: &mut Vec<u8>| {
                    let attr = TS_ATTRS
                        .get(highlight.0)
                        .copied()
                        .unwrap_or("class=\"ts-unknown\"");
                    output.extend_from_slice(attr.as_bytes());
                },
            )
            .ok()?;
        Some(String::from_utf8_lossy(&renderer.html).into_owned())
    }
}

pub fn apply_highlight(root: &mut HRoot, engine: &dyn HighlightEngine, id_gen: &mut NodeIdGen) {
    apply_to_children(&mut root.children, engine, id_gen);
}

fn apply_to_children(children: &mut [HNode], engine: &dyn HighlightEngine, id_gen: &mut NodeIdGen) {
    for child in children.iter_mut() {
        if let HNode::Element(elem) = child {
            try_highlight_code_block(elem, engine, id_gen);
            apply_to_children(&mut elem.children, engine, id_gen);
        }
    }
}

fn try_highlight_code_block(
    pre_elem: &mut HElement,
    engine: &dyn HighlightEngine,
    id_gen: &mut NodeIdGen,
) {
    if pre_elem.tag != "pre" || pre_elem.children.len() != 1 {
        return;
    }
    let HNode::Element(code_elem) = &mut pre_elem.children[0] else {
        return;
    };
    if code_elem.tag != "code" {
        return;
    }
    let Some(class) = code_elem.attributes.get(&"class".to_string()) else {
        return;
    };
    let class_val = class.clone();
    let Some(lang) = class_val.strip_prefix("language-") else {
        return;
    };
    let text = extract_text_content(&code_elem.children);
    let Some(highlighted) = engine.highlight(&text, lang) else {
        return;
    };
    code_elem.children = vec![HNode::Raw(HRaw {
        id: id_gen.next_id(),
        span: code_elem.span,
        value: highlighted,
    })];
    let existing_class = code_elem
        .attributes
        .get(&"class".to_string())
        .cloned()
        .unwrap_or_default();
    code_elem
        .attributes
        .insert("class".to_string(), format!("{existing_class} highlighted"));
}

fn extract_text_content(nodes: &[HNode]) -> String {
    let mut text = String::new();
    for node in nodes {
        match node {
            HNode::Text(t) => text.push_str(&t.value),
            HNode::Element(e) => text.push_str(&extract_text_content(&e.children)),
            HNode::Raw(r) => text.push_str(&r.value),
            _ => {}
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::Span;
    use crate::util::small_map::SmallMap;

    fn make_element(
        id_gen: &mut NodeIdGen,
        tag: &str,
        attrs: SmallMap<String, String>,
        children: Vec<HNode>,
    ) -> HNode {
        HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            tag: tag.to_string(),
            attributes: attrs,
            children,
            self_closing: false,
        })
    }

    fn make_text(id_gen: &mut NodeIdGen, value: &str) -> HNode {
        HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: value.to_string(),
        })
    }

    fn make_root(id_gen: &mut NodeIdGen, children: Vec<HNode>) -> HRoot {
        HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children,
        }
    }

    fn make_code_block(id_gen: &mut NodeIdGen, lang: &str, code: &str) -> HNode {
        let text = make_text(id_gen, code);
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), format!("language-{lang}"));
        let code_elem = make_element(id_gen, "code", attrs, vec![text]);
        make_element(id_gen, "pre", SmallMap::new(), vec![code_elem])
    }

    #[test]
    fn highlight_fenced_code_with_known_lang() {
        let mut id_gen = NodeIdGen::new();
        let code_block = make_code_block(&mut id_gen, "rust", "fn main() {}");
        let mut root = make_root(&mut id_gen, vec![code_block]);

        let engine = SyntectHighlighter::new();
        let mut highlight_id_gen = NodeIdGen::new();
        apply_highlight(&mut root, &engine, &mut highlight_id_gen);

        if let HNode::Element(ref pre) = root.children[0] {
            if let HNode::Element(ref code_elem) = pre.children[0] {
                assert_eq!(code_elem.children.len(), 1);
                if let HNode::Raw(ref raw) = code_elem.children[0] {
                    assert!(
                        raw.value.contains("<span"),
                        "expected span elements in: {}",
                        raw.value
                    );
                    assert!(
                        raw.value.contains("sy-"),
                        "expected sy- prefixed classes in: {}",
                        raw.value
                    );
                } else {
                    panic!("expected Raw node after highlighting");
                }
                let class = code_elem.attributes.get(&"class".to_string()).unwrap();
                assert!(class.contains("highlighted"));
            } else {
                panic!("expected code element");
            }
        } else {
            panic!("expected pre element");
        }
    }

    #[test]
    fn highlight_unknown_lang_fallback() {
        let mut id_gen = NodeIdGen::new();
        let code_block = make_code_block(&mut id_gen, "zzz_nonexistent_lang", "PERFORM SOMETHING");
        let mut root = make_root(&mut id_gen, vec![code_block]);

        let engine = SyntectHighlighter::new();
        let mut highlight_id_gen = NodeIdGen::new();
        apply_highlight(&mut root, &engine, &mut highlight_id_gen);

        if let HNode::Element(ref pre) = root.children[0] {
            if let HNode::Element(ref code_elem) = pre.children[0] {
                assert_eq!(code_elem.children.len(), 1);
                assert!(matches!(code_elem.children[0], HNode::Text(_)));
                let class = code_elem.attributes.get(&"class".to_string()).unwrap();
                assert!(!class.contains("highlighted"));
            } else {
                panic!("expected code element");
            }
        } else {
            panic!("expected pre element");
        }
    }

    #[test]
    fn highlight_engine_trait() {
        let engine = SyntectHighlighter::new();
        assert_eq!(engine.name(), "syntect");
        assert!(engine.highlight("fn main(){}", "rust").is_some());
        assert!(engine.highlight("code", "zzz_nonexistent_lang").is_none());
    }

    #[test]
    fn highlight_rust_produces_spans() {
        let engine = SyntectHighlighter::new();
        let result = engine
            .highlight("fn main() { let x = true; }", "rust")
            .unwrap();
        assert!(result.contains("<span"), "expected spans in: {result}");
        assert!(result.contains("sy-"), "expected sy- prefix in: {result}");
    }

    #[test]
    fn highlight_javascript_produces_spans() {
        let engine = SyntectHighlighter::new();
        let result = engine
            .highlight("function foo() { return true; }", "js")
            .unwrap();
        assert!(result.contains("<span"), "expected spans in: {result}");
    }

    #[test]
    fn highlight_python_produces_spans() {
        let engine = SyntectHighlighter::new();
        let result = engine
            .highlight("def foo():\n    return None\n", "py")
            .unwrap();
        assert!(result.contains("<span"), "expected spans in: {result}");
    }

    #[test]
    fn highlight_no_code_blocks() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Just a paragraph");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![p]);

        let engine = SyntectHighlighter::new();
        let mut highlight_id_gen = NodeIdGen::new();
        apply_highlight(&mut root, &engine, &mut highlight_id_gen);

        if let HNode::Element(ref p_elem) = root.children[0] {
            assert_eq!(p_elem.tag, "p");
            if let HNode::Text(ref t) = p_elem.children[0] {
                assert_eq!(t.value, "Just a paragraph");
            } else {
                panic!("expected text node");
            }
        } else {
            panic!("expected p element");
        }
    }

    #[test]
    fn highlight_html_is_escaped() {
        let engine = SyntectHighlighter::new();
        let result = engine.highlight("<div>hello</div>", "html").unwrap();
        assert!(
            result.contains("&lt;"),
            "expected HTML escaping in: {result}"
        );
        assert!(!result.contains("<div>"), "raw HTML should be escaped");
    }

    #[test]
    fn highlight_typescript_works() {
        let engine = SyntectHighlighter::new();
        let result = engine.highlight("const x: number = 1;", "ts");
        assert!(result.is_some(), "TypeScript should be recognized");
    }

    #[test]
    fn highlight_lang_aliases() {
        let engine = SyntectHighlighter::new();
        assert!(engine.highlight("fn main(){}", "rs").is_some());
        assert!(engine.highlight("const x=1", "js").is_some());
        assert!(engine.highlight("const x=1", "ts").is_some());
        assert!(engine.highlight("def f(): pass", "py").is_some());
    }

    #[test]
    fn highlight_empty_code() {
        let engine = SyntectHighlighter::new();
        let result = engine.highlight("", "rust").unwrap();
        assert!(!result.contains("error"));
    }

    #[test]
    fn highlight_shell_works() {
        let engine = SyntectHighlighter::new();
        let result = engine.highlight("echo hello", "sh");
        assert!(result.is_some(), "Shell should be recognized");
    }

    #[test]
    fn highlight_css_works() {
        let engine = SyntectHighlighter::new();
        let result = engine.highlight("body { color: red; }", "css");
        assert!(result.is_some(), "CSS should be recognized");
    }

    #[test]
    fn highlight_json_works() {
        let engine = SyntectHighlighter::new();
        let result = engine.highlight("{\"key\": \"value\"}", "json");
        assert!(result.is_some(), "JSON should be recognized");
    }

    #[test]
    fn tree_sitter_engine_name() {
        let engine = TreeSitterHighlighter;
        assert_eq!(engine.name(), "tree-sitter");
    }

    #[test]
    fn tree_sitter_highlight_rust() {
        let engine = TreeSitterHighlighter;
        let result = engine.highlight("fn main() { let x = true; }", "rust");
        assert!(result.is_some(), "Rust should be recognized");
        let html = result.unwrap();
        assert!(html.contains("<span"), "expected span elements in: {html}");
    }

    #[test]
    fn tree_sitter_highlight_unknown_lang() {
        let engine = TreeSitterHighlighter;
        let result = engine.highlight("some code", "zzz_nonexistent_lang");
        assert!(result.is_none(), "Unknown language should return None");
    }

    #[test]
    fn tree_sitter_lang_aliases() {
        let engine = TreeSitterHighlighter;
        assert!(engine.highlight("fn main(){}", "rs").is_some(), "rs alias");
        assert!(
            engine.highlight("const x=1", "js").is_some(),
            "js alias via typescript"
        );
        assert!(engine.highlight("const x=1", "ts").is_some(), "ts alias");
        assert!(
            engine.highlight("def f(): pass", "py").is_some(),
            "py alias"
        );
        assert!(engine.highlight("key: val", "yml").is_some(), "yml alias");
    }

    #[test]
    fn tree_sitter_highlight_empty_code() {
        let engine = TreeSitterHighlighter;
        let result = engine.highlight("", "rust");
        assert!(result.is_some(), "Empty code should succeed");
    }

    #[test]
    fn tree_sitter_all_registered_languages() {
        let engine = TreeSitterHighlighter;

        let cases: &[(&str, &str)] = &[
            ("typescript", "const x: number = 1;"),
            ("tsx", "const App = () => <div/>;"),
            ("html", "<div class=\"a\">hello</div>"),
            ("css", "body { color: red; }"),
            ("python", "def f(x): return x + 1"),
            ("rust", "fn main() { let x = 42; }"),
            ("json", "{\"key\": \"value\"}"),
            ("yaml", "key: value"),
            ("go", "func main() { fmt.Println(1) }"),
            ("ruby", "def hello; puts 'hi'; end"),
            ("toml", "[section]\nkey = \"val\""),
            ("swift", "func greet() { print(\"hi\") }"),
            ("php", "<?php echo 'hello'; ?>"),
            ("java", "public class A { int x = 1; }"),
            ("lua", "local x = 1"),
            ("scala", "val x: Int = 1"),
            ("zig", "const x: u32 = 1;"),
            ("elixir", "defmodule M do end"),
            ("ocaml", "let x = 1"),
            ("r", "x <- 1"),
            ("make", "all:\n\techo hello"),
            ("nix", "{ pkgs }: pkgs.hello"),
            ("regex", "[a-z]+"),
            ("erlang", "-module(m)."),
            ("javascript", "const x = 1;"),
            ("bash", "echo hello"),
            ("c", "int main() { return 0; }"),
            ("cpp", "int main() { return 0; }"),
            ("cmake", "project(test)"),
        ];

        for (lang, code) in cases {
            let result = engine.highlight(code, lang);
            assert!(
                result.is_some(),
                "tree-sitter should recognize language '{lang}'"
            );
            let html = result.unwrap();
            assert!(
                html.contains("<span"),
                "tree-sitter '{}' should produce span elements, got: {}",
                lang,
                &html[..html.len().min(200)]
            );
        }
    }

    #[test]
    fn tree_sitter_all_aliases() {
        let engine = TreeSitterHighlighter;

        let alias_cases: &[(&str, &str, &str)] = &[
            ("sh", "echo hi", "bash alias sh"),
            ("zsh", "echo hi", "bash alias zsh"),
            ("fish", "echo hi", "bash alias fish"),
            ("shell", "echo hi", "bash alias shell"),
            ("c++", "int x=1;", "cpp alias c++"),
            ("cxx", "int x=1;", "cpp alias cxx"),
            ("cc", "int x=1;", "cpp alias cc"),
            ("ex", "defmodule M do end", "elixir alias ex"),
            ("exs", "defmodule M do end", "elixir alias exs"),
            ("ml", "let x = 1", "ocaml alias ml"),
            ("mli", "let x = 1", "ocaml alias mli"),
            ("sc", "val x = 1", "scala alias sc"),
            ("mk", "all:\n\techo hi", "make alias mk"),
            ("makefile", "all:\n\techo hi", "make alias makefile"),
            ("nixos", "{ pkgs }: pkgs.hello", "nix alias nixos"),
            ("erl", "-module(m).", "erlang alias erl"),
            ("hrl", "-module(m).", "erlang alias hrl"),
            ("rb", "puts 'hi'", "ruby alias rb"),
        ];

        for (alias, code, label) in alias_cases {
            assert!(engine.highlight(code, alias).is_some(), "failed: {label}");
        }
    }
}
