use crate::ast::common::NodeId;
use crate::ast::hast::nodes::{HNode, HRoot};
use crate::util::small_map::SmallMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArenaId(pub u32);

#[derive(Debug, Clone)]
pub enum ArenaNodeData {
    Root,
    Element {
        tag: String,
        attributes: SmallMap<String, String>,
        self_closing: bool,
    },
    Text {
        value: String,
    },
    Comment {
        value: String,
    },
    Doctype,
    Raw {
        value: String,
    },
}

#[derive(Debug, Clone)]
pub struct ArenaNode {
    pub data: ArenaNodeData,
    pub original_id: NodeId,
    pub parent: Option<ArenaId>,
    pub first_child: Option<ArenaId>,
    pub last_child: Option<ArenaId>,
    pub prev_sibling: Option<ArenaId>,
    pub next_sibling: Option<ArenaId>,
}

#[derive(Debug)]
pub struct HastArena {
    nodes: Vec<ArenaNode>,
    root_id: ArenaId,
}

impl HastArena {
    pub fn from_hroot(root: &HRoot) -> Self {
        let mut arena = Self {
            nodes: Vec::new(),
            root_id: ArenaId(0),
        };

        let root_id = arena.alloc(ArenaNode {
            data: ArenaNodeData::Root,
            original_id: root.id,
            parent: None,
            first_child: None,
            last_child: None,
            prev_sibling: None,
            next_sibling: None,
        });
        arena.root_id = root_id;
        arena.add_children(root_id, &root.children);
        arena
    }

    pub fn from_hnode(node: &HNode) -> Self {
        match node {
            HNode::Root(root) => Self::from_hroot(root),
            _ => {
                let mut arena = Self {
                    nodes: Vec::new(),
                    root_id: ArenaId(0),
                };
                let root_id = arena.add_node(node, None);
                arena.root_id = root_id;
                arena
            }
        }
    }

    fn alloc(&mut self, node: ArenaNode) -> ArenaId {
        let id = ArenaId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }

    fn add_node(&mut self, node: &HNode, parent: Option<ArenaId>) -> ArenaId {
        match node {
            HNode::Root(root) => {
                let id = self.alloc(ArenaNode {
                    data: ArenaNodeData::Root,
                    original_id: root.id,
                    parent,
                    first_child: None,
                    last_child: None,
                    prev_sibling: None,
                    next_sibling: None,
                });
                self.add_children(id, &root.children);
                id
            }
            HNode::Element(elem) => {
                let id = self.alloc(ArenaNode {
                    data: ArenaNodeData::Element {
                        tag: elem.tag.clone(),
                        attributes: elem.attributes.clone(),
                        self_closing: elem.self_closing,
                    },
                    original_id: elem.id,
                    parent,
                    first_child: None,
                    last_child: None,
                    prev_sibling: None,
                    next_sibling: None,
                });
                self.add_children(id, &elem.children);
                id
            }
            HNode::Text(t) => self.alloc(ArenaNode {
                data: ArenaNodeData::Text {
                    value: t.value.clone(),
                },
                original_id: t.id,
                parent,
                first_child: None,
                last_child: None,
                prev_sibling: None,
                next_sibling: None,
            }),
            HNode::Comment(c) => self.alloc(ArenaNode {
                data: ArenaNodeData::Comment {
                    value: c.value.clone(),
                },
                original_id: c.id,
                parent,
                first_child: None,
                last_child: None,
                prev_sibling: None,
                next_sibling: None,
            }),
            HNode::Doctype(d) => self.alloc(ArenaNode {
                data: ArenaNodeData::Doctype,
                original_id: d.id,
                parent,
                first_child: None,
                last_child: None,
                prev_sibling: None,
                next_sibling: None,
            }),
            HNode::Raw(r) => self.alloc(ArenaNode {
                data: ArenaNodeData::Raw {
                    value: r.value.clone(),
                },
                original_id: r.id,
                parent,
                first_child: None,
                last_child: None,
                prev_sibling: None,
                next_sibling: None,
            }),
        }
    }

    fn add_children(&mut self, parent_id: ArenaId, children: &[HNode]) {
        let mut prev: Option<ArenaId> = None;
        let mut first: Option<ArenaId> = None;

        for child in children {
            let child_id = self.add_node(child, Some(parent_id));
            if first.is_none() {
                first = Some(child_id);
            }
            if let Some(prev_id) = prev {
                self.nodes[prev_id.0 as usize].next_sibling = Some(child_id);
                self.nodes[child_id.0 as usize].prev_sibling = Some(prev_id);
            }
            prev = Some(child_id);
        }

        self.nodes[parent_id.0 as usize].first_child = first;
        self.nodes[parent_id.0 as usize].last_child = prev;
    }

    pub const fn root_id(&self) -> ArenaId {
        self.root_id
    }

    pub fn node(&self, id: ArenaId) -> &ArenaNode {
        &self.nodes[id.0 as usize]
    }

    pub const fn node_ref(&self, id: ArenaId) -> ArenaNodeRef<'_> {
        ArenaNodeRef { arena: self, id }
    }

    pub fn elements(&self) -> impl Iterator<Item = ArenaId> + '_ {
        self.nodes.iter().enumerate().filter_map(|(i, node)| {
            if matches!(node.data, ArenaNodeData::Element { .. }) {
                Some(ArenaId(i as u32))
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct ArenaNodeRef<'a> {
    pub arena: &'a HastArena,
    pub id: ArenaId,
}

impl<'a> ArenaNodeRef<'a> {
    pub fn node(&self) -> &'a ArenaNode {
        self.arena.node(self.id)
    }

    pub fn is_element(&self) -> bool {
        matches!(self.node().data, ArenaNodeData::Element { .. })
    }

    pub fn is_root(&self) -> bool {
        matches!(self.node().data, ArenaNodeData::Root)
    }

    pub fn parent(&self) -> Option<Self> {
        self.node().parent.map(|id| self.arena.node_ref(id))
    }

    pub fn prev_sibling(&self) -> Option<Self> {
        self.node().prev_sibling.map(|id| self.arena.node_ref(id))
    }

    pub fn next_sibling(&self) -> Option<Self> {
        self.node().next_sibling.map(|id| self.arena.node_ref(id))
    }

    pub fn first_child(&self) -> Option<Self> {
        self.node().first_child.map(|id| self.arena.node_ref(id))
    }

    pub fn last_child(&self) -> Option<Self> {
        self.node().last_child.map(|id| self.arena.node_ref(id))
    }

    pub fn parent_element(&self) -> Option<Self> {
        let mut current = self.parent()?;
        loop {
            if current.is_element() {
                return Some(current);
            }
            if current.is_root() {
                return None;
            }
            current = current.parent()?;
        }
    }

    pub fn prev_sibling_element(&self) -> Option<Self> {
        let mut current = self.prev_sibling()?;
        loop {
            if current.is_element() {
                return Some(current);
            }
            current = current.prev_sibling()?;
        }
    }

    pub fn next_sibling_element(&self) -> Option<Self> {
        let mut current = self.next_sibling()?;
        loop {
            if current.is_element() {
                return Some(current);
            }
            current = current.next_sibling()?;
        }
    }

    pub fn first_element_child(&self) -> Option<Self> {
        let mut current = self.first_child()?;
        loop {
            if current.is_element() {
                return Some(current);
            }
            current = current.next_sibling()?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::ast::hast::nodes::*;

    fn make_element(
        id_gen: &mut NodeIdGen,
        tag: &str,
        attrs: SmallMap<String, String>,
        children: Vec<HNode>,
    ) -> HNode {
        HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::empty(),
            tag: tag.to_string(),
            attributes: attrs,
            children,
            self_closing: false,
        })
    }

    fn make_text(id_gen: &mut NodeIdGen, value: &str) -> HNode {
        HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::empty(),
            value: value.to_string(),
        })
    }

    fn make_root(id_gen: &mut NodeIdGen, children: Vec<HNode>) -> HRoot {
        HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children,
        }
    }

    #[test]
    fn arena_from_root() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "hello");
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![text]);
        let root = make_root(&mut id_gen, vec![div]);
        let arena = HastArena::from_hroot(&root);

        assert!(matches!(
            arena.node(arena.root_id()).data,
            ArenaNodeData::Root
        ));
        let root_ref = arena.node_ref(arena.root_id());
        let first = root_ref.first_child().unwrap();
        assert!(first.is_element());
    }

    #[test]
    fn sibling_navigation() {
        let mut id_gen = NodeIdGen::new();
        let a = make_element(&mut id_gen, "a", SmallMap::new(), vec![]);
        let b = make_element(&mut id_gen, "b", SmallMap::new(), vec![]);
        let c = make_element(&mut id_gen, "c", SmallMap::new(), vec![]);
        let root = make_root(&mut id_gen, vec![a, b, c]);
        let arena = HastArena::from_hroot(&root);

        let root_ref = arena.node_ref(arena.root_id());
        let first = root_ref.first_element_child().unwrap();
        assert!(matches!(
            &first.node().data,
            ArenaNodeData::Element { tag, .. } if tag == "a"
        ));

        let second = first.next_sibling_element().unwrap();
        assert!(matches!(
            &second.node().data,
            ArenaNodeData::Element { tag, .. } if tag == "b"
        ));

        let third = second.next_sibling_element().unwrap();
        assert!(matches!(
            &third.node().data,
            ArenaNodeData::Element { tag, .. } if tag == "c"
        ));

        assert!(third.next_sibling_element().is_none());

        let back = third.prev_sibling_element().unwrap();
        assert!(matches!(
            &back.node().data,
            ArenaNodeData::Element { tag, .. } if tag == "b"
        ));
    }

    #[test]
    fn parent_element_skips_root() {
        let mut id_gen = NodeIdGen::new();
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![]);
        let root = make_root(&mut id_gen, vec![div]);
        let arena = HastArena::from_hroot(&root);

        let root_ref = arena.node_ref(arena.root_id());
        let div_ref = root_ref.first_child().unwrap();
        assert!(div_ref.parent_element().is_none());
    }

    #[test]
    fn elements_iterator() {
        let mut id_gen = NodeIdGen::new();
        let span = make_element(&mut id_gen, "span", SmallMap::new(), vec![]);
        let text = make_text(&mut id_gen, "hi");
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![span, text]);
        let root = make_root(&mut id_gen, vec![div]);
        let arena = HastArena::from_hroot(&root);

        assert_eq!(arena.elements().count(), 2);
    }
}
