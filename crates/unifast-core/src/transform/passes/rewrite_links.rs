use crate::ast::mdast::nodes::*;

pub struct RewriteOptions {
    pub base_url: Option<String>,
    pub make_absolute: bool,
}

pub fn rewrite_links(doc: &mut Document, options: &RewriteOptions) {
    if options.base_url.is_none() && !options.make_absolute {
        return;
    }
    rewrite_children(&mut doc.children, options);
}

fn rewrite_children(children: &mut [MdNode], options: &RewriteOptions) {
    for child in children.iter_mut() {
        match child {
            MdNode::Link(link) => {
                if let Some(ref base) = options.base_url
                    && is_relative(&link.url)
                {
                    link.url = resolve_url(base, &link.url);
                }
            }
            MdNode::Image(img) => {
                if let Some(ref base) = options.base_url
                    && is_relative(&img.url)
                {
                    img.url = resolve_url(base, &img.url);
                }
            }
            _ => {}
        }
        if let Some(kids) = child.children_mut() {
            rewrite_children(kids, options);
        }
    }
}

fn is_relative(url: &str) -> bool {
    !url.starts_with("http://")
        && !url.starts_with("https://")
        && !url.starts_with("//")
        && !url.starts_with("mailto:")
        && !url.starts_with('#')
}

fn resolve_url(base: &str, relative: &str) -> String {
    let base = base.trim_end_matches('/');
    let relative = relative.trim_start_matches("./");
    format!("{base}/{relative}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};

    fn make_link(id_gen: &mut NodeIdGen, url: &str, text: &str) -> MdNode {
        let text_node = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, text.len() as u32),
            value: text.to_string(),
        });
        MdNode::Link(Link {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            url: url.to_string(),
            title: None,
            children: vec![text_node],
        })
    }

    fn make_image(id_gen: &mut NodeIdGen, url: &str, alt: &str) -> MdNode {
        MdNode::Image(Image {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            url: url.to_string(),
            title: None,
            alt: alt.to_string(),
        })
    }

    fn make_doc_with(id_gen: &mut NodeIdGen, children: Vec<MdNode>) -> Document {
        Document {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children,
        }
    }

    #[test]
    fn rewrite_relative_link() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "page.html", "link");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: Some("https://example.com/docs".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Link(l) = &p.children[0] {
                assert_eq!(l.url, "https://example.com/docs/page.html");
            } else {
                panic!("Expected link");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn rewrite_relative_link_with_dot_slash() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "./image.png", "link");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: Some("https://example.com/docs".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Link(l) = &p.children[0] {
                assert_eq!(l.url, "https://example.com/docs/image.png");
            } else {
                panic!("Expected link");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn no_rewrite_absolute_link() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "https://other.com/page", "link");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: Some("https://example.com".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Link(l) = &p.children[0] {
                assert_eq!(l.url, "https://other.com/page");
            } else {
                panic!("Expected link");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn no_rewrite_mailto_link() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "mailto:user@example.com", "email");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: Some("https://example.com".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Link(l) = &p.children[0] {
                assert_eq!(l.url, "mailto:user@example.com");
            } else {
                panic!("Expected link");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn no_rewrite_hash_link() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "#section", "anchor");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: Some("https://example.com".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Link(l) = &p.children[0] {
                assert_eq!(l.url, "#section");
            } else {
                panic!("Expected link");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn rewrite_image_url() {
        let mut id_gen = NodeIdGen::new();
        let img = make_image(&mut id_gen, "photo.jpg", "a photo");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![img],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: Some("https://cdn.example.com/images".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Image(i) = &p.children[0] {
                assert_eq!(i.url, "https://cdn.example.com/images/photo.jpg");
            } else {
                panic!("Expected image");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn no_rewrite_without_base_url() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "page.html", "link");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: None,
            make_absolute: false,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Link(l) = &p.children[0] {
                assert_eq!(l.url, "page.html");
            } else {
                panic!("Expected link");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn rewrite_base_url_trailing_slash() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "page.html", "link");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: Some("https://example.com/docs/".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Link(l) = &p.children[0] {
                assert_eq!(l.url, "https://example.com/docs/page.html");
            } else {
                panic!("Expected link");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn no_rewrite_protocol_relative_url() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "//cdn.example.com/file.js", "link");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let mut doc = make_doc_with(&mut id_gen, vec![para]);

        let options = RewriteOptions {
            base_url: Some("https://example.com".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            if let MdNode::Link(l) = &p.children[0] {
                assert_eq!(l.url, "//cdn.example.com/file.js");
            } else {
                panic!("Expected link");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn rewrite_nested_link_in_blockquote() {
        let mut id_gen = NodeIdGen::new();
        let link = make_link(&mut id_gen, "page.html", "link");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![link],
        });
        let bq = MdNode::Blockquote(Blockquote {
            id: id_gen.next_id(),
            span: Span::new(0, 35),
            children: vec![para],
            alert_type: None,
        });
        let mut doc = make_doc_with(&mut id_gen, vec![bq]);

        let options = RewriteOptions {
            base_url: Some("https://example.com".to_string()),
            make_absolute: true,
        };
        rewrite_links(&mut doc, &options);

        if let MdNode::Blockquote(bq) = &doc.children[0] {
            if let MdNode::Paragraph(p) = &bq.children[0] {
                if let MdNode::Link(l) = &p.children[0] {
                    assert_eq!(l.url, "https://example.com/page.html");
                } else {
                    panic!("Expected link");
                }
            } else {
                panic!("Expected paragraph");
            }
        } else {
            panic!("Expected blockquote");
        }
    }
}
