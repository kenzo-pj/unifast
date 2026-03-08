use crate::ast::common::NodeIdGen;
use crate::ast::hast::nodes::{HElement, HNode, HRoot};
use crate::util::small_map::SmallMap;

#[derive(Debug, Clone, Default)]
pub struct ParsedCodeMeta {
    pub title: Option<String>,
    pub highlighted_lines: Vec<LineRange>,
    pub diff: bool,
    pub show_line_numbers: bool,
    pub word_wrap: bool,
    pub custom: SmallMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct LineRange {
    pub start: u32,
    pub end: u32,
}

pub fn parse_meta(meta: &str) -> ParsedCodeMeta {
    let mut result = ParsedCodeMeta::default();
    let meta = meta.trim();
    if meta.is_empty() {
        return result;
    }

    let mut pos = 0;
    let bytes = meta.as_bytes();

    while pos < bytes.len() {
        while pos < bytes.len() && bytes[pos] == b' ' {
            pos += 1;
        }
        if pos >= bytes.len() {
            break;
        }

        if bytes[pos] == b'{'
            && let Some(end) = meta[pos..].find('}')
        {
            let inner = &meta[pos + 1..pos + end];
            parse_line_ranges(inner, &mut result.highlighted_lines);
            pos += end + 1;
            continue;
        }

        if meta[pos..].starts_with("title=") {
            pos += 6;
            if pos < bytes.len() && bytes[pos] == b'"' {
                pos += 1;
                let start = pos;
                while pos < bytes.len() && bytes[pos] != b'"' {
                    pos += 1;
                }
                result.title = Some(meta[start..pos].to_string());
                if pos < bytes.len() {
                    pos += 1;
                }
            } else {
                let start = pos;
                while pos < bytes.len() && bytes[pos] != b' ' {
                    pos += 1;
                }
                result.title = Some(meta[start..pos].to_string());
            }
            continue;
        }

        if meta[pos..].starts_with("showLineNumbers") {
            result.show_line_numbers = true;
            pos += "showLineNumbers".len();
            continue;
        }

        if meta[pos..].starts_with("diff") && (pos + 4 >= bytes.len() || bytes[pos + 4] == b' ') {
            result.diff = true;
            pos += "diff".len();
            continue;
        }

        if meta[pos..].starts_with("wordWrap") {
            result.word_wrap = true;
            pos += "wordWrap".len();
            continue;
        }

        while pos < bytes.len() && bytes[pos] != b' ' {
            pos += 1;
        }
    }

    result
}

fn parse_line_ranges(inner: &str, ranges: &mut Vec<LineRange>) {
    for part in inner.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(dash_pos) = part.find('-') {
            let start_str = part[..dash_pos].trim();
            let end_str = part[dash_pos + 1..].trim();
            if let (Ok(start), Ok(end)) = (start_str.parse::<u32>(), end_str.parse::<u32>()) {
                ranges.push(LineRange { start, end });
            }
        } else if let Ok(n) = part.parse::<u32>() {
            ranges.push(LineRange { start: n, end: n });
        }
    }
}

pub fn apply_code_meta(root: &mut HRoot, _id_gen: &mut NodeIdGen) {
    for child in &mut root.children {
        apply_to_node(child);
    }
}

fn apply_to_node(node: &mut HNode) {
    match node {
        HNode::Element(elem) => {
            if elem.tag == "pre" {
                if let Some(meta_str) = elem.attributes.get(&"data-meta".to_string()).cloned() {
                    let parsed = parse_meta(&meta_str);
                    apply_meta_to_pre(elem, &parsed);
                    elem.attributes.remove(&"data-meta".to_string());
                }
            } else {
                for child in &mut elem.children {
                    apply_to_node(child);
                }
            }
        }
        HNode::Root(r) => {
            for child in &mut r.children {
                apply_to_node(child);
            }
        }
        _ => {}
    }
}

fn apply_meta_to_pre(pre: &mut HElement, meta: &ParsedCodeMeta) {
    if let Some(ref title) = meta.title {
        pre.attributes
            .insert("data-title".to_string(), title.clone());
    }

    if meta.word_wrap {
        pre.attributes
            .insert("data-word-wrap".to_string(), String::new());
    }

    if !meta.highlighted_lines.is_empty() {
        annotate_highlighted_lines(&mut pre.children, &meta.highlighted_lines);
    }

    if meta.diff {
        annotate_diff_lines(&mut pre.children);
    }
}

fn annotate_highlighted_lines(children: &mut [HNode], ranges: &[LineRange]) {
    for child in children.iter_mut() {
        match child {
            HNode::Element(elem) => {
                if elem.tag == "code" {
                    annotate_highlighted_lines(&mut elem.children, ranges);
                } else if elem.tag == "span"
                    && is_line_span(elem)
                    && let Some(line_num) = get_line_number(elem)
                    && line_in_ranges(line_num, ranges)
                {
                    elem.attributes
                        .insert("data-highlighted".to_string(), String::new());
                }
            }
            HNode::Raw(raw) => {
                annotate_raw_highlighted_lines(raw, ranges);
            }
            _ => {}
        }
    }
}

fn annotate_raw_highlighted_lines(raw: &mut crate::ast::hast::nodes::HRaw, ranges: &[LineRange]) {
    let value = &raw.value;
    if !value.contains("class=\"line\"") {
        return;
    }

    let mut result = String::with_capacity(value.len() + 64);
    let mut pos = 0;

    while pos < value.len() {
        if let Some(span_start) = value[pos..].find("<span class=\"line\" data-line=\"") {
            let abs_start = pos + span_start;
            result.push_str(&value[pos..abs_start]);

            let after_prefix = abs_start + "<span class=\"line\" data-line=\"".len();
            if let Some(quote_end) = value[after_prefix..].find('"')
                && let Ok(line_num) = value[after_prefix..after_prefix + quote_end].parse::<u32>()
                && line_in_ranges(line_num, ranges)
            {
                let line_num_str = &value[after_prefix..after_prefix + quote_end];
                result.push_str("<span class=\"line\" data-line=\"");
                result.push_str(line_num_str);
                result.push_str("\" data-highlighted=\"\"");
                pos = after_prefix + quote_end;
                if pos < value.len() && value.as_bytes()[pos] == b'"' {
                    pos += 1;
                }
                continue;
            }

            result.push_str(&value[abs_start..=abs_start]);
            pos = abs_start + 1;
        } else {
            result.push_str(&value[pos..]);
            break;
        }
    }

    raw.value = result;
}

fn annotate_diff_lines(children: &mut [HNode]) {
    for child in children.iter_mut() {
        if let HNode::Element(elem) = child {
            if elem.tag == "code" {
                annotate_diff_lines(&mut elem.children);
            } else if elem.tag == "span" && is_line_span(elem) {
                let text = collect_text_content(&elem.children);
                if text.starts_with('+') {
                    elem.attributes
                        .insert("data-diff".to_string(), "add".to_string());
                } else if text.starts_with('-') {
                    elem.attributes
                        .insert("data-diff".to_string(), "remove".to_string());
                }
            }
        }
    }
}

fn is_line_span(elem: &HElement) -> bool {
    elem.attributes
        .get(&"class".to_string())
        .is_some_and(|v| v.split_whitespace().any(|c| c == "line"))
}

fn get_line_number(elem: &HElement) -> Option<u32> {
    elem.attributes
        .get(&"data-line".to_string())
        .and_then(|v| v.parse::<u32>().ok())
}

fn line_in_ranges(line: u32, ranges: &[LineRange]) -> bool {
    ranges.iter().any(|r| line >= r.start && line <= r.end)
}

fn collect_text_content(nodes: &[HNode]) -> String {
    let mut out = String::new();
    for node in nodes {
        collect_text_inner(node, &mut out);
    }
    out
}

fn collect_text_inner(node: &HNode, out: &mut String) {
    match node {
        HNode::Text(t) => out.push_str(&t.value),
        HNode::Element(e) => {
            for child in &e.children {
                collect_text_inner(child, out);
            }
        }
        HNode::Root(r) => {
            for child in &r.children {
                collect_text_inner(child, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::ast::hast::nodes::{HElement, HNode, HRaw, HRoot, HText};
    use crate::emit::html::stringify;

    #[test]
    fn parse_meta_empty() {
        let meta = parse_meta("");
        assert!(meta.title.is_none());
        assert!(meta.highlighted_lines.is_empty());
        assert!(!meta.diff);
        assert!(!meta.show_line_numbers);
        assert!(!meta.word_wrap);
    }

    #[test]
    fn parse_meta_title_quoted() {
        let meta = parse_meta("title=\"app.ts\"");
        assert_eq!(meta.title.as_deref(), Some("app.ts"));
    }

    #[test]
    fn parse_meta_title_unquoted() {
        let meta = parse_meta("title=app.ts");
        assert_eq!(meta.title.as_deref(), Some("app.ts"));
    }

    #[test]
    fn parse_meta_highlighted_lines() {
        let meta = parse_meta("{1,3-5,10}");
        assert_eq!(meta.highlighted_lines.len(), 3);
        assert_eq!(meta.highlighted_lines[0].start, 1);
        assert_eq!(meta.highlighted_lines[0].end, 1);
        assert_eq!(meta.highlighted_lines[1].start, 3);
        assert_eq!(meta.highlighted_lines[1].end, 5);
        assert_eq!(meta.highlighted_lines[2].start, 10);
        assert_eq!(meta.highlighted_lines[2].end, 10);
    }

    #[test]
    fn parse_meta_boolean_flags() {
        let meta = parse_meta("showLineNumbers diff wordWrap");
        assert!(meta.show_line_numbers);
        assert!(meta.diff);
        assert!(meta.word_wrap);
    }

    #[test]
    fn parse_meta_combined() {
        let meta = parse_meta("{1,3-5} title=\"example.rs\" diff");
        assert_eq!(meta.title.as_deref(), Some("example.rs"));
        assert!(meta.diff);
        assert_eq!(meta.highlighted_lines.len(), 2);
        assert_eq!(meta.highlighted_lines[0].start, 1);
        assert_eq!(meta.highlighted_lines[0].end, 1);
        assert_eq!(meta.highlighted_lines[1].start, 3);
        assert_eq!(meta.highlighted_lines[1].end, 5);
    }

    fn make_code_block_with_meta(
        id_gen: &mut NodeIdGen,
        code_text: &str,
        lang: Option<&str>,
        meta: Option<&str>,
    ) -> HRoot {
        let span = Span::new(0, 100);

        let text = HNode::Text(HText {
            id: id_gen.next_id(),
            span,
            value: code_text.to_string(),
        });

        let mut code_attrs = SmallMap::new();
        if let Some(l) = lang {
            code_attrs.insert("class".to_string(), format!("language-{l}"));
        }

        let code_elem = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "code".to_string(),
            attributes: code_attrs,
            children: vec![text],
            self_closing: false,
        });

        let mut pre_attrs = SmallMap::new();
        if let Some(m) = meta {
            pre_attrs.insert("data-meta".to_string(), m.to_string());
        }
        if let Some(l) = lang {
            pre_attrs.insert("data-lang".to_string(), l.to_string());
        }

        let pre = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "pre".to_string(),
            attributes: pre_attrs,
            children: vec![code_elem],
            self_closing: false,
        });

        HRoot {
            id: id_gen.next_id(),
            span,
            children: vec![pre],
        }
    }

    #[test]
    fn apply_title() {
        let mut id_gen = NodeIdGen::new();
        let mut root = make_code_block_with_meta(
            &mut id_gen,
            "console.log('hi')",
            Some("js"),
            Some("title=\"app.ts\""),
        );
        apply_code_meta(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(
            html.contains("data-title=\"app.ts\""),
            "missing data-title: {html}"
        );
        assert!(
            !html.contains("data-meta"),
            "data-meta should be removed: {html}"
        );
    }

    #[test]
    fn apply_word_wrap() {
        let mut id_gen = NodeIdGen::new();
        let mut root =
            make_code_block_with_meta(&mut id_gen, "hello", Some("js"), Some("wordWrap"));
        apply_code_meta(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(
            html.contains("data-word-wrap"),
            "missing data-word-wrap: {html}"
        );
    }

    #[test]
    fn apply_highlighted_lines_on_element_spans() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(0, 100);

        let make_line = |id_gen: &mut NodeIdGen, num: u32, text: &str| -> HNode {
            let mut attrs = SmallMap::new();
            attrs.insert("class".to_string(), "line".to_string());
            attrs.insert("data-line".to_string(), num.to_string());
            HNode::Element(HElement {
                id: id_gen.next_id(),
                span,
                tag: "span".to_string(),
                attributes: attrs,
                children: vec![HNode::Text(HText {
                    id: id_gen.next_id(),
                    span,
                    value: text.to_string(),
                })],
                self_closing: false,
            })
        };

        let line1 = make_line(&mut id_gen, 1, "first");
        let line2 = make_line(&mut id_gen, 2, "second");
        let line3 = make_line(&mut id_gen, 3, "third");

        let code_elem = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "code".to_string(),
            attributes: SmallMap::new(),
            children: vec![line1, line2, line3],
            self_closing: false,
        });

        let mut pre_attrs = SmallMap::new();
        pre_attrs.insert("data-meta".to_string(), "{1,3}".to_string());

        let pre = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "pre".to_string(),
            attributes: pre_attrs,
            children: vec![code_elem],
            self_closing: false,
        });

        let mut root = HRoot {
            id: id_gen.next_id(),
            span,
            children: vec![pre],
        };

        apply_code_meta(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(
            html.contains("data-highlighted data-line=\"1\""),
            "line 1 should be highlighted: {html}"
        );
        assert!(
            !html.contains("data-highlighted data-line=\"2\""),
            "line 2 should not be highlighted: {html}"
        );
        assert!(
            html.contains("data-highlighted data-line=\"3\""),
            "line 3 should be highlighted: {html}"
        );
    }

    #[test]
    fn apply_diff_on_element_spans() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(0, 100);

        let make_line = |id_gen: &mut NodeIdGen, num: u32, text: &str| -> HNode {
            let mut attrs = SmallMap::new();
            attrs.insert("class".to_string(), "line".to_string());
            attrs.insert("data-line".to_string(), num.to_string());
            HNode::Element(HElement {
                id: id_gen.next_id(),
                span,
                tag: "span".to_string(),
                attributes: attrs,
                children: vec![HNode::Text(HText {
                    id: id_gen.next_id(),
                    span,
                    value: text.to_string(),
                })],
                self_closing: false,
            })
        };

        let line1 = make_line(&mut id_gen, 1, "+added line");
        let line2 = make_line(&mut id_gen, 2, "-removed line");
        let line3 = make_line(&mut id_gen, 3, " unchanged");

        let code_elem = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "code".to_string(),
            attributes: SmallMap::new(),
            children: vec![line1, line2, line3],
            self_closing: false,
        });

        let mut pre_attrs = SmallMap::new();
        pre_attrs.insert("data-meta".to_string(), "diff".to_string());

        let pre = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "pre".to_string(),
            attributes: pre_attrs,
            children: vec![code_elem],
            self_closing: false,
        });

        let mut root = HRoot {
            id: id_gen.next_id(),
            span,
            children: vec![pre],
        };

        apply_code_meta(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(
            html.contains("data-diff=\"add\""),
            "should have data-diff=add: {html}"
        );
        assert!(
            html.contains("data-diff=\"remove\""),
            "should have data-diff=remove: {html}"
        );
    }

    #[test]
    fn removes_data_meta_after_processing() {
        let mut id_gen = NodeIdGen::new();
        let mut root = make_code_block_with_meta(
            &mut id_gen,
            "hello",
            Some("js"),
            Some("title=\"test\" diff wordWrap"),
        );
        apply_code_meta(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(
            !html.contains("data-meta"),
            "data-meta should be removed: {html}"
        );
    }

    #[test]
    fn highlighted_lines_on_raw_spans() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(0, 100);

        let raw1 = HNode::Raw(HRaw {
            id: id_gen.next_id(),
            span,
            value: "<span class=\"line\" data-line=\"1\">first</span>".to_string(),
        });
        let raw2 = HNode::Raw(HRaw {
            id: id_gen.next_id(),
            span,
            value: "<span class=\"line\" data-line=\"2\">second</span>".to_string(),
        });

        let code_elem = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "code".to_string(),
            attributes: SmallMap::new(),
            children: vec![raw1, raw2],
            self_closing: false,
        });

        let mut pre_attrs = SmallMap::new();
        pre_attrs.insert("data-meta".to_string(), "{1}".to_string());

        let pre = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "pre".to_string(),
            attributes: pre_attrs,
            children: vec![code_elem],
            self_closing: false,
        });

        let mut root = HRoot {
            id: id_gen.next_id(),
            span,
            children: vec![pre],
        };

        apply_code_meta(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(
            html.contains("data-line=\"1\" data-highlighted=\"\""),
            "line 1 raw should be highlighted: {html}"
        );
        assert!(
            !html.contains("data-line=\"2\" data-highlighted"),
            "line 2 raw should not be highlighted: {html}"
        );
    }
}
