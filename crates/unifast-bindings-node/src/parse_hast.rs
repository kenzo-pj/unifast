use unifast_core::ast::common::{NodeId, Span};
use unifast_core::ast::hast::nodes::*;
use unifast_core::util::small_map::SmallMap;

pub fn hroot_from_json(json: &str) -> Result<HRoot, String> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    deserialize_hroot(&value).ok_or_else(|| "Invalid HAST root".to_string())
}

fn deserialize_hroot(value: &serde_json::Value) -> Option<HRoot> {
    let obj = value.as_object()?;
    if obj.get("type")?.as_str()? != "root" {
        return None;
    }
    let children = obj
        .get("children")
        .and_then(deserialize_children)
        .unwrap_or_default();
    Some(HRoot {
        id: NodeId(0),
        span: Span::empty(),
        children,
    })
}

fn deserialize_children(value: &serde_json::Value) -> Option<Vec<HNode>> {
    value.as_array()?.iter().map(deserialize_hnode).collect()
}

fn deserialize_hnode(value: &serde_json::Value) -> Option<HNode> {
    let obj = value.as_object()?;
    let type_str = obj.get("type")?.as_str()?;
    match type_str {
        "root" => deserialize_hroot(value).map(HNode::Root),
        "element" => {
            let tag = obj.get("tagName")?.as_str()?.to_string();
            let properties = obj.get("properties").and_then(|p| p.as_object());
            let mut attrs = SmallMap::new();
            if let Some(props) = properties {
                for (key, val) in props {
                    if key == "className" {
                        if let Some(arr) = val.as_array() {
                            let classes: Vec<&str> =
                                arr.iter().filter_map(serde_json::Value::as_str).collect();
                            if !classes.is_empty() {
                                attrs.insert("class".to_string(), classes.join(" "));
                            }
                        }
                    } else if key == "class" {
                        if let Some(s) = val.as_str() {
                            attrs.insert("class".to_string(), s.to_string());
                        }
                    } else if let Some(s) = val.as_str() {
                        attrs.insert(key.clone(), s.to_string());
                    } else if val.as_bool() == Some(true) {
                        attrs.insert(key.clone(), String::new());
                    } else if let Some(arr) = val.as_array() {
                        let items: Vec<String> = arr
                            .iter()
                            .map(|v| match v {
                                serde_json::Value::String(s) => s.clone(),
                                serde_json::Value::Null => String::new(),
                                other => other.to_string(),
                            })
                            .collect();
                        attrs.insert(key.clone(), items.join(","));
                    } else if let Some(n) = val.as_f64() {
                        attrs.insert(key.clone(), n.to_string());
                    } else if val.is_object() {
                        attrs.insert(key.clone(), "[object Object]".to_string());
                    }
                }
            }
            let children = obj
                .get("children")
                .and_then(deserialize_children)
                .unwrap_or_default();
            Some(HNode::Element(HElement {
                id: NodeId(0),
                span: Span::empty(),
                tag,
                attributes: attrs,
                children,
                self_closing: false,
            }))
        }
        "text" => Some(HNode::Text(HText {
            id: NodeId(0),
            span: Span::empty(),
            value: obj.get("value")?.as_str()?.to_string(),
        })),
        "raw" => Some(HNode::Raw(HRaw {
            id: NodeId(0),
            span: Span::empty(),
            value: obj.get("value")?.as_str()?.to_string(),
        })),
        "comment" => Some(HNode::Comment(HComment {
            id: NodeId(0),
            span: Span::empty(),
            value: obj.get("value")?.as_str()?.to_string(),
        })),
        "doctype" => Some(HNode::Doctype(HDoctype {
            id: NodeId(0),
            span: Span::empty(),
        })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_serialize_deserialize() {
        use unifast_core::emit::html::stringify::stringify;

        let mut attrs = SmallMap::new();
        attrs.insert("class".into(), "language-rust".into());
        attrs.insert("id".into(), "code1".into());
        let root = HRoot {
            id: NodeId(0),
            span: Span::empty(),
            children: vec![
                HNode::Element(HElement {
                    id: NodeId(0),
                    span: Span::empty(),
                    tag: "code".into(),
                    attributes: attrs,
                    children: vec![HNode::Text(HText {
                        id: NodeId(0),
                        span: Span::empty(),
                        value: "fn main()".into(),
                    })],
                    self_closing: false,
                }),
                HNode::Comment(HComment {
                    id: NodeId(0),
                    span: Span::empty(),
                    value: " note ".into(),
                }),
            ],
        };
        let json = serde_json::to_string(&root).unwrap();
        let deserialized = hroot_from_json(&json).unwrap();
        assert_eq!(stringify(&root), stringify(&deserialized));
    }

    #[test]
    fn classname_array_to_class_string() {
        let json = r#"{"type":"root","children":[{"type":"element","tagName":"div","properties":{"className":["foo","bar"],"id":"main"},"children":[{"type":"text","value":"hello"}]}]}"#;
        let root = hroot_from_json(json).unwrap();
        if let HNode::Element(e) = &root.children[0] {
            assert_eq!(
                e.attributes.get(&"class".to_string()),
                Some(&"foo bar".to_string())
            );
            assert_eq!(
                e.attributes.get(&"id".to_string()),
                Some(&"main".to_string())
            );
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn boolean_properties() {
        let json = r#"{"type":"root","children":[{"type":"element","tagName":"input","properties":{"disabled":true,"hidden":false},"children":[]}]}"#;
        let root = hroot_from_json(json).unwrap();
        if let HNode::Element(e) = &root.children[0] {
            assert!(e.attributes.contains_key(&"disabled".to_string()));
            assert!(!e.attributes.contains_key(&"hidden".to_string()));
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn non_classname_array_joins_with_comma() {
        let json = r#"{"type":"root","children":[{"type":"element","tagName":"input","properties":{"accept":["image/png","image/jpeg"]},"children":[]}]}"#;
        let root = hroot_from_json(json).unwrap();
        if let HNode::Element(e) = &root.children[0] {
            assert_eq!(
                e.attributes.get(&"accept".to_string()),
                Some(&"image/png,image/jpeg".to_string())
            );
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn object_property_matches_js_string_coercion() {
        let json = r#"{"type":"root","children":[{"type":"element","tagName":"div","properties":{"style":{"color":"red"}},"children":[]}]}"#;
        let root = hroot_from_json(json).unwrap();
        if let HNode::Element(e) = &root.children[0] {
            assert_eq!(
                e.attributes.get(&"style".to_string()),
                Some(&"[object Object]".to_string())
            );
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn numeric_property() {
        let json = r#"{"type":"root","children":[{"type":"element","tagName":"td","properties":{"colspan":2},"children":[]}]}"#;
        let root = hroot_from_json(json).unwrap();
        if let HNode::Element(e) = &root.children[0] {
            assert_eq!(
                e.attributes.get(&"colspan".to_string()),
                Some(&"2".to_string())
            );
        } else {
            panic!("expected element");
        }
    }
}
