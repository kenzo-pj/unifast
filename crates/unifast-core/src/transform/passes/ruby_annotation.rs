use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::{MdNode, RubyAnnotation as RubyAnnotationNode, Text};

pub fn apply_ruby(children: &mut Vec<MdNode>, id_gen: &mut NodeIdGen) {
    let mut i = 0;
    while i < children.len() {
        let should_split = if let MdNode::Text(text) = &children[i] {
            text.value.contains('{') && text.value.contains('|') && text.value.contains('}')
        } else {
            false
        };
        if should_split && let MdNode::Text(text) = &children[i] {
            let new_nodes = split_ruby(&text.value, text.span, id_gen);
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
            apply_ruby(kids, id_gen);
        }
        i += 1;
    }
}

pub fn split_ruby(text: &str, span: Span, id_gen: &mut NodeIdGen) -> Vec<MdNode> {
    let mut nodes = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{' {
            let mut j = i + 1;
            let mut base = String::new();
            let mut annotation = String::new();
            let mut found_pipe = false;
            let mut found_end = false;

            while j < chars.len() {
                if chars[j] == '|' && !found_pipe {
                    found_pipe = true;
                    j += 1;
                    continue;
                }
                if chars[j] == '}' && found_pipe {
                    found_end = true;
                    j += 1;
                    break;
                }
                if found_pipe {
                    annotation.push(chars[j]);
                } else {
                    base.push(chars[j]);
                }
                j += 1;
            }

            if found_end && !base.is_empty() && !annotation.is_empty() {
                if !current.is_empty() {
                    nodes.push(MdNode::Text(Text {
                        id: id_gen.next_id(),
                        span,
                        value: std::mem::take(&mut current),
                    }));
                }
                nodes.push(MdNode::RubyAnnotation(RubyAnnotationNode {
                    id: id_gen.next_id(),
                    span,
                    base,
                    annotation,
                }));
                i = j;
                continue;
            }
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
    fn splits_ruby_annotation() {
        let mut id_gen = NodeIdGen::new();
        let nodes = split_ruby("Hello {漢字|かんじ} world", Span::new(0, 30), &mut id_gen);
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[0], MdNode::Text(t) if t.value == "Hello "));
        if let MdNode::RubyAnnotation(r) = &nodes[1] {
            assert_eq!(r.base, "漢字");
            assert_eq!(r.annotation, "かんじ");
        } else {
            panic!("expected RubyAnnotation");
        }
        assert!(matches!(&nodes[2], MdNode::Text(t) if t.value == " world"));
    }

    #[test]
    fn no_ruby_in_text() {
        let mut id_gen = NodeIdGen::new();
        let nodes = split_ruby("no ruby here", Span::new(0, 12), &mut id_gen);
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn incomplete_ruby_kept_as_text() {
        let mut id_gen = NodeIdGen::new();
        let nodes = split_ruby("test {broken", Span::new(0, 12), &mut id_gen);
        assert_eq!(nodes.len(), 1);
        assert!(matches!(&nodes[0], MdNode::Text(t) if t.value == "test {broken"));
    }
}
