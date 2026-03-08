use crate::ast::mdast::nodes::MdNode;

pub fn remove_comments(children: &mut Vec<MdNode>) {
    children.retain(|node| !matches!(node, MdNode::Html(h) if is_comment(&h.value)));
    for child in children.iter_mut() {
        if let Some(kids) = child.children_mut() {
            remove_comments(kids);
        }
    }
}

fn is_comment(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.starts_with("<!--") && trimmed.ends_with("-->")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeId, Span};
    use crate::ast::mdast::nodes::*;

    fn html_node(value: &str) -> MdNode {
        MdNode::Html(Html {
            id: NodeId(0),
            span: Span::empty(),
            value: value.to_string(),
        })
    }

    fn text_node(value: &str) -> MdNode {
        MdNode::Text(Text {
            id: NodeId(0),
            span: Span::empty(),
            value: value.to_string(),
        })
    }

    fn paragraph(children: Vec<MdNode>) -> MdNode {
        MdNode::Paragraph(Paragraph {
            id: NodeId(0),
            span: Span::empty(),
            children,
        })
    }

    #[test]
    fn removes_comment_nodes() {
        let mut children = vec![
            paragraph(vec![text_node("Hello")]),
            html_node("<!-- this is a comment -->"),
            paragraph(vec![text_node("World")]),
        ];
        remove_comments(&mut children);
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn keeps_non_comment_html() {
        let mut children = vec![
            html_node("<div>not a comment</div>"),
            html_node("<!-- a comment -->"),
        ];
        remove_comments(&mut children);
        assert_eq!(children.len(), 1);
        assert!(matches!(&children[0], MdNode::Html(h) if h.value.contains("div")));
    }

    #[test]
    fn handles_whitespace_in_comment() {
        let mut children = vec![html_node("  <!-- padded -->  ")];
        remove_comments(&mut children);
        assert!(children.is_empty());
    }

    #[test]
    fn removes_nested_comments() {
        let mut children = vec![MdNode::Blockquote(Blockquote {
            id: NodeId(0),
            span: Span::empty(),
            children: vec![
                paragraph(vec![text_node("quote")]),
                html_node("<!-- nested comment -->"),
            ],
            alert_type: None,
        })];
        remove_comments(&mut children);
        assert_eq!(children.len(), 1);
        if let MdNode::Blockquote(bq) = &children[0] {
            assert_eq!(bq.children.len(), 1);
        } else {
            panic!("expected Blockquote");
        }
    }

    #[test]
    fn empty_children() {
        let mut children: Vec<MdNode> = vec![];
        remove_comments(&mut children);
        assert!(children.is_empty());
    }

    #[test]
    fn multiline_comment() {
        let mut children = vec![html_node("<!--\nmultiline\ncomment\n-->")];
        remove_comments(&mut children);
        assert!(children.is_empty());
    }
}
