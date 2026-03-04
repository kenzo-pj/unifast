use crate::ast::hast::nodes::*;

use super::escape::escape_html;
use super::void_elements::is_void_element;

/// Stringify an HRoot into an HTML string.
pub fn stringify(root: &HRoot) -> String {
    let mut output = String::new();
    stringify_children(&root.children, &mut output);
    output
}

fn stringify_children(children: &[HNode], output: &mut String) {
    for child in children {
        stringify_node(child, output);
    }
}

fn stringify_node(node: &HNode, output: &mut String) {
    match node {
        HNode::Root(root) => stringify_children(&root.children, output),

        HNode::Element(elem) => {
            output.push('<');
            output.push_str(&elem.tag);

            // Attributes in stable order (SmallMap is BTreeMap-backed)
            for (key, value) in elem.attributes.iter() {
                output.push(' ');
                output.push_str(key);
                if !value.is_empty() {
                    output.push_str("=\"");
                    output.push_str(&escape_html(value));
                    output.push('"');
                }
            }

            if is_void_element(&elem.tag) {
                output.push_str(" />");
            } else {
                output.push('>');
                stringify_children(&elem.children, output);
                output.push_str("</");
                output.push_str(&elem.tag);
                output.push('>');
            }
        }

        HNode::Text(text) => {
            output.push_str(&escape_html(&text.value));
        }

        HNode::Comment(comment) => {
            output.push_str("<!--");
            output.push_str(&comment.value);
            output.push_str("-->");
        }

        HNode::Doctype(_) => {
            output.push_str("<!DOCTYPE html>");
        }

        HNode::Raw(raw) => {
            output.push_str(&raw.value);
        }
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
    fn stringify_heading() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let h1 = make_element(&mut id_gen, "h1", SmallMap::new(), vec![text]);
        let root = make_root(&mut id_gen, vec![h1]);
        assert_eq!(stringify(&root), "<h1>Hello</h1>");
    }

    #[test]
    fn stringify_paragraph() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello world");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let root = make_root(&mut id_gen, vec![p]);
        assert_eq!(stringify(&root), "<p>Hello world</p>");
    }

    #[test]
    fn stringify_emphasis() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "word");
        let em = make_element(&mut id_gen, "em", SmallMap::new(), vec![text]);
        let root = make_root(&mut id_gen, vec![em]);
        assert_eq!(stringify(&root), "<em>word</em>");
    }

    #[test]
    fn stringify_strong() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "bold");
        let strong = make_element(&mut id_gen, "strong", SmallMap::new(), vec![text]);
        let root = make_root(&mut id_gen, vec![strong]);
        assert_eq!(stringify(&root), "<strong>bold</strong>");
    }

    #[test]
    fn stringify_code() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "fn main()");
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), "language-rust".to_string());
        let code = make_element(&mut id_gen, "code", attrs, vec![text]);
        let pre = make_element(&mut id_gen, "pre", SmallMap::new(), vec![code]);
        let root = make_root(&mut id_gen, vec![pre]);
        assert_eq!(
            stringify(&root),
            "<pre><code class=\"language-rust\">fn main()</code></pre>"
        );
    }

    #[test]
    fn stringify_link() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Click");
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "http://example.com".to_string());
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let root = make_root(&mut id_gen, vec![a]);
        assert_eq!(stringify(&root), "<a href=\"http://example.com\">Click</a>");
    }

    #[test]
    fn stringify_image_void_element() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("alt".to_string(), "photo".to_string());
        attrs.insert("src".to_string(), "img.png".to_string());
        let img = make_element(&mut id_gen, "img", attrs, vec![]);
        let root = make_root(&mut id_gen, vec![img]);
        assert_eq!(stringify(&root), "<img alt=\"photo\" src=\"img.png\" />");
    }

    #[test]
    fn stringify_br_void_element() {
        let mut id_gen = NodeIdGen::new();
        let br = make_element(&mut id_gen, "br", SmallMap::new(), vec![]);
        let root = make_root(&mut id_gen, vec![br]);
        assert_eq!(stringify(&root), "<br />");
    }

    #[test]
    fn stringify_hr_void_element() {
        let mut id_gen = NodeIdGen::new();
        let hr = make_element(&mut id_gen, "hr", SmallMap::new(), vec![]);
        let root = make_root(&mut id_gen, vec![hr]);
        assert_eq!(stringify(&root), "<hr />");
    }

    #[test]
    fn stringify_text_escaping() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "a < b & c > d");
        let root = make_root(&mut id_gen, vec![text]);
        assert_eq!(stringify(&root), "a &lt; b &amp; c &gt; d");
    }

    #[test]
    fn stringify_attribute_escaping() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("title".to_string(), "say \"hello\"".to_string());
        let text = make_text(&mut id_gen, "x");
        let div = make_element(&mut id_gen, "div", attrs, vec![text]);
        let root = make_root(&mut id_gen, vec![div]);
        assert_eq!(
            stringify(&root),
            "<div title=\"say &quot;hello&quot;\">x</div>"
        );
    }

    #[test]
    fn stringify_attribute_ordering() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("z-attr".to_string(), "z".to_string());
        attrs.insert("a-attr".to_string(), "a".to_string());
        attrs.insert("m-attr".to_string(), "m".to_string());
        let text = make_text(&mut id_gen, "x");
        let div = make_element(&mut id_gen, "div", attrs, vec![text]);
        let root = make_root(&mut id_gen, vec![div]);
        // BTreeMap-backed: alphabetical order
        assert_eq!(
            stringify(&root),
            "<div a-attr=\"a\" m-attr=\"m\" z-attr=\"z\">x</div>"
        );
    }

    #[test]
    fn stringify_raw_html() {
        let mut id_gen = NodeIdGen::new();
        let raw = HNode::Raw(HRaw {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            value: "<b>bold</b>".to_string(),
        });
        let root = make_root(&mut id_gen, vec![raw]);
        assert_eq!(stringify(&root), "<b>bold</b>");
    }

    #[test]
    fn stringify_nested_elements() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let em = make_element(&mut id_gen, "em", SmallMap::new(), vec![text]);
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![em]);
        let root = make_root(&mut id_gen, vec![p]);
        assert_eq!(stringify(&root), "<p><em>Hello</em></p>");
    }

    #[test]
    fn stringify_table() {
        let mut id_gen = NodeIdGen::new();
        let th_text = make_text(&mut id_gen, "Header");
        let th = make_element(&mut id_gen, "th", SmallMap::new(), vec![th_text]);
        let tr_head = make_element(&mut id_gen, "tr", SmallMap::new(), vec![th]);
        let thead = make_element(&mut id_gen, "thead", SmallMap::new(), vec![tr_head]);
        let td_text = make_text(&mut id_gen, "Cell");
        let td = make_element(&mut id_gen, "td", SmallMap::new(), vec![td_text]);
        let tr_body = make_element(&mut id_gen, "tr", SmallMap::new(), vec![td]);
        let tbody = make_element(&mut id_gen, "tbody", SmallMap::new(), vec![tr_body]);
        let table = make_element(&mut id_gen, "table", SmallMap::new(), vec![thead, tbody]);
        let root = make_root(&mut id_gen, vec![table]);
        assert_eq!(
            stringify(&root),
            "<table><thead><tr><th>Header</th></tr></thead><tbody><tr><td>Cell</td></tr></tbody></table>"
        );
    }

    #[test]
    fn stringify_empty_element() {
        let mut id_gen = NodeIdGen::new();
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![]);
        let root = make_root(&mut id_gen, vec![div]);
        assert_eq!(stringify(&root), "<div></div>");
    }

    #[test]
    fn stringify_empty_root() {
        let mut id_gen = NodeIdGen::new();
        let root = make_root(&mut id_gen, vec![]);
        assert_eq!(stringify(&root), "");
    }

    #[test]
    fn stringify_comment() {
        let mut id_gen = NodeIdGen::new();
        let comment = HNode::Comment(HComment {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            value: " a comment ".to_string(),
        });
        let root = make_root(&mut id_gen, vec![comment]);
        assert_eq!(stringify(&root), "<!-- a comment -->");
    }

    #[test]
    fn stringify_doctype() {
        let mut id_gen = NodeIdGen::new();
        let dt = HNode::Doctype(HDoctype {
            id: id_gen.next_id(),
            span: Span::new(0, 15),
        });
        let root = make_root(&mut id_gen, vec![dt]);
        assert_eq!(stringify(&root), "<!DOCTYPE html>");
    }

    #[test]
    fn stringify_boolean_attribute() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("checked".to_string(), String::new());
        attrs.insert("disabled".to_string(), String::new());
        attrs.insert("type".to_string(), "checkbox".to_string());
        let input = make_element(&mut id_gen, "input", attrs, vec![]);
        let root = make_root(&mut id_gen, vec![input]);
        // Boolean attrs have no ="value"
        assert_eq!(
            stringify(&root),
            "<input checked disabled type=\"checkbox\" />"
        );
    }

    #[test]
    fn stringify_multiple_children() {
        let mut id_gen = NodeIdGen::new();
        let text1 = make_text(&mut id_gen, "Hello ");
        let em_text = make_text(&mut id_gen, "world");
        let em = make_element(&mut id_gen, "em", SmallMap::new(), vec![em_text]);
        let text2 = make_text(&mut id_gen, "!");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text1, em, text2]);
        let root = make_root(&mut id_gen, vec![p]);
        assert_eq!(stringify(&root), "<p>Hello <em>world</em>!</p>");
    }

    #[test]
    fn stringify_nested_root() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "inner");
        let inner_root = HNode::Root(HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            children: vec![text],
        });
        let root = make_root(&mut id_gen, vec![inner_root]);
        assert_eq!(stringify(&root), "inner");
    }

    #[test]
    fn stringify_input_void() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("type".to_string(), "text".to_string());
        let input = make_element(&mut id_gen, "input", attrs, vec![]);
        let root = make_root(&mut id_gen, vec![input]);
        assert_eq!(stringify(&root), "<input type=\"text\" />");
    }
}
