use crate::ast::mdast::nodes::MdNode;

pub fn apply_code_import(children: &mut [MdNode], root_dir: Option<&str>) {
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

            if let Ok(content) = std::fs::read_to_string(&resolved) {
                code.value = content;
            }
        }
        if let Some(kids) = child.children_mut() {
            apply_code_import(kids, root_dir);
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
}
