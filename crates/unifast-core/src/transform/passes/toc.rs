use crate::ast::mdast::nodes::*;

#[derive(Debug, Clone)]
pub struct TocEntry {
    pub depth: u8,
    pub text: String,
    pub slug: String,
}

#[must_use]
pub fn generate_toc(doc: &Document, max_depth: u8) -> Vec<TocEntry> {
    let mut entries = Vec::new();
    collect_headings(&doc.children, max_depth, &mut entries);
    entries
}

fn collect_headings(children: &[MdNode], max_depth: u8, entries: &mut Vec<TocEntry>) {
    for child in children {
        if let MdNode::Heading(h) = child
            && h.depth <= max_depth
        {
            entries.push(TocEntry {
                depth: h.depth,
                text: super::slug::extract_text(&h.children),
                slug: h.slug.clone().unwrap_or_default(),
            });
        }
        if !matches!(child, MdNode::Heading(_))
            && let Some(kids) = child.children()
        {
            collect_headings(kids, max_depth, entries);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::transform::passes::slug::{SlugMode, apply_slugs};
    use crate::util::small_map::SmallMap;

    fn make_heading_node(
        id_gen: &mut NodeIdGen,
        depth: u8,
        text: &str,
        slug: Option<&str>,
    ) -> MdNode {
        let text_node = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, text.len() as u32),
            value: text.to_string(),
        });
        MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(0, text.len() as u32 + 3),
            depth,
            children: vec![text_node],
            slug: slug.map(|s| s.to_string()),
            extra_attrs: SmallMap::new(),
        })
    }

    #[test]
    fn toc_collects_headings() {
        let mut id_gen = NodeIdGen::new();
        let h1 = make_heading_node(&mut id_gen, 1, "One", Some("one"));
        let h2 = make_heading_node(&mut id_gen, 2, "Two", Some("two"));
        let h3 = make_heading_node(&mut id_gen, 3, "Three", Some("three"));
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 50),
            children: vec![h1, h2, h3],
        };

        let toc = generate_toc(&doc, 6);
        assert_eq!(toc.len(), 3);
        assert_eq!(toc[0].depth, 1);
        assert_eq!(toc[0].text, "One");
        assert_eq!(toc[0].slug, "one");
        assert_eq!(toc[1].depth, 2);
        assert_eq!(toc[1].text, "Two");
        assert_eq!(toc[2].depth, 3);
        assert_eq!(toc[2].text, "Three");
    }

    #[test]
    fn toc_respects_max_depth() {
        let mut id_gen = NodeIdGen::new();
        let h1 = make_heading_node(&mut id_gen, 1, "One", Some("one"));
        let h2 = make_heading_node(&mut id_gen, 2, "Two", Some("two"));
        let h3 = make_heading_node(&mut id_gen, 3, "Three", Some("three"));
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 50),
            children: vec![h1, h2, h3],
        };

        let toc = generate_toc(&doc, 2);
        assert_eq!(toc.len(), 2);
        assert_eq!(toc[0].text, "One");
        assert_eq!(toc[1].text, "Two");
    }

    #[test]
    fn toc_empty_document() {
        let mut id_gen = NodeIdGen::new();
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 0),
            children: vec![],
        };

        let toc = generate_toc(&doc, 6);
        assert!(toc.is_empty());
    }

    #[test]
    fn toc_no_headings() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".to_string(),
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![text],
        });
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![para],
        };

        let toc = generate_toc(&doc, 6);
        assert!(toc.is_empty());
    }

    #[test]
    fn toc_uses_slug_from_heading() {
        let mut id_gen = NodeIdGen::new();
        let text_node = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(2, 13),
            value: "Hello World".to_string(),
        });
        let h1 = MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(0, 14),
            depth: 1,
            children: vec![text_node],
            slug: None,
            extra_attrs: SmallMap::new(),
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 14),
            children: vec![h1],
        };

        apply_slugs(&mut doc, SlugMode::GitHub);
        let toc = generate_toc(&doc, 6);

        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].slug, "hello-world");
        assert_eq!(toc[0].text, "Hello World");
    }

    #[test]
    fn toc_heading_without_slug_uses_empty_default() {
        let mut id_gen = NodeIdGen::new();
        let h1 = make_heading_node(&mut id_gen, 1, "No Slug", None);
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![h1],
        };

        let toc = generate_toc(&doc, 6);
        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].slug, "");
        assert_eq!(toc[0].text, "No Slug");
    }

    #[test]
    fn toc_heading_inside_blockquote() {
        let mut id_gen = NodeIdGen::new();
        let h2 = make_heading_node(&mut id_gen, 2, "Quoted", Some("quoted"));
        let bq = MdNode::Blockquote(Blockquote {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![h2],
            alert_type: None,
        });
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![bq],
        };

        let toc = generate_toc(&doc, 6);
        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].text, "Quoted");
    }

    #[test]
    fn toc_heading_inside_container_directive() {
        let mut id_gen = NodeIdGen::new();
        let h2 = make_heading_node(&mut id_gen, 2, "Inside Directive", Some("inside-directive"));
        let dir = MdNode::ContainerDirective(ContainerDirective {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            name: "note".to_string(),
            attributes: vec![],
            children: vec![h2],
        });
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![dir],
        };

        let toc = generate_toc(&doc, 6);
        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].text, "Inside Directive");
    }

    #[test]
    fn toc_heading_inside_table_cell() {
        let mut id_gen = NodeIdGen::new();
        let h3 = make_heading_node(&mut id_gen, 3, "In Cell", Some("in-cell"));
        let cell = MdNode::TableCell(TableCell {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![h3],
        });
        let row = MdNode::TableRow(TableRow {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            is_header: false,
            children: vec![cell],
        });
        let table = MdNode::Table(Table {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![row],
            align: vec![],
        });
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![table],
        };

        let toc = generate_toc(&doc, 6);
        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].text, "In Cell");
    }

    #[test]
    fn toc_heading_inside_definition_list() {
        let mut id_gen = NodeIdGen::new();
        let h2 = make_heading_node(&mut id_gen, 2, "In DL", Some("in-dl"));
        let dd = MdNode::DefinitionDescription(DefinitionDescription {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![h2],
        });
        let dl = MdNode::DefinitionList(DefinitionList {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![dd],
        });
        let doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![dl],
        };

        let toc = generate_toc(&doc, 6);
        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].text, "In DL");
    }
}
