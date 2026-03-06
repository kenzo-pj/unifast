use crate::ast::common::NodeIdGen;
use crate::ast::mdast::nodes::{ContainerDirective, MdNode};

pub fn apply_directives(children: &mut Vec<MdNode>, id_gen: &mut NodeIdGen) {
    let mut i = 0;
    while i < children.len() {
        let directive_info = if let MdNode::Paragraph(p) = &children[i] {
            if let Some(MdNode::Text(text)) = p.children.first() {
                parse_directive_opener(&text.value)
            } else {
                None
            }
        } else {
            None
        };

        if let Some((name, attrs)) = directive_info {
            let span = children[i].span();
            let mut end_idx = None;
            for (j, child) in children.iter().enumerate().skip(i + 1) {
                if let MdNode::Paragraph(p) = child
                    && let Some(MdNode::Text(text)) = p.children.first()
                    && text.value.trim() == ":::"
                {
                    end_idx = Some(j);
                    break;
                }
            }

            if let Some(end) = end_idx {
                let content_children: Vec<MdNode> = children.drain(i + 1..end).collect();
                children.remove(i);
                children[i] = MdNode::ContainerDirective(ContainerDirective {
                    id: id_gen.next_id(),
                    span,
                    name,
                    attributes: attrs,
                    children: content_children,
                });
            }
        }

        if let Some(kids) = children[i].children_mut() {
            apply_directives(kids, id_gen);
        }
        i += 1;
    }
}

pub fn parse_directive_opener(text: &str) -> Option<(String, Vec<(String, String)>)> {
    let trimmed = text.trim();
    if !trimmed.starts_with(":::") {
        return None;
    }
    let rest = trimmed[3..].trim();
    if rest.is_empty() || rest == ":::" {
        return None;
    }

    let mut parts = rest.splitn(2, ' ');
    let name = parts.next()?.to_string();

    let mut attrs = Vec::new();
    if let Some(attr_str) = parts.next() {
        let mut remaining = attr_str.trim();
        while !remaining.is_empty() {
            if let Some((k, after_eq)) = remaining.split_once('=') {
                let k = k.trim();
                let after_eq = after_eq.trim_start();
                if let Some(stripped) = after_eq.strip_prefix('"') {
                    if let Some(end_quote) = stripped.find('"') {
                        let v = &stripped[..end_quote];
                        attrs.push((k.to_string(), v.to_string()));
                        remaining = stripped[end_quote + 1..].trim_start();
                    } else {
                        attrs.push((k.to_string(), stripped.to_string()));
                        break;
                    }
                } else {
                    let (v, rest) = match after_eq.find(char::is_whitespace) {
                        Some(idx) => (&after_eq[..idx], after_eq[idx..].trim_start()),
                        None => (after_eq, ""),
                    };
                    attrs.push((k.to_string(), v.to_string()));
                    remaining = rest;
                }
            } else {
                break;
            }
        }
    }

    Some((name, attrs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_directive_opener() {
        let (name, attrs) = parse_directive_opener(":::note").unwrap();
        assert_eq!(name, "note");
        assert!(attrs.is_empty());
    }

    #[test]
    fn parses_directive_with_attrs() {
        let (name, attrs) = parse_directive_opener(":::warning title=\"Be careful\"").unwrap();
        assert_eq!(name, "warning");
        assert_eq!(attrs[0], ("title".to_string(), "Be careful".to_string()));
    }

    #[test]
    fn rejects_empty_directive() {
        assert!(parse_directive_opener(":::").is_none());
    }

    #[test]
    fn rejects_non_directive() {
        assert!(parse_directive_opener("not a directive").is_none());
    }
}
