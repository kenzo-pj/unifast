use crate::api::options::RawHtmlPolicy;
use crate::ast::common::{NodeIdGen, Span};
use crate::ast::hast::nodes::{HComment, HElement, HNode, HText};
use crate::diagnostics::sink::DiagnosticSink;
use crate::util::small_map::SmallMap;
use html5ever::parse_fragment;
use html5ever::tendril::TendrilSink;
use html5ever::{QualName, local_name, ns};
use markup5ever_rcdom::{NodeData, RcDom};

pub fn process_raw_html(
    root: &mut crate::ast::hast::nodes::HRoot,
    policy: RawHtmlPolicy,
    id_gen: &mut NodeIdGen,
    diagnostics: &mut DiagnosticSink,
) {
    if policy != RawHtmlPolicy::ParseAndSanitize {
        return;
    }
    expand_raw_nodes(&mut root.children, id_gen, diagnostics);
}

fn expand_raw_nodes(
    children: &mut Vec<HNode>,
    id_gen: &mut NodeIdGen,
    diagnostics: &mut DiagnosticSink,
) {
    let mut i = 0;
    while i < children.len() {
        match &children[i] {
            HNode::Raw(raw) => {
                let span = raw.span;
                let parsed = parse_html_fragment(&raw.value, span, id_gen, diagnostics);
                children.splice(i..=i, parsed.into_iter());
            }
            HNode::Root(_) | HNode::Element(_) => {
                if let Some(kids) = children[i].children_mut() {
                    expand_raw_nodes(kids, id_gen, diagnostics);
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
}

fn parse_html_fragment(
    html: &str,
    span: Span,
    id_gen: &mut NodeIdGen,
    diagnostics: &mut DiagnosticSink,
) -> Vec<HNode> {
    let context_name = QualName::new(None, ns!(html), local_name!("body"));
    let dom = parse_fragment(
        RcDom::default(),
        html5ever::ParseOpts::default(),
        context_name,
        vec![],
        false,
    )
    .one(html);

    let errors = dom.errors.borrow();
    if !errors.is_empty() {
        for err in errors.iter() {
            diagnostics.warn(format!("HTML parse warning: {err}"), span);
        }
    }

    let doc = dom.document;
    collect_fragment_children(&doc, span, id_gen)
}

fn collect_fragment_children(
    node: &markup5ever_rcdom::Node,
    span: Span,
    id_gen: &mut NodeIdGen,
) -> Vec<HNode> {
    let children = node.children.borrow();
    let mut result = Vec::new();
    for child in children.iter() {
        if let NodeData::Element { ref name, .. } = child.data {
            let tag = name.local.as_ref();
            if tag == "html" || tag == "head" || tag == "body" {
                result.extend(collect_fragment_children(child, span, id_gen));
                continue;
            }
        }
        result.extend(convert_node(child, span, id_gen));
    }
    result
}

fn convert_node(
    node: &markup5ever_rcdom::Node,
    span: Span,
    id_gen: &mut NodeIdGen,
) -> Option<HNode> {
    match &node.data {
        NodeData::Text { contents } => {
            let text = contents.borrow().to_string();
            if text.is_empty() {
                return None;
            }
            Some(HNode::Text(HText {
                id: id_gen.next_id(),
                span,
                value: text,
            }))
        }
        NodeData::Element { name, attrs, .. } => {
            let tag = name.local.as_ref().to_string();
            let mut attributes = SmallMap::new();
            for attr in attrs.borrow().iter() {
                attributes.insert(attr.name.local.as_ref().to_string(), attr.value.to_string());
            }
            let self_closing = is_void_element(&tag);
            let children: Vec<HNode> = node
                .children
                .borrow()
                .iter()
                .filter_map(|child| convert_node(child, span, id_gen))
                .collect();
            Some(HNode::Element(HElement {
                id: id_gen.next_id(),
                span,
                tag,
                attributes,
                children,
                self_closing,
            }))
        }
        NodeData::Comment { contents } => Some(HNode::Comment(HComment {
            id: id_gen.next_id(),
            span,
            value: contents.to_string(),
        })),
        NodeData::Document | NodeData::Doctype { .. } | NodeData::ProcessingInstruction { .. } => {
            None
        }
    }
}

fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "source"
            | "track"
            | "wbr"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::hast::nodes::HRoot;

    fn make_root(id_gen: &mut NodeIdGen, children: Vec<HNode>) -> HRoot {
        HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children,
        }
    }

    fn make_raw(id_gen: &mut NodeIdGen, span: Span, value: &str) -> HNode {
        use crate::ast::hast::nodes::HRaw;
        HNode::Raw(HRaw {
            id: id_gen.next_id(),
            span,
            value: value.to_string(),
        })
    }

    #[test]
    fn disallow_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let text = HNode::Text(crate::ast::hast::nodes::HText {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".to_string(),
        });
        let mut root = make_root(&mut id_gen, vec![text]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::Disallow,
            &mut id_gen,
            &mut diagnostics,
        );

        assert!(diagnostics.is_empty());
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn allow_dangerous_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(&mut id_gen, Span::new(0, 10), "<div></div>");
        let mut root = make_root(&mut id_gen, vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::AllowDangerous,
            &mut id_gen,
            &mut diagnostics,
        );

        assert!(diagnostics.is_empty());
        assert_eq!(root.children.len(), 1);
        assert!(matches!(root.children[0], HNode::Raw(_)));
    }

    #[test]
    fn parse_and_sanitize_converts_div() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(&mut id_gen, Span::new(0, 20), "<div>hello</div>");
        let mut root = make_root(&mut id_gen, vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::ParseAndSanitize,
            &mut id_gen,
            &mut diagnostics,
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.tag, "div");
            assert_eq!(elem.children.len(), 1);
            if let HNode::Text(t) = &elem.children[0] {
                assert_eq!(t.value, "hello");
            } else {
                panic!("expected text child");
            }
        } else {
            panic!("expected element, got {:?}", root.children[0]);
        }
    }

    #[test]
    fn parse_and_sanitize_converts_script() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(&mut id_gen, Span::new(0, 30), "<script>alert(1)</script>");
        let mut root = make_root(&mut id_gen, vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::ParseAndSanitize,
            &mut id_gen,
            &mut diagnostics,
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.tag, "script");
        } else {
            panic!("expected script element to be parsed");
        }
    }

    #[test]
    fn parse_and_sanitize_with_attributes() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(
            &mut id_gen,
            Span::new(0, 50),
            r#"<a href="https://example.com" class="link">click</a>"#,
        );
        let mut root = make_root(&mut id_gen, vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::ParseAndSanitize,
            &mut id_gen,
            &mut diagnostics,
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.tag, "a");
            assert_eq!(
                elem.attributes.get("href"),
                Some(&"https://example.com".to_string())
            );
            assert_eq!(elem.attributes.get("class"), Some(&"link".to_string()));
        } else {
            panic!("expected a element");
        }
    }

    #[test]
    fn parse_and_sanitize_multiple_siblings() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(
            &mut id_gen,
            Span::new(0, 40),
            "<em>a</em><strong>b</strong>",
        );
        let mut root = make_root(&mut id_gen, vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::ParseAndSanitize,
            &mut id_gen,
            &mut diagnostics,
        );

        assert_eq!(root.children.len(), 2);
        if let HNode::Element(e) = &root.children[0] {
            assert_eq!(e.tag, "em");
        } else {
            panic!("expected em");
        }
        if let HNode::Element(e) = &root.children[1] {
            assert_eq!(e.tag, "strong");
        } else {
            panic!("expected strong");
        }
    }

    #[test]
    fn parse_and_sanitize_void_element() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(&mut id_gen, Span::new(0, 10), "<br>");
        let mut root = make_root(&mut id_gen, vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::ParseAndSanitize,
            &mut id_gen,
            &mut diagnostics,
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.tag, "br");
            assert!(elem.self_closing);
        } else {
            panic!("expected br element");
        }
    }

    #[test]
    fn parse_and_sanitize_nested_in_element() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(&mut id_gen, Span::new(0, 20), "<b>bold</b>");
        let outer = HNode::Element(crate::ast::hast::nodes::HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            tag: "p".to_string(),
            attributes: SmallMap::new(),
            children: vec![raw],
            self_closing: false,
        });
        let mut root = make_root(&mut id_gen, vec![outer]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::ParseAndSanitize,
            &mut id_gen,
            &mut diagnostics,
        );

        let p = &root.children[0];
        if let HNode::Element(p_elem) = p {
            assert_eq!(p_elem.tag, "p");
            assert_eq!(p_elem.children.len(), 1);
            if let HNode::Element(b_elem) = &p_elem.children[0] {
                assert_eq!(b_elem.tag, "b");
            } else {
                panic!("expected b element");
            }
        } else {
            panic!("expected p element");
        }
    }

    #[test]
    fn parse_plain_text_only() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(&mut id_gen, Span::new(0, 10), "just text");
        let mut root = make_root(&mut id_gen, vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::ParseAndSanitize,
            &mut id_gen,
            &mut diagnostics,
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Text(t) = &root.children[0] {
            assert_eq!(t.value, "just text");
        } else {
            panic!("expected text node");
        }
    }

    #[test]
    fn parse_comment() {
        let mut id_gen = NodeIdGen::new();
        let raw = make_raw(&mut id_gen, Span::new(0, 20), "<!-- a comment -->");
        let mut root = make_root(&mut id_gen, vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(
            &mut root,
            RawHtmlPolicy::ParseAndSanitize,
            &mut id_gen,
            &mut diagnostics,
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Comment(c) = &root.children[0] {
            assert_eq!(c.value, " a comment ");
        } else {
            panic!("expected comment node");
        }
    }
}
