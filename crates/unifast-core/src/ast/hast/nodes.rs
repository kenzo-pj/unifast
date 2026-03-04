use crate::ast::common::{NodeId, Span};
use crate::util::small_map::SmallMap;

/// HTML Abstract Syntax Tree node types.
#[derive(Debug, Clone)]
pub enum HNode {
    Root(HRoot),
    Element(HElement),
    Text(HText),
    Comment(HComment),
    Doctype(HDoctype),
    Raw(HRaw), // for raw HTML passthrough
}

impl HNode {
    /// Returns the source span for any node variant.
    pub fn span(&self) -> Span {
        match self {
            HNode::Root(n) => n.span,
            HNode::Element(n) => n.span,
            HNode::Text(n) => n.span,
            HNode::Comment(n) => n.span,
            HNode::Doctype(n) => n.span,
            HNode::Raw(n) => n.span,
        }
    }

    /// Returns the unique node ID for any node variant.
    pub fn id(&self) -> NodeId {
        match self {
            HNode::Root(n) => n.id,
            HNode::Element(n) => n.id,
            HNode::Text(n) => n.id,
            HNode::Comment(n) => n.id,
            HNode::Doctype(n) => n.id,
            HNode::Raw(n) => n.id,
        }
    }

    /// Returns a slice of children if the node has children, or `None` for leaf nodes.
    pub fn children(&self) -> Option<&[HNode]> {
        match self {
            HNode::Root(n) => Some(&n.children),
            HNode::Element(n) => Some(&n.children),
            HNode::Text(_) | HNode::Comment(_) | HNode::Doctype(_) | HNode::Raw(_) => None,
        }
    }

    /// Returns a mutable reference to the children vec if the node has children.
    pub fn children_mut(&mut self) -> Option<&mut Vec<HNode>> {
        match self {
            HNode::Root(n) => Some(&mut n.children),
            HNode::Element(n) => Some(&mut n.children),
            HNode::Text(_) | HNode::Comment(_) | HNode::Doctype(_) | HNode::Raw(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HRoot {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<HNode>,
}

#[derive(Debug, Clone)]
pub struct HElement {
    pub id: NodeId,
    pub span: Span,
    pub tag: String,
    pub attributes: SmallMap<String, String>, // BTreeMap-backed for stable ordering
    pub children: Vec<HNode>,
    pub self_closing: bool,
}

#[derive(Debug, Clone)]
pub struct HText {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct HComment {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct HDoctype {
    pub id: NodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct HRaw {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeIdGen;

    #[test]
    fn construct_element_with_attributes() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("class".into(), "container".into());
        attrs.insert("id".into(), "main".into());
        attrs.insert("data-value".into(), "42".into());

        let el = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 50),
            tag: "div".into(),
            attributes: attrs,
            children: vec![HNode::Text(HText {
                id: id_gen.next_id(),
                span: Span::new(20, 30),
                value: "Hello".into(),
            })],
            self_closing: false,
        });

        assert_eq!(el.span(), Span::new(0, 50));
        assert_eq!(el.id(), NodeId(0));

        if let HNode::Element(e) = &el {
            assert_eq!(e.tag, "div");
            assert_eq!(e.attributes.len(), 3);
            assert_eq!(
                e.attributes.get(&"class".to_string()),
                Some(&"container".to_string())
            );
            assert_eq!(
                e.attributes.get(&"id".to_string()),
                Some(&"main".to_string())
            );
            assert!(!e.self_closing);
        } else {
            panic!("expected Element");
        }
    }

    #[test]
    fn attribute_stable_ordering() {
        let mut attrs: SmallMap<String, String> = SmallMap::new();
        attrs.insert("z-index".into(), "1".into());
        attrs.insert("class".into(), "box".into());
        attrs.insert("id".into(), "x".into());
        attrs.insert("aria-label".into(), "test".into());

        let keys: Vec<&String> = attrs.iter().map(|(k, _)| k).collect();
        // BTreeMap ordering: alphabetical
        assert_eq!(keys, vec!["aria-label", "class", "id", "z-index"]);
    }

    #[test]
    fn span_and_id_accessors() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(5, 15);

        let nodes: Vec<HNode> = vec![
            HNode::Root(HRoot {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            HNode::Element(HElement {
                id: id_gen.next_id(),
                span,
                tag: "p".into(),
                attributes: SmallMap::new(),
                children: vec![],
                self_closing: false,
            }),
            HNode::Text(HText {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            HNode::Comment(HComment {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            HNode::Doctype(HDoctype {
                id: id_gen.next_id(),
                span,
            }),
            HNode::Raw(HRaw {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
        ];

        for (i, node) in nodes.iter().enumerate() {
            assert_eq!(node.span(), span);
            assert_eq!(node.id(), NodeId(i as u32));
        }
    }

    #[test]
    fn children_accessor() {
        let mut id_gen = NodeIdGen::new();
        let root = HNode::Root(HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children: vec![HNode::Text(HText {
                id: id_gen.next_id(),
                span: Span::new(0, 5),
                value: "hi".into(),
            })],
        });
        assert_eq!(root.children().unwrap().len(), 1);

        let text = HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::new(0, 2),
            value: "hi".into(),
        });
        assert!(text.children().is_none());
    }

    #[test]
    fn children_mut_accessor() {
        let mut id_gen = NodeIdGen::new();
        let mut root = HNode::Root(HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children: vec![],
        });

        root.children_mut().unwrap().push(HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "added".into(),
        }));

        assert_eq!(root.children().unwrap().len(), 1);
    }

    #[test]
    fn self_closing_element() {
        let mut id_gen = NodeIdGen::new();
        let img = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            tag: "img".into(),
            attributes: {
                let mut attrs = SmallMap::new();
                attrs.insert("src".into(), "photo.jpg".into());
                attrs
            },
            children: vec![],
            self_closing: true,
        });

        if let HNode::Element(e) = &img {
            assert!(e.self_closing);
            assert_eq!(e.tag, "img");
            assert_eq!(
                e.attributes.get(&"src".to_string()),
                Some(&"photo.jpg".to_string())
            );
        }
    }
}
