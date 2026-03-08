use crate::ast::hast::nodes::*;
use crate::diagnostics::sink::DiagnosticSink;
use std::collections::{HashMap, HashSet};

pub struct SanitizeSchema {
    pub allowed_tags: HashSet<String>,
    pub allowed_attributes: HashMap<String, HashSet<String>>,
    pub allowed_protocols: HashMap<String, HashSet<String>>,
}

#[must_use]
pub fn default_safe_schema() -> SanitizeSchema {
    SanitizeSchema {
        allowed_tags: [
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "p",
            "br",
            "hr",
            "ul",
            "ol",
            "li",
            "blockquote",
            "pre",
            "code",
            "em",
            "strong",
            "del",
            "s",
            "a",
            "img",
            "table",
            "thead",
            "tbody",
            "tr",
            "th",
            "td",
            "sup",
            "sub",
            "div",
            "span",
            "input",
            "dl",
            "dt",
            "dd",
            "ruby",
            "rt",
            "rp",
            "svg",
            "path",
            "figure",
            "figcaption",
            "abbr",
        ]
        .iter()
        .map(std::string::ToString::to_string)
        .collect(),
        allowed_attributes: {
            let mut map = HashMap::new();
            map.insert(
                "a".to_string(),
                ["href", "title", "class"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "img".to_string(),
                ["src", "alt", "title", "loading", "decoding"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "td".to_string(),
                ["align"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "th".to_string(),
                ["align"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "input".to_string(),
                ["type", "checked", "disabled"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "code".to_string(),
                ["class"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "ol".to_string(),
                ["start"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "pre".to_string(),
                ["data-meta", "data-lang", "data-title", "data-word-wrap"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "span".to_string(),
                ["data-line", "data-highlighted", "data-diff"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "*".to_string(),
                ["id", "class"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "div".to_string(),
                ["data-directive"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "abbr".to_string(),
                ["title"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "svg".to_string(),
                ["viewBox", "width", "height", "aria-hidden", "fill", "xmlns"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "path".to_string(),
                ["d", "fill", "fill-rule", "clip-rule"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map
        },
        allowed_protocols: {
            let mut map = HashMap::new();
            map.insert(
                "href".to_string(),
                ["http", "https", "mailto", "#"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map.insert(
                "src".to_string(),
                ["http", "https"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            map
        },
    }
}

#[must_use]
pub fn from_api_schema(api: &crate::api::options::SanitizeSchema) -> SanitizeSchema {
    SanitizeSchema {
        allowed_tags: api.allowed_tags.iter().cloned().collect(),
        allowed_attributes: api
            .allowed_attributes
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().cloned().collect()))
            .collect(),
        allowed_protocols: api
            .allowed_protocols
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().cloned().collect()))
            .collect(),
    }
}

pub fn sanitize(root: &mut HRoot, schema: &SanitizeSchema, diagnostics: &mut DiagnosticSink) {
    sanitize_children(&mut root.children, schema, diagnostics);
}

fn sanitize_children(
    children: &mut Vec<HNode>,
    schema: &SanitizeSchema,
    diagnostics: &mut DiagnosticSink,
) {
    let mut i = 0;
    while i < children.len() {
        match &children[i] {
            HNode::Element(elem) => {
                if !schema.allowed_tags.contains(&elem.tag) {
                    diagnostics.warn(format!("Removed disallowed tag: <{}>", elem.tag), elem.span);
                    let elem_owned = if let HNode::Element(e) = children.remove(i) {
                        e
                    } else {
                        unreachable!()
                    };
                    let mut promoted_children = elem_owned.children;
                    sanitize_children(&mut promoted_children, schema, diagnostics);
                    for (j, child) in promoted_children.into_iter().enumerate() {
                        children.insert(i + j, child);
                    }
                    continue;
                }
                if let HNode::Element(ref mut elem) = children[i] {
                    sanitize_attributes(elem, schema, diagnostics);
                    sanitize_children(&mut elem.children, schema, diagnostics);
                }
            }
            HNode::Raw(raw) => {
                diagnostics.warn("Raw HTML removed during sanitization", raw.span);
                let text = HNode::Text(HText {
                    id: raw.id,
                    span: raw.span,
                    value: raw.value.clone(),
                });
                children[i] = text;
            }
            HNode::Root(_) => {
                if let HNode::Root(ref mut r) = children[i] {
                    sanitize_children(&mut r.children, schema, diagnostics);
                }
            }
            HNode::Text(_) | HNode::Comment(_) | HNode::Doctype(_) => {}
        }
        i += 1;
    }
}

fn sanitize_attributes(
    elem: &mut HElement,
    schema: &SanitizeSchema,
    diagnostics: &mut DiagnosticSink,
) {
    let tag_allowed = schema.allowed_attributes.get(&elem.tag);
    let global_allowed = schema.allowed_attributes.get("*");

    let mut to_remove = Vec::new();
    for (attr_name, attr_value) in elem.attributes.iter() {
        let is_allowed = tag_allowed.is_some_and(|set| set.contains(attr_name))
            || global_allowed.is_some_and(|set| set.contains(attr_name));

        if !is_allowed {
            to_remove.push(attr_name.clone());
            continue;
        }

        if let Some(protocols) = schema.allowed_protocols.get(attr_name)
            && !attr_value.is_empty()
            && !is_safe_protocol(attr_value, protocols)
        {
            to_remove.push(attr_name.clone());
        }
    }

    for attr in &to_remove {
        diagnostics.warn(
            format!("Removed disallowed attribute: {} on <{}>", attr, elem.tag),
            elem.span,
        );
        elem.attributes.remove(attr);
    }
}

fn is_safe_protocol(url: &str, allowed: &HashSet<String>) -> bool {
    if url.starts_with('#') {
        return allowed.contains("#");
    }
    if let Some(colon_pos) = url.find(':') {
        let protocol = &url[..colon_pos];
        allowed.contains(protocol)
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::util::small_map::SmallMap;

    fn make_element(
        id_gen: &mut NodeIdGen,
        tag: &str,
        attrs: SmallMap<String, String>,
        children: Vec<HNode>,
    ) -> HNode {
        HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            tag: tag.to_string(),
            attributes: attrs,
            children,
            self_closing: false,
        })
    }

    fn make_text(id_gen: &mut NodeIdGen, value: &str) -> HNode {
        HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: value.to_string(),
        })
    }

    fn make_root(id_gen: &mut NodeIdGen, children: Vec<HNode>) -> HRoot {
        HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children,
        }
    }

    #[test]
    fn sanitize_removes_script_tag() {
        let mut id_gen = NodeIdGen::new();
        let script_text = make_text(&mut id_gen, "alert('xss')");
        let script = make_element(&mut id_gen, "script", SmallMap::new(), vec![script_text]);
        let mut root = make_root(&mut id_gen, vec![script]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        assert_eq!(root.children.len(), 1);
        if let HNode::Text(t) = &root.children[0] {
            assert_eq!(t.value, "alert('xss')");
        } else {
            panic!("expected text node after sanitization");
        }
        assert!(!diag.is_empty());
    }

    #[test]
    fn sanitize_keeps_safe_tags() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![p]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.tag, "p");
        } else {
            panic!("expected p element");
        }
        assert!(diag.is_empty());
    }

    #[test]
    fn sanitize_keeps_safe_attributes() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "https://example.com".to_string());
        attrs.insert("title".to_string(), "Example".to_string());
        let text = make_text(&mut id_gen, "click");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.attributes.len(), 2);
            assert!(elem.attributes.contains_key(&"href".to_string()));
            assert!(elem.attributes.contains_key(&"title".to_string()));
        } else {
            panic!("expected a element");
        }
        assert!(diag.is_empty());
    }

    #[test]
    fn sanitize_removes_disallowed_attributes() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "https://example.com".to_string());
        attrs.insert("onclick".to_string(), "alert('xss')".to_string());
        let text = make_text(&mut id_gen, "click");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.attributes.len(), 1);
            assert!(elem.attributes.contains_key(&"href".to_string()));
            assert!(!elem.attributes.contains_key(&"onclick".to_string()));
        } else {
            panic!("expected a element");
        }
        assert!(!diag.is_empty());
    }

    #[test]
    fn sanitize_removes_javascript_protocol() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "javascript:alert('xss')".to_string());
        let text = make_text(&mut id_gen, "click");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(!elem.attributes.contains_key(&"href".to_string()));
        } else {
            panic!("expected a element");
        }
    }

    #[test]
    fn sanitize_allows_safe_protocols() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "mailto:user@example.com".to_string());
        let text = make_text(&mut id_gen, "email");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(elem.attributes.contains_key(&"href".to_string()));
        } else {
            panic!("expected a element");
        }
    }

    #[test]
    fn sanitize_allows_fragment_urls() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "#section".to_string());
        let text = make_text(&mut id_gen, "link");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(elem.attributes.contains_key(&"href".to_string()));
        } else {
            panic!("expected a element");
        }
    }

    #[test]
    fn sanitize_allows_relative_urls() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "/page/about".to_string());
        let text = make_text(&mut id_gen, "about");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(elem.attributes.contains_key(&"href".to_string()));
        } else {
            panic!("expected a element");
        }
        assert!(diag.is_empty());
    }

    #[test]
    fn sanitize_raw_html_to_text() {
        let mut id_gen = NodeIdGen::new();
        let raw = HNode::Raw(HRaw {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            value: "<b>bold</b>".to_string(),
        });
        let mut root = make_root(&mut id_gen, vec![raw]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        assert_eq!(root.children.len(), 1);
        if let HNode::Text(t) = &root.children[0] {
            assert_eq!(t.value, "<b>bold</b>");
        } else {
            panic!("expected text node after raw sanitization");
        }
        assert!(!diag.is_empty());
    }

    #[test]
    fn sanitize_nested_elements() {
        let mut id_gen = NodeIdGen::new();
        let script_text = make_text(&mut id_gen, "alert('xss')");
        let script = make_element(&mut id_gen, "script", SmallMap::new(), vec![script_text]);
        let safe_text = make_text(&mut id_gen, "safe");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![safe_text]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![script, p]);
        let mut root = make_root(&mut id_gen, vec![div]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(div_elem) = &root.children[0] {
            assert_eq!(div_elem.tag, "div");
            assert_eq!(div_elem.children.len(), 2);
            if let HNode::Text(t) = &div_elem.children[0] {
                assert_eq!(t.value, "alert('xss')");
            } else {
                panic!("expected promoted text");
            }
            if let HNode::Element(p_elem) = &div_elem.children[1] {
                assert_eq!(p_elem.tag, "p");
            } else {
                panic!("expected p element");
            }
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn sanitize_custom_schema_restrictive() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let text2 = make_text(&mut id_gen, "Title");
        let h1 = make_element(&mut id_gen, "h1", SmallMap::new(), vec![text2]);
        let mut root = make_root(&mut id_gen, vec![p, h1]);

        let schema = SanitizeSchema {
            allowed_tags: ["p"].iter().map(|s| s.to_string()).collect(),
            allowed_attributes: HashMap::new(),
            allowed_protocols: HashMap::new(),
        };
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        assert_eq!(root.children.len(), 2);
        if let HNode::Element(p_elem) = &root.children[0] {
            assert_eq!(p_elem.tag, "p");
        } else {
            panic!("expected p element");
        }
        if let HNode::Text(t) = &root.children[1] {
            assert_eq!(t.value, "Title");
        } else {
            panic!("expected text (promoted from h1)");
        }
    }

    #[test]
    fn sanitize_global_id_attribute_allowed() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("id".to_string(), "my-heading".to_string());
        let text = make_text(&mut id_gen, "Title");
        let h1 = make_element(&mut id_gen, "h1", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![h1]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(elem.attributes.contains_key(&"id".to_string()));
        } else {
            panic!("expected h1 element");
        }
        assert!(diag.is_empty());
    }

    #[test]
    fn sanitize_empty_tree() {
        let mut id_gen = NodeIdGen::new();
        let mut root = make_root(&mut id_gen, vec![]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        assert!(root.children.is_empty());
        assert!(diag.is_empty());
    }

    #[test]
    fn sanitize_img_src_protocol_check() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("src".to_string(), "data:image/png;base64,abc".to_string());
        attrs.insert("alt".to_string(), "img".to_string());
        let img = make_element(&mut id_gen, "img", attrs, vec![]);
        let mut root = make_root(&mut id_gen, vec![img]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(!elem.attributes.contains_key(&"src".to_string()));
            assert!(elem.attributes.contains_key(&"alt".to_string()));
        } else {
            panic!("expected img element");
        }
    }

    #[test]
    fn sanitize_comment_passes_through() {
        let mut id_gen = NodeIdGen::new();
        let comment = HNode::Comment(HComment {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            value: "a comment".to_string(),
        });
        let mut root = make_root(&mut id_gen, vec![comment]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        assert_eq!(root.children.len(), 1);
        assert!(matches!(root.children[0], HNode::Comment(_)));
        assert!(diag.is_empty());
    }

    #[test]
    fn sanitize_deeply_nested_disallowed() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "text");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let section = make_element(&mut id_gen, "section", SmallMap::new(), vec![p]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![section]);
        let mut root = make_root(&mut id_gen, vec![div]);
        let schema = default_safe_schema();
        let mut diag = DiagnosticSink::new();

        sanitize(&mut root, &schema, &mut diag);

        if let HNode::Element(div_elem) = &root.children[0] {
            assert_eq!(div_elem.tag, "div");
            assert_eq!(div_elem.children.len(), 1);
            if let HNode::Element(p_elem) = &div_elem.children[0] {
                assert_eq!(p_elem.tag, "p");
            } else {
                panic!("expected p element");
            }
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn from_api_schema_converts_correctly() {
        let api_schema = crate::api::options::SanitizeSchema {
            allowed_tags: vec!["p".to_string(), "a".to_string()],
            allowed_attributes: {
                let mut map = HashMap::new();
                map.insert("a".to_string(), vec!["href".to_string()]);
                map
            },
            allowed_protocols: {
                let mut map = HashMap::new();
                map.insert("href".to_string(), vec!["https".to_string()]);
                map
            },
        };
        let schema = from_api_schema(&api_schema);
        assert!(schema.allowed_tags.contains("p"));
        assert!(schema.allowed_tags.contains("a"));
        assert!(schema.allowed_attributes.get("a").unwrap().contains("href"));
        assert!(
            schema
                .allowed_protocols
                .get("href")
                .unwrap()
                .contains("https")
        );
    }
}
