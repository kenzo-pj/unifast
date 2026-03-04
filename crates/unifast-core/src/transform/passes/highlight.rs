use crate::ast::common::NodeIdGen;
use crate::ast::hast::nodes::*;

/// Trait for pluggable highlight engines.
pub trait HighlightEngine: Send + Sync {
    fn name(&self) -> &str;
    /// Highlight code and return HTML string with span elements for tokens.
    fn highlight(&self, code: &str, language: &str) -> Option<String>;
}

/// Simple builtin highlighter that adds language class and wraps code.
/// For a real implementation, this would use syntect or tree-sitter.
pub struct BuiltinHighlighter;

impl HighlightEngine for BuiltinHighlighter {
    fn name(&self) -> &str {
        "builtin"
    }

    fn highlight(&self, code: &str, language: &str) -> Option<String> {
        match language {
            "rust" | "rs" => Some(highlight_rust(code)),
            "javascript" | "js" => Some(highlight_javascript(code)),
            "typescript" | "ts" => Some(highlight_typescript(code)),
            "python" | "py" => Some(highlight_python(code)),
            "html" => Some(highlight_generic(code)),
            _ => None,
        }
    }
}

/// Apply syntax highlighting to code blocks in the HAst.
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

// Language-specific highlight functions

fn highlight_rust(code: &str) -> String {
    let keywords = [
        "fn", "let", "mut", "pub", "use", "struct", "enum", "impl", "trait", "for", "while",
        "loop", "if", "else", "match", "return", "self", "mod", "crate", "super", "where", "type",
        "const", "static", "async", "await", "move", "ref", "true", "false",
    ];
    simple_keyword_highlight(code, &keywords)
}

fn highlight_javascript(code: &str) -> String {
    let keywords = [
        "function", "const", "let", "var", "return", "if", "else", "for", "while", "class", "new",
        "this", "import", "export", "from", "async", "await", "try", "catch", "throw", "true",
        "false", "null",
    ];
    simple_keyword_highlight(code, &keywords)
}

fn highlight_typescript(code: &str) -> String {
    highlight_javascript(code)
}

fn highlight_python(code: &str) -> String {
    let keywords = [
        "def", "class", "return", "if", "elif", "else", "for", "while", "import", "from", "try",
        "except", "raise", "with", "as", "True", "False", "None", "self", "lambda", "yield",
    ];
    simple_keyword_highlight(code, &keywords)
}

fn highlight_generic(code: &str) -> String {
    escape_for_html(code)
}

fn simple_keyword_highlight(code: &str, keywords: &[&str]) -> String {
    let mut result = String::new();
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let ch = bytes[i];

        // String literals
        if ch == b'"' || ch == b'\'' {
            let quote = ch;
            let mut s = String::new();
            s.push(quote as char);
            i += 1;
            while i < len {
                let c = bytes[i];
                s.push(c as char);
                i += 1;
                if c == quote {
                    break;
                }
                if c == b'\\' && i < len {
                    s.push(bytes[i] as char);
                    i += 1;
                }
            }
            result.push_str(&format!(
                "<span class=\"hljs-string\">{}</span>",
                escape_for_html(&s)
            ));
            continue;
        }

        // Line comments (// style)
        if ch == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            let start = i;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            result.push_str(&format!(
                "<span class=\"hljs-comment\">{}</span>",
                escape_for_html(&code[start..i])
            ));
            continue;
        }

        // Hash comments (# style - for Python)
        if ch == b'#' {
            let start = i;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            result.push_str(&format!(
                "<span class=\"hljs-comment\">{}</span>",
                escape_for_html(&code[start..i])
            ));
            continue;
        }

        // Identifiers/keywords
        if ch.is_ascii_alphabetic() || ch == b'_' {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &code[start..i];
            if keywords.contains(&word) {
                result.push_str(&format!(
                    "<span class=\"hljs-keyword\">{}</span>",
                    escape_for_html(word)
                ));
            } else {
                result.push_str(&escape_for_html(word));
            }
            continue;
        }

        // Numbers
        if ch.is_ascii_digit() {
            let start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.' || bytes[i] == b'_')
            {
                i += 1;
            }
            result.push_str(&format!(
                "<span class=\"hljs-number\">{}</span>",
                escape_for_html(&code[start..i])
            ));
            continue;
        }

        // Default: escape and emit single character
        result.push_str(&escape_for_html(&(ch as char).to_string()));
        i += 1;
    }

    result
}

fn escape_for_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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

        let engine = BuiltinHighlighter;
        let mut highlight_id_gen = NodeIdGen::new();
        apply_highlight(&mut root, &engine, &mut highlight_id_gen);

        // The code element should now contain a Raw node with span elements
        if let HNode::Element(ref pre) = root.children[0] {
            if let HNode::Element(ref code_elem) = pre.children[0] {
                assert_eq!(code_elem.children.len(), 1);
                if let HNode::Raw(ref raw) = code_elem.children[0] {
                    assert!(raw.value.contains("<span class=\"hljs-keyword\">fn</span>"));
                    assert!(raw.value.contains("main"));
                } else {
                    panic!("expected Raw node after highlighting");
                }
                // Check the highlighted class was added
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
        let code_block = make_code_block(&mut id_gen, "cobol", "PERFORM SOMETHING");
        let mut root = make_root(&mut id_gen, vec![code_block]);

        let engine = BuiltinHighlighter;
        let mut highlight_id_gen = NodeIdGen::new();
        apply_highlight(&mut root, &engine, &mut highlight_id_gen);

        // Should remain unchanged (Text node, not Raw)
        if let HNode::Element(ref pre) = root.children[0] {
            if let HNode::Element(ref code_elem) = pre.children[0] {
                assert_eq!(code_elem.children.len(), 1);
                assert!(matches!(code_elem.children[0], HNode::Text(_)));
                // No "highlighted" class added
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
        let engine = BuiltinHighlighter;
        assert_eq!(engine.name(), "builtin");
        // Known language returns Some
        assert!(engine.highlight("let x = 1;", "rust").is_some());
        // Unknown language returns None
        assert!(engine.highlight("code", "brainfuck").is_none());
    }

    #[test]
    fn highlight_rust_keywords() {
        let result = highlight_rust("fn main() { let x = true; }");
        assert!(result.contains("<span class=\"hljs-keyword\">fn</span>"));
        assert!(result.contains("<span class=\"hljs-keyword\">let</span>"));
        assert!(result.contains("<span class=\"hljs-keyword\">true</span>"));
        assert!(result.contains("main"));
        // main is not a keyword
        assert!(!result.contains("<span class=\"hljs-keyword\">main</span>"));
    }

    #[test]
    fn highlight_string_literals() {
        let result = highlight_rust("let s = \"hello\";");
        assert!(result.contains("<span class=\"hljs-string\">"));
        assert!(result.contains("hello"));
    }

    #[test]
    fn highlight_no_code_blocks() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Just a paragraph");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![p]);

        let engine = BuiltinHighlighter;
        let mut highlight_id_gen = NodeIdGen::new();
        // Should not crash
        apply_highlight(&mut root, &engine, &mut highlight_id_gen);

        // Paragraph unchanged
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
    fn highlight_comments_detected() {
        let result = highlight_rust("// a comment\nfn main() {}");
        assert!(result.contains("<span class=\"hljs-comment\">// a comment</span>"));
    }

    #[test]
    fn highlight_numbers_detected() {
        let result = highlight_rust("let x = 42;");
        assert!(result.contains("<span class=\"hljs-number\">42</span>"));
    }

    #[test]
    fn highlight_javascript_keywords() {
        let result = highlight_javascript("function foo() { return true; }");
        assert!(result.contains("<span class=\"hljs-keyword\">function</span>"));
        assert!(result.contains("<span class=\"hljs-keyword\">return</span>"));
        assert!(result.contains("<span class=\"hljs-keyword\">true</span>"));
    }

    #[test]
    fn highlight_python_keywords() {
        let result = highlight_python("def foo():\n    return None");
        assert!(result.contains("<span class=\"hljs-keyword\">def</span>"));
        assert!(result.contains("<span class=\"hljs-keyword\">return</span>"));
        assert!(result.contains("<span class=\"hljs-keyword\">None</span>"));
    }

    #[test]
    fn highlight_python_comments() {
        let result = highlight_python("# a comment\ndef foo(): pass");
        assert!(result.contains("<span class=\"hljs-comment\"># a comment</span>"));
    }

    #[test]
    fn highlight_html_escapes() {
        let result = highlight_generic("<div>hello</div>");
        assert!(result.contains("&lt;div&gt;"));
        assert!(result.contains("&lt;/div&gt;"));
    }

    #[test]
    fn highlight_typescript_same_as_js() {
        let js = highlight_javascript("const x = 1;");
        let ts = highlight_typescript("const x = 1;");
        assert_eq!(js, ts);
    }

    #[test]
    fn highlight_escaped_string_literals() {
        let result = highlight_rust(r#"let s = "hello \"world\"";"#);
        assert!(result.contains("<span class=\"hljs-string\">"));
    }

    #[test]
    fn escape_for_html_basic() {
        assert_eq!(escape_for_html("<>&\""), "&lt;&gt;&amp;&quot;");
    }

    #[test]
    fn highlight_empty_code() {
        let result = highlight_rust("");
        assert_eq!(result, "");
    }

    #[test]
    fn highlight_lang_aliases() {
        let engine = BuiltinHighlighter;
        assert!(engine.highlight("fn main(){}", "rs").is_some());
        assert!(engine.highlight("const x=1", "js").is_some());
        assert!(engine.highlight("const x=1", "ts").is_some());
        assert!(engine.highlight("def f(): pass", "py").is_some());
    }
}
