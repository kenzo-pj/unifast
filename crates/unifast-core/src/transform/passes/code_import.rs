use crate::ast::mdast::nodes::MdNode;
use crate::diagnostics::sink::DiagnosticSink;

pub fn apply_code_import(
    children: &mut [MdNode],
    root_dir: Option<&str>,
    diagnostics: &mut DiagnosticSink,
) {
    for child in children.iter_mut() {
        if let MdNode::Code(code) = child
            && let Some(ref meta) = code.meta
            && let Some(file_path) = extract_file_path(meta)
        {
            let resolved = if let Some(root) = root_dir {
                if file_path.starts_with('/') {
                    file_path.clone()
                } else {
                    format!("{}/{}", root.trim_end_matches('/'), file_path)
                }
            } else {
                file_path.clone()
            };

            match std::fs::read_to_string(&resolved) {
                Ok(content) => code.value = content,
                Err(e) => {
                    diagnostics.warn(
                        format!("code-import: failed to read '{resolved}': {e}"),
                        code.span,
                    );
                }
            }
        }
        if let Some(kids) = child.children_mut() {
            apply_code_import(kids, root_dir, diagnostics);
        }
    }
}

pub fn extract_file_path(meta: &str) -> Option<String> {
    for part in meta.split_whitespace() {
        if let Some(path) = part.strip_prefix("file=") {
            let path = path.trim_matches('"').trim_matches('\'');
            if !path.is_empty() {
                return Some(path.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_file_path() {
        assert_eq!(
            extract_file_path("file=./example.rs"),
            Some("./example.rs".to_string())
        );
        assert_eq!(
            extract_file_path("file=\"./src/main.rs\""),
            Some("./src/main.rs".to_string())
        );
        assert_eq!(
            extract_file_path("title=\"test\" file=./lib.rs"),
            Some("./lib.rs".to_string())
        );
        assert_eq!(extract_file_path("no file here"), None);
    }

    #[test]
    fn empty_file_value_returns_none() {
        assert_eq!(extract_file_path("file="), None);
        assert_eq!(extract_file_path("file=\"\""), None);
        assert_eq!(extract_file_path("file=''"), None);
    }

    #[test]
    fn single_quoted_path() {
        assert_eq!(
            extract_file_path("file='./src/main.rs'"),
            Some("./src/main.rs".to_string())
        );
    }

    #[test]
    fn apply_reads_existing_file() {
        use crate::ast::common::{NodeIdGen, Span};
        use crate::ast::mdast::nodes::{Code, MdNode};
        use std::io::Write;

        let dir = std::env::temp_dir().join("unifast_test_code_import");
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("sample.txt");
        let mut f = std::fs::File::create(&file_path).unwrap();
        write!(f, "imported content").unwrap();

        let mut id_gen = NodeIdGen::new();
        let mut diag = DiagnosticSink::new();
        let meta = format!("file={}", file_path.display());
        let mut children = vec![MdNode::Code(Code {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: "placeholder".to_string(),
            lang: Some("rust".to_string()),
            meta: Some(meta),
        })];
        apply_code_import(&mut children, None, &mut diag);
        if let MdNode::Code(code) = &children[0] {
            assert_eq!(code.value, "imported content");
        } else {
            panic!("expected code node");
        }
        assert!(diag.is_empty());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn apply_with_root_dir_resolves_relative() {
        use crate::ast::common::{NodeIdGen, Span};
        use crate::ast::mdast::nodes::{Code, MdNode};
        use std::io::Write;

        let dir = std::env::temp_dir().join("unifast_test_code_import_root");
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("example.rs");
        let mut f = std::fs::File::create(&file_path).unwrap();
        write!(f, "fn main() {{}}").unwrap();

        let mut id_gen = NodeIdGen::new();
        let mut diag = DiagnosticSink::new();
        let mut children = vec![MdNode::Code(Code {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: String::new(),
            lang: Some("rust".to_string()),
            meta: Some("file=example.rs".to_string()),
        })];
        apply_code_import(&mut children, Some(dir.to_str().unwrap()), &mut diag);
        if let MdNode::Code(code) = &children[0] {
            assert_eq!(code.value, "fn main() {}");
        } else {
            panic!("expected code node");
        }
        assert!(diag.is_empty());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn apply_missing_file_emits_warning() {
        use crate::ast::common::{NodeIdGen, Span};
        use crate::ast::mdast::nodes::{Code, MdNode};

        let mut id_gen = NodeIdGen::new();
        let mut diag = DiagnosticSink::new();
        let mut children = vec![MdNode::Code(Code {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: "original".to_string(),
            lang: Some("rust".to_string()),
            meta: Some("file=/nonexistent/path.rs".to_string()),
        })];
        apply_code_import(&mut children, None, &mut diag);
        if let MdNode::Code(code) = &children[0] {
            assert_eq!(code.value, "original");
        } else {
            panic!("expected code node");
        }
        assert!(!diag.is_empty(), "should emit warning for missing file");
    }
}
