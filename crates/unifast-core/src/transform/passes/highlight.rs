use crate::ast::common::NodeIdGen;
use crate::ast::hast::nodes::*;
use syntect::html::ClassStyle;
use syntect::parsing::SyntaxSet;

/// Trait for pluggable highlight engines.
pub trait HighlightEngine: Send + Sync {
    fn name(&self) -> &str;
    /// Highlight code and return HTML string with span elements for tokens.
    fn highlight(&self, code: &str, language: &str) -> Option<String>;
}

/// Syntect-based highlighter using CSS classes.
pub struct SyntectHighlighter {
    syntax_set: SyntaxSet,
}

impl SyntectHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
        }
    }

    fn find_syntax(&self, language: &str) -> Option<&syntect::parsing::SyntaxReference> {
        self.syntax_set
            .find_syntax_by_token(language)
            .or_else(|| self.syntax_set.find_syntax_by_name(language))
            .or_else(|| {
                // Fallback aliases not covered by syntect's defaults
                let alias = match language {
                    "ts" | "typescript" | "tsx" => Some("javascript"),
                    "sh" | "zsh" | "fish" => Some("bash"),
                    "yml" => Some("yaml"),
                    "dockerfile" => Some("Dockerfile"),
                    _ => None,
                };
                alias.and_then(|a| {
                    self.syntax_set
                        .find_syntax_by_token(a)
                        .or_else(|| self.syntax_set.find_syntax_by_name(a))
                })
            })
    }
}

impl HighlightEngine for SyntectHighlighter {
    fn name(&self) -> &str {
        "syntect"
    }

    fn highlight(&self, code: &str, language: &str) -> Option<String> {
        let syntax = self.find_syntax(language)?;
        let mut generator = syntect::html::ClassedHTMLGenerator::new_with_class_style(
            syntax,
            &self.syntax_set,
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

/// Apply syntax highlighting to code blocks in the HAST.
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
    code_elem.attributes.insert(
        "class".to_string(),
        format!("{} highlighted", existing_class),
    );
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
        attrs.insert("class".to_string(), format!("language-{}", lang));
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
                    assert!(raw.value.contains("<span"), "expected span elements in: {}", raw.value);
                    assert!(raw.value.contains("sy-"), "expected sy- prefixed classes in: {}", raw.value);
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
        let result = engine.highlight("fn main() { let x = true; }", "rust").unwrap();
        assert!(result.contains("<span"), "expected spans in: {}", result);
        assert!(result.contains("sy-"), "expected sy- prefix in: {}", result);
    }

    #[test]
    fn highlight_javascript_produces_spans() {
        let engine = SyntectHighlighter::new();
        let result = engine.highlight("function foo() { return true; }", "js").unwrap();
        assert!(result.contains("<span"), "expected spans in: {}", result);
    }

    #[test]
    fn highlight_python_produces_spans() {
        let engine = SyntectHighlighter::new();
        let result = engine.highlight("def foo():\n    return None\n", "py").unwrap();
        assert!(result.contains("<span"), "expected spans in: {}", result);
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
        assert!(result.contains("&lt;"), "expected HTML escaping in: {}", result);
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
        // Empty code should produce empty or minimal output
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
}
