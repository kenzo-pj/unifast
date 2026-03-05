use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::*;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct DefinitionListPass;

impl Pass for DefinitionListPass {
    fn name(&self) -> &'static str {
        "definition_list"
    }
    fn phase(&self) -> Phase {
        Phase::Transform
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.definition_list.enabled {
            return Ok(());
        }
        match ast {
            AstPayload::Mdast(doc) | AstPayload::Both { mdast: doc, .. } => {
                apply_definition_lists(&mut doc.children, ctx.id_gen);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn apply_definition_lists(children: &mut Vec<MdNode>, id_gen: &mut NodeIdGen) {
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
    fn metadata() {
        let pass = DefinitionListPass;
        assert_eq!(pass.name(), "definition_list");
        assert_eq!(pass.phase(), Phase::Transform);
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
}
