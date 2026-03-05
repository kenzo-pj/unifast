use crate::ast::mdast::nodes::MdNode;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct CodeImportPass;

impl Pass for CodeImportPass {
    fn name(&self) -> &'static str {
        "code_import"
    }
    fn phase(&self) -> Phase {
        Phase::Transform
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.code_import.enabled {
            return Ok(());
        }
        let root_dir = ctx.options.code_import.root_dir.as_deref();
        match ast {
            AstPayload::Mdast(doc) | AstPayload::Both { mdast: doc, .. } => {
                apply_code_import(&mut doc.children, root_dir);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn apply_code_import(children: &mut [MdNode], root_dir: Option<&str>) {
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

fn extract_file_path(meta: &str) -> Option<String> {
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
    fn metadata() {
        let pass = CodeImportPass;
        assert_eq!(pass.name(), "code_import");
        assert_eq!(pass.phase(), Phase::Transform);
    }

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
