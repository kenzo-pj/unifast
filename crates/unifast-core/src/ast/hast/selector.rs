use std::borrow::Borrow;
use std::fmt;

use cssparser::ToCss;
use precomputed_hash::PrecomputedHash;
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::context::{MatchingContext, MatchingMode, NeedsSelectorFlags, QuirksMode};
use selectors::matching::{ElementSelectorFlags, matches_selector_list};
use selectors::parser::{NonTSPseudoClass, ParseRelative, PseudoElement, SelectorList};
use selectors::{Element, NthIndexCache, OpaqueElement};

use super::arena::{ArenaNodeData, ArenaNodeRef};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CssString(pub String);

impl From<&str> for CssString {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for CssString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToCss for CssString {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        cssparser::serialize_identifier(&self.0, dest)
    }
}

impl fmt::Display for CssString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PrecomputedHash for CssString {
    fn precomputed_hash(&self) -> u32 {
        fxhash::hash32(&self.0)
    }
}

impl Borrow<str> for CssString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CssLocalName(pub String);

impl From<&str> for CssLocalName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for CssLocalName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToCss for CssLocalName {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        cssparser::serialize_identifier(&self.0, dest)
    }
}

impl fmt::Display for CssLocalName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PrecomputedHash for CssLocalName {
    fn precomputed_hash(&self) -> u32 {
        fxhash::hash32(&self.0)
    }
}

impl Borrow<str> for CssLocalName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HastPseudoClass {}

#[allow(clippy::uninhabited_references)]
impl ToCss for HastPseudoClass {
    fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
        match *self {}
    }
}

#[allow(clippy::uninhabited_references)]
impl NonTSPseudoClass for HastPseudoClass {
    type Impl = HastSelectorImpl;

    fn is_active_or_hover(&self) -> bool {
        match *self {}
    }

    fn is_user_action_state(&self) -> bool {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HastPseudoElement {}

#[allow(clippy::uninhabited_references)]
impl ToCss for HastPseudoElement {
    fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
        match *self {}
    }
}

impl PseudoElement for HastPseudoElement {
    type Impl = HastSelectorImpl;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HastSelectorImpl;

impl selectors::parser::SelectorImpl for HastSelectorImpl {
    type ExtraMatchingData<'a> = ();
    type AttrValue = CssString;
    type Identifier = CssString;
    type LocalName = CssLocalName;
    type NamespaceUrl = CssString;
    type NamespacePrefix = CssString;
    type BorrowedNamespaceUrl = str;
    type BorrowedLocalName = str;
    type NonTSPseudoClass = HastPseudoClass;
    type PseudoElement = HastPseudoElement;
}

struct HastSelectorParser;

impl<'i> selectors::parser::Parser<'i> for HastSelectorParser {
    type Impl = HastSelectorImpl;
    type Error = selectors::parser::SelectorParseErrorKind<'i>;

    fn parse_is_and_where(&self) -> bool {
        true
    }

    fn parse_has(&self) -> bool {
        true
    }
}

pub fn parse_css_selector(input: &str) -> Result<SelectorList<HastSelectorImpl>, String> {
    let mut parser_input = cssparser::ParserInput::new(input);
    let mut parser = cssparser::Parser::new(&mut parser_input);
    SelectorList::parse(&HastSelectorParser, &mut parser, ParseRelative::No)
        .map_err(|e| format!("{e:?}"))
}

pub fn matches_selector(
    selector: &SelectorList<HastSelectorImpl>,
    element: ArenaNodeRef<'_>,
) -> bool {
    let mut cache = NthIndexCache::default();
    let mut context = MatchingContext::new(
        MatchingMode::Normal,
        None,
        &mut cache,
        QuirksMode::NoQuirks,
        NeedsSelectorFlags::No,
        selectors::context::IgnoreNthChildForInvalidation::No,
    );
    matches_selector_list(selector, &element, &mut context)
}

impl Element for ArenaNodeRef<'_> {
    type Impl = HastSelectorImpl;

    #[allow(unsafe_code)]
    fn opaque(&self) -> OpaqueElement {
        let node = self.node();
        OpaqueElement::new(node)
    }

    fn parent_element(&self) -> Option<Self> {
        ArenaNodeRef::parent_element(self)
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        ArenaNodeRef::prev_sibling_element(self)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        ArenaNodeRef::next_sibling_element(self)
    }

    fn first_element_child(&self) -> Option<Self> {
        ArenaNodeRef::first_element_child(self)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        true
    }

    fn has_local_name(&self, local_name: &str) -> bool {
        match &self.node().data {
            ArenaNodeData::Element { tag, .. } => tag == local_name,
            _ => false,
        }
    }

    fn has_namespace(&self, _ns: &str) -> bool {
        true
    }

    fn is_same_type(&self, other: &Self) -> bool {
        match (&self.node().data, &other.node().data) {
            (ArenaNodeData::Element { tag: a, .. }, ArenaNodeData::Element { tag: b, .. }) => {
                a == b
            }
            _ => false,
        }
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&CssString>,
        local_name: &CssLocalName,
        operation: &AttrSelectorOperation<&CssString>,
    ) -> bool {
        match ns {
            NamespaceConstraint::Specific(ns_url) if !ns_url.0.is_empty() => return false,
            _ => {}
        }

        let ArenaNodeData::Element { attributes, .. } = &self.node().data else {
            return false;
        };

        let Some(value) = attributes.get(&local_name.0) else {
            return false;
        };

        operation.eval_str(value)
    }

    #[allow(clippy::uninhabited_references)]
    fn match_non_ts_pseudo_class(
        &self,
        pc: &HastPseudoClass,
        _context: &mut MatchingContext<HastSelectorImpl>,
    ) -> bool {
        match *pc {}
    }

    #[allow(clippy::uninhabited_references)]
    fn match_pseudo_element(
        &self,
        pe: &HastPseudoElement,
        _context: &mut MatchingContext<HastSelectorImpl>,
    ) -> bool {
        match *pe {}
    }

    fn apply_selector_flags(&self, _flags: ElementSelectorFlags) {}

    fn is_link(&self) -> bool {
        match &self.node().data {
            ArenaNodeData::Element {
                tag, attributes, ..
            } => tag == "a" && attributes.contains_key(&"href".to_string()),
            _ => false,
        }
    }

    fn is_html_slot_element(&self) -> bool {
        false
    }

    fn has_id(&self, id: &CssString, case_sensitivity: CaseSensitivity) -> bool {
        let ArenaNodeData::Element { attributes, .. } = &self.node().data else {
            return false;
        };
        let Some(elem_id) = attributes.get(&"id".to_string()) else {
            return false;
        };
        case_sensitivity.eq(elem_id.as_bytes(), id.0.as_bytes())
    }

    fn has_class(&self, name: &CssString, case_sensitivity: CaseSensitivity) -> bool {
        let ArenaNodeData::Element { attributes, .. } = &self.node().data else {
            return false;
        };
        let Some(class_attr) = attributes.get(&"class".to_string()) else {
            return false;
        };
        class_attr
            .split_whitespace()
            .any(|cls| case_sensitivity.eq(cls.as_bytes(), name.0.as_bytes()))
    }

    fn imported_part(&self, _name: &CssString) -> Option<CssString> {
        None
    }

    fn is_part(&self, _name: &CssString) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        let node = self.node();
        let Some(first_child) = node.first_child else {
            return true;
        };
        let mut child = self.arena.node_ref(first_child);
        loop {
            match &child.node().data {
                ArenaNodeData::Element { .. } => return false,
                ArenaNodeData::Text { value } if !value.is_empty() => return false,
                _ => {}
            }
            match child.node().next_sibling {
                Some(next) => child = self.arena.node_ref(next),
                None => return true,
            }
        }
    }

    fn is_root(&self) -> bool {
        match self.node().parent {
            None => true,
            Some(parent_id) => {
                matches!(self.arena.node(parent_id).data, ArenaNodeData::Root)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::ast::hast::arena::HastArena;
    use crate::ast::hast::nodes::*;
    use crate::util::small_map::SmallMap;

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

    fn make_root(id_gen: &mut NodeIdGen, children: Vec<HNode>) -> HRoot {
        HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children,
        }
    }

    #[test]
    fn parse_and_match_tag() {
        let mut id_gen = NodeIdGen::new();
        let h1 = make_element(&mut id_gen, "h1", SmallMap::new(), vec![]);
        let root = make_root(&mut id_gen, vec![h1]);
        let arena = HastArena::from_hroot(&root);

        let sel = parse_css_selector("h1").unwrap();
        let h1_ref = arena.node_ref(arena.root_id()).first_child().unwrap();
        assert!(matches_selector(&sel, h1_ref));

        let sel2 = parse_css_selector("h2").unwrap();
        let h1_ref2 = arena.node_ref(arena.root_id()).first_child().unwrap();
        assert!(!matches_selector(&sel2, h1_ref2));
    }

    #[test]
    fn parse_and_match_class() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), "foo bar".to_string());
        let div = make_element(&mut id_gen, "div", attrs, vec![]);
        let root = make_root(&mut id_gen, vec![div]);
        let arena = HastArena::from_hroot(&root);

        let sel = parse_css_selector(".foo").unwrap();
        let div_ref = arena.node_ref(arena.root_id()).first_child().unwrap();
        assert!(matches_selector(&sel, div_ref));

        let sel2 = parse_css_selector(".baz").unwrap();
        let div_ref2 = arena.node_ref(arena.root_id()).first_child().unwrap();
        assert!(!matches_selector(&sel2, div_ref2));
    }

    #[test]
    fn parse_and_match_descendant() {
        let mut id_gen = NodeIdGen::new();
        let span = make_element(&mut id_gen, "span", SmallMap::new(), vec![]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![span]);
        let root = make_root(&mut id_gen, vec![div]);
        let arena = HastArena::from_hroot(&root);

        let sel = parse_css_selector("div span").unwrap();
        let elements: Vec<_> = arena.elements().collect();
        let span_ref = arena.node_ref(elements[1]);
        assert!(matches_selector(&sel, span_ref));
    }

    #[test]
    fn parse_and_match_child() {
        let mut id_gen = NodeIdGen::new();
        let span = make_element(&mut id_gen, "span", SmallMap::new(), vec![]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![span]);
        let root = make_root(&mut id_gen, vec![div]);
        let arena = HastArena::from_hroot(&root);

        let sel = parse_css_selector("div > span").unwrap();
        let elements: Vec<_> = arena.elements().collect();
        let span_ref = arena.node_ref(elements[1]);
        assert!(matches_selector(&sel, span_ref));
    }
}
