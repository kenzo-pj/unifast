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
