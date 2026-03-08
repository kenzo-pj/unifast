use crate::api::options::{AlertIconDef, GithubAlertIconMode};
use crate::ast::common::NodeIdGen;
use crate::ast::mdast::nodes::*;

#[derive(Debug, Clone, Copy)]
pub enum AlertType {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl AlertType {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "NOTE" => Some(Self::Note),
            "TIP" => Some(Self::Tip),
            "IMPORTANT" => Some(Self::Important),
            "WARNING" => Some(Self::Warning),
            "CAUTION" => Some(Self::Caution),
            _ => None,
        }
    }

    const fn as_str(&self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Tip => "tip",
            Self::Important => "important",
            Self::Warning => "warning",
            Self::Caution => "caution",
        }
    }
}

pub fn apply_github_alerts(doc: &mut Document, id_gen: &mut NodeIdGen) {
    apply_to_children(&mut doc.children, id_gen);
}

fn apply_to_children(children: &mut [MdNode], id_gen: &mut NodeIdGen) {
    for child in children.iter_mut() {
        if let MdNode::Blockquote(bq) = child
            && let Some(alert_type) = detect_alert_type(&bq.children)
        {
            transform_to_alert(bq, alert_type, id_gen);
        }
        if let Some(kids) = child.children_mut() {
            apply_to_children(kids, id_gen);
        }
    }
}

fn detect_alert_type(children: &[MdNode]) -> Option<AlertType> {
    let first = children.first()?;
    if let MdNode::Paragraph(p) = first {
        let first_child = p.children.first()?;
        if let MdNode::Text(text) = first_child {
            let trimmed = text.value.trim();
            if trimmed.starts_with("[!") {
                let end = trimmed.find(']')?;
                let alert_name = &trimmed[2..end];
                return AlertType::from_str(alert_name);
            }
        }
    }
    None
}

fn transform_to_alert(bq: &mut Blockquote, alert_type: AlertType, _id_gen: &mut NodeIdGen) {
    enum Action {
        ReplaceText(String),
        RemoveFirstChild { is_only_child: bool },
        None,
    }

    let action = if let Some(MdNode::Paragraph(p)) = bq.children.first() {
        if let Some(MdNode::Text(text)) = p.children.first() {
            if let Some(end) = text.value.find(']') {
                let rest = text.value[end + 1..].trim_start().to_string();
                if rest.is_empty() {
                    Action::RemoveFirstChild {
                        is_only_child: p.children.len() == 1,
                    }
                } else {
                    Action::ReplaceText(rest)
                }
            } else {
                Action::None
            }
        } else {
            Action::None
        }
    } else {
        Action::None
    };

    if let Some(MdNode::Paragraph(p)) = bq.children.first_mut() {
        match action {
            Action::ReplaceText(rest) => {
                if let Some(MdNode::Text(text)) = p.children.first_mut() {
                    text.value = rest;
                }
            }
            Action::RemoveFirstChild { is_only_child } => {
                p.children.remove(0);
                if is_only_child {}
            }
            Action::None => {}
        }
    }

    if let Some(MdNode::Paragraph(p)) = bq.children.first()
        && p.children.is_empty()
    {
        bq.children.remove(0);
    }

    bq.alert_type = Some(alert_type.as_str().to_string());
}

pub fn resolve_icon_svg(alert_kind: &str, icon_mode: &GithubAlertIconMode) -> Option<String> {
    match icon_mode {
        GithubAlertIconMode::None => None,
        GithubAlertIconMode::Octicon => octicon_svg(alert_kind).map(String::from),
        GithubAlertIconMode::Custom(map) => {
            if let Some(def) = map.get(alert_kind) {
                resolve_custom_icon_svg(def)
            } else {
                octicon_svg(alert_kind).map(String::from)
            }
        }
    }
}

pub fn resolve_icon_import(
    alert_kind: &str,
    icon_mode: &GithubAlertIconMode,
) -> Option<AlertIconDef> {
    match icon_mode {
        GithubAlertIconMode::Custom(map) => map.get(alert_kind).and_then(|def| {
            if def.import_source.is_some() && def.import_name.is_some() {
                Some(def.clone())
            } else {
                None
            }
        }),
        _ => None,
    }
}

fn resolve_custom_icon_svg(def: &AlertIconDef) -> Option<String> {
    def.svg.clone()
}

fn octicon_svg(alert_kind: &str) -> Option<&'static str> {
    let path_d = match alert_kind {
        "note" => OCTICON_NOTE,
        "tip" => OCTICON_TIP,
        "important" => OCTICON_IMPORTANT,
        "warning" => OCTICON_WARNING,
        "caution" => OCTICON_CAUTION,
        _ => return None,
    };
    Some(path_d)
}

pub const OCTICON_NOTE: &str = r#"<svg class="alert-icon" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"></path></svg>"#;

pub const OCTICON_TIP: &str = r#"<svg class="alert-icon" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path d="M8 1.5c-2.363 0-4 1.69-4 3.75 0 .984.424 1.625.984 2.304l.214.253c.223.264.47.556.673.848.284.411.537.896.621 1.49a.75.75 0 0 1-1.484.211c-.04-.282-.163-.547-.37-.847a8.456 8.456 0 0 0-.542-.68c-.084-.1-.173-.205-.268-.32C3.201 7.75 2.5 6.766 2.5 5.25 2.5 2.31 4.863 0 8 0s5.5 2.31 5.5 5.25c0 1.516-.701 2.5-1.328 3.259-.095.115-.184.22-.268.319-.207.245-.383.453-.541.681-.208.3-.33.565-.37.847a.751.751 0 0 1-1.485-.212c.084-.593.337-1.078.621-1.489.203-.292.45-.584.673-.848.075-.088.147-.173.213-.253.561-.679.985-1.32.985-2.304 0-2.06-1.637-3.75-4-3.75ZM5.75 12h4.5a.75.75 0 0 1 0 1.5h-4.5a.75.75 0 0 1 0-1.5ZM6 15.25a.75.75 0 0 1 .75-.75h2.5a.75.75 0 0 1 0 1.5h-2.5a.75.75 0 0 1-.75-.75Z"></path></svg>"#;

pub const OCTICON_IMPORTANT: &str = r#"<svg class="alert-icon" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path d="M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0 1 14.25 13H8.06l-2.573 2.573A1.458 1.458 0 0 1 3 14.543V13H1.75A1.75 1.75 0 0 1 0 11.25Zm1.75-.25a.25.25 0 0 0-.25.25v9.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h6.5a.25.25 0 0 0 .25-.25v-9.5a.25.25 0 0 0-.25-.25Zm7 2.25v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path></svg>"#;

pub const OCTICON_WARNING: &str = r#"<svg class="alert-icon" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path></svg>"#;

pub const OCTICON_CAUTION: &str = r#"<svg class="alert-icon" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path d="M4.47.22A.749.749 0 0 1 5 0h6c.199 0 .389.079.53.22l4.25 4.25c.141.14.22.331.22.53v6a.749.749 0 0 1-.22.53l-4.25 4.25A.749.749 0 0 1 11 16H5a.749.749 0 0 1-.53-.22L.22 11.53A.749.749 0 0 1 0 11V5c0-.199.079-.389.22-.53Zm.84 1.28L1.5 5.31v5.38l3.81 3.81h5.38l3.81-3.81V5.31L10.69 1.5ZM8 4a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0v-3.5A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"></path></svg>"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};

    #[test]
    fn detects_note_alert() {
        let children = vec![MdNode::Paragraph(Paragraph {
            id: NodeIdGen::new().next_id(),
            span: Span::empty(),
            children: vec![MdNode::Text(Text {
                id: NodeIdGen::new().next_id(),
                span: Span::empty(),
                value: "[!NOTE]".to_string(),
            })],
        })];
        assert!(matches!(
            detect_alert_type(&children),
            Some(AlertType::Note)
        ));
    }

    #[test]
    fn detects_warning_alert() {
        let children = vec![MdNode::Paragraph(Paragraph {
            id: NodeIdGen::new().next_id(),
            span: Span::empty(),
            children: vec![MdNode::Text(Text {
                id: NodeIdGen::new().next_id(),
                span: Span::empty(),
                value: "[!WARNING] Be careful".to_string(),
            })],
        })];
        assert!(matches!(
            detect_alert_type(&children),
            Some(AlertType::Warning)
        ));
    }

    #[test]
    fn no_alert_in_normal_blockquote() {
        let children = vec![MdNode::Paragraph(Paragraph {
            id: NodeIdGen::new().next_id(),
            span: Span::empty(),
            children: vec![MdNode::Text(Text {
                id: NodeIdGen::new().next_id(),
                span: Span::empty(),
                value: "Just a quote".to_string(),
            })],
        })];
        assert!(detect_alert_type(&children).is_none());
    }
}
