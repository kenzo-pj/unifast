use super::nodes::*;

/// Immutable visitor trait for walking the HTML AST.
#[allow(unused_variables)]
pub trait HVisitor {
    fn visit_node(&mut self, node: &HNode) {
        match node {
            HNode::Root(n) => self.visit_root(n),
            HNode::Element(n) => self.visit_element(n),
            HNode::Text(n) => self.visit_text(n),
            HNode::Comment(n) => self.visit_comment(n),
            HNode::Doctype(n) => self.visit_doctype(n),
            HNode::Raw(n) => self.visit_raw(n),
        }
    }

    fn visit_children(&mut self, children: &[HNode]) {
        for child in children {
            self.visit_node(child);
        }
    }

    fn visit_root(&mut self, node: &HRoot) {
        self.visit_children(&node.children);
    }

    fn visit_element(&mut self, node: &HElement) {
        self.visit_children(&node.children);
    }

    fn visit_text(&mut self, node: &HText) {}

    fn visit_comment(&mut self, node: &HComment) {}

    fn visit_doctype(&mut self, node: &HDoctype) {}

    fn visit_raw(&mut self, node: &HRaw) {}
}

/// Mutable visitor trait for walking and modifying the HTML AST in place.
#[allow(unused_variables)]
pub trait HVisitorMut {
    fn visit_node_mut(&mut self, node: &mut HNode) {
        match node {
            HNode::Root(n) => self.visit_root_mut(n),
            HNode::Element(n) => self.visit_element_mut(n),
            HNode::Text(n) => self.visit_text_mut(n),
            HNode::Comment(n) => self.visit_comment_mut(n),
            HNode::Doctype(n) => self.visit_doctype_mut(n),
            HNode::Raw(n) => self.visit_raw_mut(n),
        }
    }

    fn visit_children_mut(&mut self, children: &mut Vec<HNode>) {
        for child in children.iter_mut() {
            self.visit_node_mut(child);
        }
    }

    fn visit_root_mut(&mut self, node: &mut HRoot) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_element_mut(&mut self, node: &mut HElement) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_text_mut(&mut self, node: &mut HText) {}

    fn visit_comment_mut(&mut self, node: &mut HComment) {}

    fn visit_doctype_mut(&mut self, node: &mut HDoctype) {}

    fn visit_raw_mut(&mut self, node: &mut HRaw) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::util::small_map::SmallMap;

    /// A visitor that counts elements by tag name.
    struct ElementCounter {
        count: usize,
    }

    impl ElementCounter {
        fn new() -> Self {
            Self { count: 0 }
        }
    }

    impl HVisitor for ElementCounter {
        fn visit_node(&mut self, node: &HNode) {
            self.count += 1;
            if let Some(children) = node.children() {
                self.visit_children(children);
            }
        }
    }

    #[test]
    fn counting_visitor() {
        let mut id_gen = NodeIdGen::new();
        let root = HNode::Root(HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::new(0, 50),
                tag: "div".into(),
                attributes: SmallMap::new(),
                children: vec![
                    HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::new(5, 10),
                        value: "Hello".into(),
                    }),
                    HNode::Element(HElement {
                        id: id_gen.next_id(),
                        span: Span::new(10, 40),
                        tag: "span".into(),
                        attributes: SmallMap::new(),
                        children: vec![HNode::Text(HText {
                            id: id_gen.next_id(),
                            span: Span::new(16, 30),
                            value: "World".into(),
                        })],
                        self_closing: false,
                    }),
                ],
                self_closing: false,
            })],
        });

        let mut counter = ElementCounter::new();
        counter.visit_node(&root);
        // root + div + text + span + text = 5
        assert_eq!(counter.count, 5);
    }

    /// A visitor that collects all tag names.
    struct TagCollector {
        tags: Vec<String>,
    }

    impl TagCollector {
        fn new() -> Self {
            Self { tags: vec![] }
        }
    }

    impl HVisitor for TagCollector {
        fn visit_element(&mut self, node: &HElement) {
            self.tags.push(node.tag.clone());
            self.visit_children(&node.children);
        }
    }

    #[test]
    fn tag_collector() {
        let mut id_gen = NodeIdGen::new();
        let root = HNode::Root(HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children: vec![
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::new(0, 50),
                    tag: "h1".into(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::new(50, 100),
                    tag: "p".into(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
            ],
        });

        let mut collector = TagCollector::new();
        collector.visit_node(&root);
        assert_eq!(collector.tags, vec!["h1", "p"]);
    }

    /// A mutable visitor that adds a class attribute to all elements.
    struct ClassAdder {
        class: String,
    }

    impl HVisitorMut for ClassAdder {
        fn visit_element_mut(&mut self, node: &mut HElement) {
            node.attributes.insert("class".into(), self.class.clone());
            self.visit_children_mut(&mut node.children);
        }
    }

    #[test]
    fn mutable_visitor() {
        let mut id_gen = NodeIdGen::new();
        let mut root = HNode::Root(HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 50),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::new(0, 50),
                tag: "div".into(),
                attributes: SmallMap::new(),
                children: vec![],
                self_closing: false,
            })],
        });

        let mut adder = ClassAdder {
            class: "styled".into(),
        };
        adder.visit_node_mut(&mut root);

        if let HNode::Root(r) = &root {
            if let HNode::Element(e) = &r.children[0] {
                assert_eq!(
                    e.attributes.get(&"class".to_string()),
                    Some(&"styled".to_string())
                );
            } else {
                panic!("expected Element");
            }
        } else {
            panic!("expected Root");
        }
    }
}
