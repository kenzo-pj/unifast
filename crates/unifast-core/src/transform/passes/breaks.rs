use crate::ast::common::NodeIdGen;
use crate::ast::mdast::nodes::{Break, MdNode, Text};

pub fn apply_breaks(children: &mut Vec<MdNode>, id_gen: &mut NodeIdGen) {
    let mut i = 0;
    while i < children.len() {
        if let MdNode::Text(text) = &children[i]
            && text.value.contains('\n')
        {
            let span = text.span;
            let parts: Vec<&str> = text.value.split('\n').collect();
            let mut new_nodes = Vec::new();
            for (j, part) in parts.iter().enumerate() {
                if !part.is_empty() {
                    new_nodes.push(MdNode::Text(Text {
                        id: id_gen.next_id(),
                        span,
                        value: part.to_string(),
                    }));
                }
                if j < parts.len() - 1 {
                    new_nodes.push(MdNode::Break(Break {
                        id: id_gen.next_id(),
                        span,
                    }));
                }
            }
            children.splice(i..=i, new_nodes.clone());
            i += new_nodes.len();
            continue;
        }
        if let Some(kids) = children[i].children_mut() {
            apply_breaks(kids, id_gen);
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::Span;

    #[test]
    fn converts_newline_to_break() {
        let mut id_gen = NodeIdGen::new();
        let mut children = vec![MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: "hello\nworld".to_string(),
        })];
        apply_breaks(&mut children, &mut id_gen);
        assert_eq!(children.len(), 3);
        assert!(matches!(&children[0], MdNode::Text(t) if t.value == "hello"));
        assert!(matches!(&children[1], MdNode::Break(_)));
        assert!(matches!(&children[2], MdNode::Text(t) if t.value == "world"));
    }

    #[test]
    fn no_newline_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let mut children = vec![MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".to_string(),
        })];
        apply_breaks(&mut children, &mut id_gen);
        assert_eq!(children.len(), 1);
        assert!(matches!(&children[0], MdNode::Text(t) if t.value == "hello"));
    }

    #[test]
    fn multiple_newlines_produce_multiple_breaks() {
        let mut id_gen = NodeIdGen::new();
        let mut children = vec![MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: "a\n\nb".to_string(),
        })];
        apply_breaks(&mut children, &mut id_gen);
        assert_eq!(children.len(), 4);
        assert!(matches!(&children[0], MdNode::Text(t) if t.value == "a"));
        assert!(matches!(&children[1], MdNode::Break(_)));
        assert!(matches!(&children[2], MdNode::Break(_)));
        assert!(matches!(&children[3], MdNode::Text(t) if t.value == "b"));
    }

    #[test]
    fn trailing_newline() {
        let mut id_gen = NodeIdGen::new();
        let mut children = vec![MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 6),
            value: "hello\n".to_string(),
        })];
        apply_breaks(&mut children, &mut id_gen);
        assert_eq!(children.len(), 2);
        assert!(matches!(&children[0], MdNode::Text(t) if t.value == "hello"));
        assert!(matches!(&children[1], MdNode::Break(_)));
    }
}
