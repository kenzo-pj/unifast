use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::*;

fn has_definition_pattern(children: &[MdNode]) -> bool {
    if children
        .iter()
        .any(|c| matches!(c, MdNode::Text(t) if t.value.contains("\n: ")))
    {
        return true;
    }
    children.windows(2).any(|w| {
        matches!(&w[0], MdNode::Break(_))
            && matches!(&w[1], MdNode::Text(t) if t.value.starts_with(": "))
    })
}

fn split_paragraphs_at_definitions(children: &mut Vec<MdNode>, id_gen: &mut NodeIdGen) {
    let mut i = 0;
    while i < children.len() {
        let needs_split = if let MdNode::Paragraph(p) = &children[i] {
            has_definition_pattern(&p.children)
        } else {
            false
        };

        if !needs_split {
            i += 1;
            continue;
        }

        let node = children.remove(i);
        let (span, inline_children) = match node {
            MdNode::Paragraph(p) => (p.span, p.children),
            _ => unreachable!(),
        };

        let mut segments: Vec<Vec<MdNode>> = vec![vec![]];
        let mut iter = inline_children.into_iter().peekable();

        while let Some(child) = iter.next() {
            match &child {
                MdNode::Text(text) if text.value.contains("\n: ") => {
                    let parts: Vec<&str> = text.value.split("\n: ").collect();
                    for (pi, part) in parts.iter().enumerate() {
                        if pi > 0 {
                            segments.push(vec![]);
                        }
                        let value = if pi > 0 {
                            format!(": {part}")
                        } else {
                            (*part).to_string()
                        };
                        if !value.is_empty() {
                            segments.last_mut().unwrap().push(MdNode::Text(Text {
                                id: id_gen.next_id(),
                                span: text.span,
                                value,
                            }));
                        }
                    }
                }
                MdNode::Break(_) if matches!(iter.peek(), Some(MdNode::Text(t)) if t.value.starts_with(": ")) =>
                {
                    segments.push(vec![]);
                }
                _ => {
                    segments.last_mut().unwrap().push(child);
                }
            }
        }

        let paragraphs: Vec<MdNode> = segments
            .into_iter()
            .filter(|seg| !seg.is_empty())
            .map(|seg| {
                MdNode::Paragraph(Paragraph {
                    id: id_gen.next_id(),
                    span,
                    children: seg,
                })
            })
            .collect();

        let count = paragraphs.len();
        for (j, para) in paragraphs.into_iter().enumerate() {
            children.insert(i + j, para);
        }
        i += count;
    }
}

pub fn apply_definition_lists(children: &mut Vec<MdNode>, id_gen: &mut NodeIdGen) {
    split_paragraphs_at_definitions(children, id_gen);

    let mut i = 0;
    while i + 1 < children.len() {
        let is_term = matches!(&children[i], MdNode::Paragraph(_));
        let is_def = if let MdNode::Paragraph(p) =
            &children
                .get(i + 1)
                .unwrap_or(&MdNode::ThematicBreak(ThematicBreak {
                    id: crate::ast::common::NodeId(0),
                    span: Span::empty(),
                })) {
            if let Some(MdNode::Text(text)) = p.children.first() {
                text.value.starts_with(": ")
            } else {
                false
            }
        } else {
            false
        };

        if is_term && is_def {
            let span = children[i].span();
            let term_node = children.remove(i);
            let term_children = match term_node {
                MdNode::Paragraph(p) => p.children,
                _ => vec![],
            };
            let term = MdNode::DefinitionTerm(DefinitionTerm {
                id: id_gen.next_id(),
                span,
                children: term_children,
            });

            let mut items: Vec<MdNode> = vec![term];
            while i < children.len() {
                let is_next_def = if let MdNode::Paragraph(p) = &children[i] {
                    if let Some(MdNode::Text(text)) = p.children.first() {
                        text.value.starts_with(": ")
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_next_def {
                    let def_node = children.remove(i);
                    let mut def_children = match def_node {
                        MdNode::Paragraph(p) => p.children,
                        _ => vec![],
                    };
                    if let Some(MdNode::Text(text)) = def_children.first_mut() {
                        text.value = text
                            .value
                            .strip_prefix(": ")
                            .unwrap_or(&text.value)
                            .to_string();
                    }
                    items.push(MdNode::DefinitionDescription(DefinitionDescription {
                        id: id_gen.next_id(),
                        span,
                        children: def_children,
                    }));
                } else {
                    break;
                }
            }

            children.insert(
                i,
                MdNode::DefinitionList(DefinitionList {
                    id: id_gen.next_id(),
                    span,
                    children: items,
                }),
            );
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_definitions_for_one_term() {
        let mut id_gen = NodeIdGen::new();
        let mut children = vec![
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(0, 4),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(0, 4),
                    value: "Term".to_string(),
                })],
            }),
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(5, 20),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(5, 20),
                    value: ": First definition".to_string(),
                })],
            }),
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(21, 40),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(21, 40),
                    value: ": Second definition".to_string(),
                })],
            }),
        ];
        apply_definition_lists(&mut children, &mut id_gen);
        assert_eq!(children.len(), 1);
        if let MdNode::DefinitionList(dl) = &children[0] {
            assert_eq!(dl.children.len(), 3); // 1 term + 2 descriptions
            assert!(matches!(&dl.children[0], MdNode::DefinitionTerm(_)));
            assert!(matches!(&dl.children[1], MdNode::DefinitionDescription(_)));
            assert!(matches!(&dl.children[2], MdNode::DefinitionDescription(_)));
            if let MdNode::DefinitionDescription(dd) = &dl.children[2]
                && let Some(MdNode::Text(t)) = dd.children.first()
            {
                assert_eq!(t.value, "Second definition");
            }
        } else {
            panic!("expected DefinitionList");
        }
    }

    #[test]
    fn non_definition_paragraph_stops_list() {
        let mut id_gen = NodeIdGen::new();
        let mut children = vec![
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(0, 4),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(0, 4),
                    value: "Term".to_string(),
                })],
            }),
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(5, 20),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(5, 20),
                    value: ": Definition".to_string(),
                })],
            }),
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(21, 40),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(21, 40),
                    value: "Regular paragraph".to_string(),
                })],
            }),
        ];
        apply_definition_lists(&mut children, &mut id_gen);
        assert_eq!(children.len(), 2);
        assert!(matches!(&children[0], MdNode::DefinitionList(_)));
        assert!(matches!(&children[1], MdNode::Paragraph(_)));
    }

    #[test]
    fn creates_definition_list() {
        let mut id_gen = NodeIdGen::new();
        let mut children = vec![
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(0, 4),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(0, 4),
                    value: "Term".to_string(),
                })],
            }),
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(5, 20),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(5, 20),
                    value: ": Description here".to_string(),
                })],
            }),
        ];
        apply_definition_lists(&mut children, &mut id_gen);
        assert_eq!(children.len(), 1);
        assert!(matches!(&children[0], MdNode::DefinitionList(_)));
        if let MdNode::DefinitionList(dl) = &children[0] {
            assert_eq!(dl.children.len(), 2);
            assert!(matches!(&dl.children[0], MdNode::DefinitionTerm(_)));
            assert!(matches!(&dl.children[1], MdNode::DefinitionDescription(_)));
        }
    }

    #[test]
    fn splits_merged_paragraph_with_definition() {
        let mut id_gen = NodeIdGen::new();
        let mut children = vec![MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 25),
            children: vec![MdNode::Text(Text {
                id: id_gen.next_id(),
                span: Span::new(0, 25),
                value: "Term\n: Description here".to_string(),
            })],
        })];
        apply_definition_lists(&mut children, &mut id_gen);
        assert_eq!(children.len(), 1);
        assert!(matches!(&children[0], MdNode::DefinitionList(_)));
        if let MdNode::DefinitionList(dl) = &children[0] {
            assert_eq!(dl.children.len(), 2);
            assert!(matches!(&dl.children[0], MdNode::DefinitionTerm(_)));
            if let MdNode::DefinitionTerm(dt) = &dl.children[0]
                && let Some(MdNode::Text(t)) = dt.children.first()
            {
                assert_eq!(t.value, "Term");
            }
            assert!(matches!(&dl.children[1], MdNode::DefinitionDescription(_)));
            if let MdNode::DefinitionDescription(dd) = &dl.children[1]
                && let Some(MdNode::Text(t)) = dd.children.first()
            {
                assert_eq!(t.value, "Description here");
            }
        }
    }
}
