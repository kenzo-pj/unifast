use super::nodes::{HComment, HDoctype, HElement, HNode, HRaw, HRoot, HText};
use crate::ast::common::{NodeIdGen, Span};
use crate::util::small_map::SmallMap;

pub struct HBuilder<'a> {
    id_gen: &'a mut NodeIdGen,
}

impl<'a> HBuilder<'a> {
    pub const fn new(id_gen: &'a mut NodeIdGen) -> Self {
        Self { id_gen }
    }

    pub const fn root(&mut self, span: Span, children: Vec<HNode>) -> HNode {
        HNode::Root(HRoot {
            id: self.id_gen.next_id(),
            span,
            children,
        })
    }

    pub fn element(
        &mut self,
        span: Span,
        tag: impl Into<String>,
        attributes: SmallMap<String, String>,
        children: Vec<HNode>,
        self_closing: bool,
    ) -> HNode {
        HNode::Element(HElement {
            id: self.id_gen.next_id(),
            span,
            tag: tag.into(),
            attributes,
            children,
            self_closing,
        })
    }

    pub fn elem(&mut self, span: Span, tag: impl Into<String>, children: Vec<HNode>) -> HNode {
        self.element(span, tag, SmallMap::new(), children, false)
    }

    pub fn text(&mut self, span: Span, value: impl Into<String>) -> HNode {
        HNode::Text(HText {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub fn comment(&mut self, span: Span, value: impl Into<String>) -> HNode {
        HNode::Comment(HComment {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub const fn doctype(&mut self, span: Span) -> HNode {
        HNode::Doctype(HDoctype {
            id: self.id_gen.next_id(),
            span,
        })
    }

    pub fn raw(&mut self, span: Span, value: impl Into<String>) -> HNode {
        HNode::Raw(HRaw {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeId;

    #[test]
    fn build_element_tree() {
        let mut id_gen = NodeIdGen::new();
        let mut b = HBuilder::new(&mut id_gen);

        let text = b.text(Span::new(5, 10), "Hello");
        let inner = b.elem(Span::new(3, 12), "span", vec![text]);
        let outer = b.elem(Span::new(0, 20), "div", vec![inner]);
        let root = b.root(Span::new(0, 20), vec![outer]);

        assert_eq!(root.id(), NodeId(3));
        assert_eq!(root.span(), Span::new(0, 20));

        let root_children = root.children().unwrap();
        assert_eq!(root_children.len(), 1);

        if let HNode::Element(div) = &root_children[0] {
            assert_eq!(div.tag, "div");
            assert_eq!(div.children.len(), 1);
            if let HNode::Element(span) = &div.children[0] {
                assert_eq!(span.tag, "span");
                assert_eq!(span.children.len(), 1);
            } else {
                panic!("expected span Element");
            }
        } else {
            panic!("expected div Element");
        }
    }

    #[test]
    fn build_element_with_attributes() {
        let mut id_gen = NodeIdGen::new();
        let mut b = HBuilder::new(&mut id_gen);

        let mut attrs = SmallMap::new();
        attrs.insert("href".into(), "https://example.com".into());
        attrs.insert("target".into(), "_blank".into());

        let link_text = b.text(Span::new(5, 10), "click");
        let link = b.element(Span::new(0, 30), "a", attrs, vec![link_text], false);

        if let HNode::Element(e) = &link {
            assert_eq!(e.tag, "a");
            assert_eq!(
                e.attributes.get("href"),
                Some(&"https://example.com".to_string())
            );
            assert_eq!(e.attributes.get("target"), Some(&"_blank".to_string()));
            assert!(!e.self_closing);
        } else {
            panic!("expected Element");
        }
    }

    #[test]
    fn build_self_closing_element() {
        let mut id_gen = NodeIdGen::new();
        let mut b = HBuilder::new(&mut id_gen);

        let mut attrs = SmallMap::new();
        attrs.insert("src".into(), "img.png".into());
        let img = b.element(Span::new(0, 20), "img", attrs, vec![], true);

        if let HNode::Element(e) = &img {
            assert!(e.self_closing);
            assert!(e.children.is_empty());
        } else {
            panic!("expected Element");
        }
    }

    #[test]
    fn build_comment_and_doctype() {
        let mut id_gen = NodeIdGen::new();
        let mut b = HBuilder::new(&mut id_gen);

        let dt = b.doctype(Span::new(0, 15));
        assert_eq!(dt.id(), NodeId(0));
        assert!(dt.children().is_none());

        let comment = b.comment(Span::new(15, 40), "a comment");
        if let HNode::Comment(c) = &comment {
            assert_eq!(c.value, "a comment");
        } else {
            panic!("expected Comment");
        }
    }

    #[test]
    fn builder_assigns_sequential_ids() {
        let mut id_gen = NodeIdGen::new();
        let mut b = HBuilder::new(&mut id_gen);

        let n0 = b.text(Span::empty(), "a");
        let n1 = b.text(Span::empty(), "b");
        let n2 = b.text(Span::empty(), "c");

        assert_eq!(n0.id(), NodeId(0));
        assert_eq!(n1.id(), NodeId(1));
        assert_eq!(n2.id(), NodeId(2));
    }
}
