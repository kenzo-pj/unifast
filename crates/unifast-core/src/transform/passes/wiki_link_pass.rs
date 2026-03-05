use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::{MdNode, Text, WikiLink};
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct WikiLinkPass;

impl Pass for WikiLinkPass {
    fn name(&self) -> &'static str {
        "wiki_link"
    }
    fn phase(&self) -> Phase {
        Phase::Transform
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.wiki_link.enabled {
            return Ok(());
        }
        match ast {
            AstPayload::Mdast(doc) | AstPayload::Both { mdast: doc, .. } => {
                apply_wiki_links(&mut doc.children, ctx.id_gen);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn apply_wiki_links(children: &mut Vec<MdNode>, id_gen: &mut NodeIdGen) {
    let mut i = 0;
    while i < children.len() {
        let should_split = if let MdNode::Text(text) = &children[i] {
            text.value.contains("[[")
        } else {
            false
        };
        if should_split && let MdNode::Text(text) = &children[i] {
            let new_nodes = split_wiki_links(&text.value, text.span, id_gen);
            if new_nodes.len() > 1 {
                let len = new_nodes.len();
                children.splice(i..=i, new_nodes);
                i += len;
                continue;
            }
        }
        if !matches!(&children[i], MdNode::Code(_) | MdNode::InlineCode(_))
            && let Some(kids) = children[i].children_mut()
        {
            apply_wiki_links(kids, id_gen);
        }
        i += 1;
    }
}

fn split_wiki_links(text: &str, span: Span, id_gen: &mut NodeIdGen) -> Vec<MdNode> {
    let mut nodes = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '[' {
            if chars.peek() == Some(&'[') {
                chars.next();
                if !current.is_empty() {
                    nodes.push(MdNode::Text(Text {
                        id: id_gen.next_id(),
                        span,
                        value: std::mem::take(&mut current),
                    }));
                }
                let mut link_content = String::new();
                let mut found_end = false;
                while let Some(c) = chars.next() {
                    if c == ']' && chars.peek() == Some(&']') {
                        chars.next();
                        found_end = true;
                        break;
                    }
                    link_content.push(c);
                }
                if found_end && !link_content.is_empty() {
                    let (target, alias) = if let Some((t, a)) = link_content.split_once('|') {
                        (t.trim().to_string(), Some(a.trim().to_string()))
                    } else {
                        (link_content.trim().to_string(), None)
                    };
                    let display = alias.as_ref().unwrap_or(&target).clone();
                    nodes.push(MdNode::WikiLink(WikiLink {
                        id: id_gen.next_id(),
                        span,
                        target,
                        alias: alias.clone(),
                        children: vec![MdNode::Text(Text {
                            id: id_gen.next_id(),
                            span,
                            value: display,
                        })],
                    }));
                } else {
                    current.push_str("[[");
                    current.push_str(&link_content);
                }
            } else {
                current.push(ch);
            }
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        nodes.push(MdNode::Text(Text {
            id: id_gen.next_id(),
            span,
            value: current,
        }));
    }

    if nodes.is_empty() {
        nodes.push(MdNode::Text(Text {
            id: id_gen.next_id(),
            span,
            value: String::new(),
        }));
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let pass = WikiLinkPass;
        assert_eq!(pass.name(), "wiki_link");
        assert_eq!(pass.phase(), Phase::Transform);
    }

    #[test]
    fn splits_wiki_link() {
        let mut id_gen = NodeIdGen::new();
        let nodes = split_wiki_links(
            "See [[Page Name]] for details",
            Span::new(0, 30),
            &mut id_gen,
        );
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[1], MdNode::WikiLink(w) if w.target == "Page Name"));
    }

    #[test]
    fn splits_wiki_link_with_alias() {
        let mut id_gen = NodeIdGen::new();
        let nodes = split_wiki_links("See [[Page|display text]]", Span::new(0, 25), &mut id_gen);
        assert_eq!(nodes.len(), 2);
        if let MdNode::WikiLink(w) = &nodes[1] {
            assert_eq!(w.target, "Page");
            assert_eq!(w.alias, Some("display text".to_string()));
        } else {
            panic!("expected WikiLink");
        }
    }
}
