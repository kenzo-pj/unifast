use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::{InlineMath, Math as MathNode, MdNode, Text};

pub fn apply_math(children: &mut Vec<MdNode>, id_gen: &mut NodeIdGen) {
    let mut i = 0;
    while i < children.len() {
        let should_convert = if let MdNode::Code(code) = &children[i] {
            code.lang.as_deref() == Some("math")
        } else {
            false
        };
        if should_convert && let MdNode::Code(code) = &children[i] {
            let math_node = MdNode::Math(MathNode {
                id: id_gen.next_id(),
                span: code.span,
                value: code.value.clone(),
                meta: code.meta.clone(),
            });
            children[i] = math_node;
        }
        i += 1;
    }

    let mut i = 0;
    while i < children.len() {
        let should_split = if let MdNode::Text(text) = &children[i] {
            text.value.contains('$')
        } else {
            false
        };
        if should_split && let MdNode::Text(text) = &children[i] {
            let new_nodes = split_inline_math(&text.value, text.span, id_gen);
            if new_nodes.len() > 1 {
                let len = new_nodes.len();
                children.splice(i..=i, new_nodes);
                i += len;
                continue;
            }
        }
        if !matches!(&children[i], MdNode::Code(_) | MdNode::InlineCode(_))
            && let Some(kids) = children[i].children_mut()
        {
            apply_math(kids, id_gen);
        }
        i += 1;
    }
}

pub fn split_inline_math(text: &str, span: Span, id_gen: &mut NodeIdGen) -> Vec<MdNode> {
    let mut nodes = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' && i + 1 < chars.len() && chars[i + 1] == '$' {
            if !current.is_empty() {
                nodes.push(MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span,
                    value: std::mem::take(&mut current),
                }));
            }
            i += 2;
            let mut math_content = String::new();
            while i < chars.len() {
                if chars[i] == '$' && i + 1 < chars.len() && chars[i + 1] == '$' {
                    i += 2;
                    break;
                }
                math_content.push(chars[i]);
                i += 1;
            }
            nodes.push(MdNode::Math(MathNode {
                id: id_gen.next_id(),
                span,
                value: math_content,
                meta: None,
            }));
            continue;
        }

        if chars[i] == '$' && i + 1 < chars.len() && chars[i + 1] != ' ' {
            if !current.is_empty() {
                nodes.push(MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span,
                    value: std::mem::take(&mut current),
                }));
            }
            i += 1;
            let mut math_content = String::new();
            let mut found_end = false;
            while i < chars.len() {
                if chars[i] == '$' {
                    found_end = true;
                    i += 1;
                    break;
                }
                math_content.push(chars[i]);
                i += 1;
            }
            if found_end && !math_content.is_empty() {
                nodes.push(MdNode::InlineMath(InlineMath {
                    id: id_gen.next_id(),
                    span,
                    value: math_content,
                }));
            } else {
                current.push('$');
                current.push_str(&math_content);
            }
            continue;
        }

        current.push(chars[i]);
        i += 1;
    }

    if !current.is_empty() {
        nodes.push(MdNode::Text(Text {
            id: id_gen.next_id(),
            span,
            value: current,
        }));
    }

    if nodes.is_empty() {
        nodes.push(MdNode::Text(Text {
            id: id_gen.next_id(),
            span,
            value: String::new(),
        }));
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_inline_math() {
        let mut id_gen = NodeIdGen::new();
        let nodes = split_inline_math("Hello $x^2$ world", Span::new(0, 20), &mut id_gen);
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[0], MdNode::Text(t) if t.value == "Hello "));
        assert!(matches!(&nodes[1], MdNode::InlineMath(m) if m.value == "x^2"));
        assert!(matches!(&nodes[2], MdNode::Text(t) if t.value == " world"));
    }

    #[test]
    fn handles_display_math_in_text() {
        let mut id_gen = NodeIdGen::new();
        let nodes = split_inline_math("Before $$E=mc^2$$ after", Span::new(0, 30), &mut id_gen);
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[1], MdNode::Math(m) if m.value == "E=mc^2"));
    }

    #[test]
    fn no_math_in_text() {
        let mut id_gen = NodeIdGen::new();
        let nodes = split_inline_math("no math here", Span::new(0, 12), &mut id_gen);
        assert_eq!(nodes.len(), 1);
    }
}
