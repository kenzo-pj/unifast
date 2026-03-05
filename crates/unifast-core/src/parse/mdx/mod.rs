pub mod esm;
pub mod expr;
pub mod jsx;

use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::{Document, MdNode, MdxjsEsm};
use crate::diagnostics::sink::DiagnosticSink;
use crate::parse::FrontmatterData;

pub struct MdxParseResult {
    pub document: Document,
    pub diagnostics: DiagnosticSink,
    pub frontmatter: FrontmatterData,
}

#[must_use]
pub fn parse_mdx(input: &str) -> MdxParseResult {
    let mut id_gen = NodeIdGen::new();
    let mut diagnostics = DiagnosticSink::new();

    let (frontmatter_data, parse_offset) =
        if let Some(fm) = crate::parse::frontmatter::extract_frontmatter(input) {
            (fm.data, fm.end_offset)
        } else {
            (FrontmatterData::new(), 0)
        };

    let content = &input[parse_offset..];

    let mut children: Vec<MdNode> = Vec::new();
    let mut current_md_lines: Vec<String> = Vec::new();
    let mut current_md_start = parse_offset;
    let mut in_fenced_code = false;
    let mut fence_marker: String = String::new();

    let lines: Vec<&str> = content.lines().collect();
    let mut line_idx = 0;

    while line_idx < lines.len() {
        let line = lines[line_idx];
        let line_offset = calculate_line_offset(content, line_idx) + parse_offset;

        let trimmed = line.trim_start();
        if in_fenced_code {
            if trimmed.starts_with(&fence_marker) && trimmed[fence_marker.len()..].trim().is_empty()
            {
                in_fenced_code = false;
                fence_marker.clear();
            }
            current_md_lines.push(line.to_string());
            line_idx += 1;
            continue;
        }
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            let ch = trimmed.as_bytes()[0];
            let fence_len = trimmed.bytes().take_while(|&b| b == ch).count();
            fence_marker = std::iter::repeat_n(ch as char, fence_len).collect();
            in_fenced_code = true;
            current_md_lines.push(line.to_string());
            line_idx += 1;
            continue;
        }

        if esm::is_esm_line(line) {
            flush_markdown(
                &mut current_md_lines,
                current_md_start,
                &mut children,
                &mut id_gen,
                &mut diagnostics,
            );

            let esm_start = line_offset;
            let mut esm_lines = vec![line.to_string()];
            line_idx += 1;
            while line_idx < lines.len() {
                let next = lines[line_idx];
                if esm::is_esm_continuation(next, &esm_lines) {
                    esm_lines.push(next.to_string());
                    line_idx += 1;
                } else {
                    break;
                }
            }
            let esm_text = esm_lines.join("\n");
            let esm_end = esm_start + esm_text.len();
            children.push(MdNode::MdxjsEsm(MdxjsEsm {
                id: id_gen.next_id(),
                span: Span::new(esm_start as u32, esm_end as u32),
                value: esm_text,
            }));
            current_md_start = esm_end + 1;
            continue;
        }

        if let Some(jsx_result) =
            jsx::try_parse_jsx_flow(line, &lines[line_idx..], line_offset, &mut id_gen)
        {
            flush_markdown(
                &mut current_md_lines,
                current_md_start,
                &mut children,
                &mut id_gen,
                &mut diagnostics,
            );
            children.push(jsx_result.node);
            line_idx += jsx_result.lines_consumed;
            current_md_start = line_offset + jsx_result.bytes_consumed;
            continue;
        }

        if let Some(expr_node) = expr::try_parse_flow_expression(line, line_offset, &mut id_gen) {
            flush_markdown(
                &mut current_md_lines,
                current_md_start,
                &mut children,
                &mut id_gen,
                &mut diagnostics,
            );
            children.push(expr_node);
            line_idx += 1;
            current_md_start = line_offset + line.len() + 1;
            continue;
        }

        current_md_lines.push(line.to_string());
        line_idx += 1;
    }

    flush_markdown(
        &mut current_md_lines,
        current_md_start,
        &mut children,
        &mut id_gen,
        &mut diagnostics,
    );

    let doc = Document {
        id: id_gen.next_id(),
        span: Span::new(0, input.len() as u32),
        children,
    };

    MdxParseResult {
        document: doc,
        diagnostics,
        frontmatter: frontmatter_data,
    }
}

fn flush_markdown(
    lines: &mut Vec<String>,
    start_offset: usize,
    children: &mut Vec<MdNode>,
    id_gen: &mut NodeIdGen,
    diagnostics: &mut DiagnosticSink,
) {
    if lines.is_empty() {
        return;
    }
    let md_text = lines.join("\n") + "\n";
    let md_nodes = parse_markdown_fragment(&md_text, start_offset, id_gen, diagnostics);
    children.extend(md_nodes);
    lines.clear();
}

fn parse_markdown_fragment(
    text: &str,
    _offset: usize,
    id_gen: &mut NodeIdGen,
    diagnostics: &mut DiagnosticSink,
) -> Vec<MdNode> {
    use crate::parse::markdown::parser;
    let doc = parser::parse(text, id_gen, diagnostics);
    doc.children
}

fn calculate_line_offset(content: &str, line_idx: usize) -> usize {
    content.lines().take(line_idx).map(|l| l.len() + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mdx_plain_markdown() {
        let r = parse_mdx("# Hello\n\nWorld\n");
        assert!(!r.document.children.is_empty());
        assert!(
            r.document
                .children
                .iter()
                .any(|n| matches!(n, MdNode::Heading(_)))
        );
    }

    #[test]
    fn mdx_esm_import() {
        let r = parse_mdx("import { Button } from './Button'\n\n# Hello\n");
        assert!(matches!(&r.document.children[0], MdNode::MdxjsEsm(_)));
        if let MdNode::MdxjsEsm(esm) = &r.document.children[0] {
            assert!(esm.value.contains("import"));
            assert!(esm.value.contains("Button"));
        }
    }

    #[test]
    fn mdx_esm_export() {
        let r = parse_mdx("export const meta = { title: 'Hello' }\n\n# Hello\n");
        assert!(matches!(&r.document.children[0], MdNode::MdxjsEsm(_)));
        if let MdNode::MdxjsEsm(esm) = &r.document.children[0] {
            assert!(esm.value.starts_with("export"));
        }
    }

    #[test]
    fn mdx_esm_multiline_import() {
        let input = "import {\n  Button,\n  Card,\n} from './ui'\n\n# Hello\n";
        let r = parse_mdx(input);
        assert!(matches!(&r.document.children[0], MdNode::MdxjsEsm(_)));
        if let MdNode::MdxjsEsm(esm) = &r.document.children[0] {
            assert!(esm.value.contains("Button"));
            assert!(esm.value.contains("Card"));
        }
    }

    #[test]
    fn mdx_jsx_self_closing() {
        let r = parse_mdx("# Hello\n\n<Button />\n");
        assert!(
            r.document
                .children
                .iter()
                .any(|n| matches!(n, MdNode::MdxJsxFlowElement(_)))
        );
    }

    #[test]
    fn mdx_jsx_with_children() {
        let r = parse_mdx("<Card>\n  # Title\n</Card>\n");
        assert!(matches!(
            &r.document.children[0],
            MdNode::MdxJsxFlowElement(_)
        ));
    }

    #[test]
    fn mdx_jsx_with_props() {
        let r = parse_mdx("<Button variant=\"primary\" disabled />\n");
        if let MdNode::MdxJsxFlowElement(el) = &r.document.children[0] {
            assert_eq!(el.name.as_deref(), Some("Button"));
            assert!(
                el.attributes
                    .iter()
                    .any(|a| a.name == "variant" && a.value.as_deref() == Some("primary"))
            );
            assert!(
                el.attributes
                    .iter()
                    .any(|a| a.name == "disabled" && a.value.is_none())
            );
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }

    #[test]
    fn mdx_jsx_expression_prop() {
        let r = parse_mdx("<Button onClick={handleClick} />\n");
        if let MdNode::MdxJsxFlowElement(el) = &r.document.children[0] {
            assert!(
                el.attributes
                    .iter()
                    .any(|a| a.name == "onClick" && a.value.as_deref() == Some("{handleClick}"))
            );
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }

    #[test]
    fn mdx_flow_expression() {
        let r = parse_mdx("{someVariable}\n");
        assert!(matches!(
            &r.document.children[0],
            MdNode::MdxFlowExpression(_)
        ));
    }

    #[test]
    fn mdx_flow_expression_complex() {
        let r = parse_mdx("{items.map(i => <li>{i}</li>)}\n");
        if let MdNode::MdxFlowExpression(expr) = &r.document.children[0] {
            assert!(expr.value.contains("items.map"));
        } else {
            panic!("expected flow expression");
        }
    }

    #[test]
    fn mdx_frontmatter() {
        let r = parse_mdx("---\ntitle: MDX Doc\n---\n\n# Hello\n");
        assert_eq!(
            r.frontmatter.get("title").and_then(|v| v.as_str()),
            Some("MDX Doc")
        );
    }

    #[test]
    fn mdx_mixed_content() {
        let input = "import X from 'x'\n\n# Title\n\n<Card />\n\nSome text\n\n{expr}\n";
        let r = parse_mdx(input);
        let has_esm = r
            .document
            .children
            .iter()
            .any(|n| matches!(n, MdNode::MdxjsEsm(_)));
        let has_heading = r
            .document
            .children
            .iter()
            .any(|n| matches!(n, MdNode::Heading(_)));
        let has_jsx = r
            .document
            .children
            .iter()
            .any(|n| matches!(n, MdNode::MdxJsxFlowElement(_)));
        let has_expr = r
            .document
            .children
            .iter()
            .any(|n| matches!(n, MdNode::MdxFlowExpression(_)));
        assert!(has_esm, "should have ESM");
        assert!(has_heading, "should have heading");
        assert!(has_jsx, "should have JSX");
        assert!(has_expr, "should have expression");
    }

    #[test]
    fn mdx_empty_input() {
        let r = parse_mdx("");
        assert!(r.document.children.is_empty());
    }

    #[test]
    fn mdx_only_markdown() {
        let r = parse_mdx("Hello **world**\n");
        assert!(!r.document.children.is_empty());
        assert!(
            r.document
                .children
                .iter()
                .any(|n| matches!(n, MdNode::Paragraph(_)))
        );
    }

    #[test]
    fn mdx_multiple_imports() {
        let input = "import A from 'a'\nimport B from 'b'\n\n# Hello\n";
        let r = parse_mdx(input);
        let esm_count = r
            .document
            .children
            .iter()
            .filter(|n| matches!(n, MdNode::MdxjsEsm(_)))
            .count();
        assert!(esm_count >= 2, "expected 2+ ESM nodes, got {esm_count}");
    }

    #[test]
    fn mdx_jsx_dotted_name() {
        let r = parse_mdx("<Form.Input />\n");
        if let MdNode::MdxJsxFlowElement(el) = &r.document.children[0] {
            assert_eq!(el.name.as_deref(), Some("Form.Input"));
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }

    #[test]
    fn mdx_no_diagnostics_for_valid() {
        let r = parse_mdx("import X from 'x'\n\n# Hello\n\n<Card />\n");
        assert!(r.diagnostics.is_empty());
    }

    #[test]
    fn mdx_jsx_with_markdown_children() {
        let r = parse_mdx("<Callout>\n\n**Important**: read this\n\n</Callout>\n");
        if let MdNode::MdxJsxFlowElement(el) = &r.document.children[0] {
            assert_eq!(el.name.as_deref(), Some("Callout"));
            assert!(!el.children.is_empty());
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }

    #[test]
    fn mdx_export_default() {
        let r = parse_mdx(
            "export default function Layout({ children }) { return <div>{children}</div> }\n",
        );
        assert!(matches!(&r.document.children[0], MdNode::MdxjsEsm(_)));
    }

    #[test]
    fn mdx_import_inside_code_block_not_treated_as_esm() {
        let input = "```ts\nimport { compile } from \"@unifast/node\";\n```\n";
        let r = parse_mdx(input);
        let esm_count = r
            .document
            .children
            .iter()
            .filter(|n| matches!(n, MdNode::MdxjsEsm(_)))
            .count();
        assert_eq!(esm_count, 0, "import inside code block should not be ESM");
        assert!(
            r.document
                .children
                .iter()
                .any(|n| matches!(n, MdNode::Code(_))),
            "should have a Code node"
        );
    }

    #[test]
    fn mdx_jsx_inside_code_block_not_treated_as_jsx() {
        let input = "```jsx\n<Button onClick={handler} />\n```\n";
        let r = parse_mdx(input);
        let jsx_count = r
            .document
            .children
            .iter()
            .filter(|n| matches!(n, MdNode::MdxJsxFlowElement(_)))
            .count();
        assert_eq!(
            jsx_count, 0,
            "JSX inside code block should not be parsed as JSX flow"
        );
    }

    #[test]
    fn mdx_jsx_string_attr_single_quote() {
        let r = parse_mdx("<Button label='hello' />\n");
        if let MdNode::MdxJsxFlowElement(el) = &r.document.children[0] {
            assert!(
                el.attributes
                    .iter()
                    .any(|a| a.name == "label" && a.value.as_deref() == Some("hello"))
            );
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }
}
