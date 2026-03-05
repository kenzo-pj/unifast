use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::{MdNode, MdxExpression};

pub fn try_parse_flow_expression(
    line: &str,
    offset: usize,
    id_gen: &mut NodeIdGen,
) -> Option<MdNode> {
    let trimmed = line.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return None;
    }

    let mut depth: i32 = 0;
    for ch in trimmed.chars() {
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return None;
        }
    }
    if depth != 0 {
        return None;
    }

    let inner = &trimmed[1..trimmed.len() - 1];
    let inner = inner.trim();
    if inner.is_empty() {
        return None;
    }

    Some(MdNode::MdxFlowExpression(MdxExpression {
        id: id_gen.next_id(),
        span: Span::new(offset as u32, (offset + line.len()) as u32),
        value: inner.to_string(),
    }))
}

#[must_use]
pub fn find_inline_expression(text: &str, pos: usize) -> Option<(String, usize)> {
    if !text[pos..].starts_with('{') {
        return None;
    }
    let mut depth: i32 = 0;
    for (i, ch) in text[pos..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let end = pos + i + 1;
                    let inner = text[pos + 1..pos + i].to_string();
                    return Some((inner, end));
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_expression() {
        let mut id_gen = NodeIdGen::new();
        let result = try_parse_flow_expression("{someVar}", 0, &mut id_gen);
        assert!(result.is_some());
        if let Some(MdNode::MdxFlowExpression(expr)) = result {
            assert_eq!(expr.value, "someVar");
        }
    }

    #[test]
    fn parse_expression_with_spaces() {
        let mut id_gen = NodeIdGen::new();
        let result = try_parse_flow_expression("  { a + b }  ", 0, &mut id_gen);
        assert!(result.is_some());
        if let Some(MdNode::MdxFlowExpression(expr)) = result {
            assert_eq!(expr.value, "a + b");
        }
    }

    #[test]
    fn parse_nested_braces() {
        let mut id_gen = NodeIdGen::new();
        let result = try_parse_flow_expression("{obj.map(x => { return x })}", 0, &mut id_gen);
        assert!(result.is_some());
        if let Some(MdNode::MdxFlowExpression(expr)) = result {
            assert_eq!(expr.value, "obj.map(x => { return x })");
        }
    }

    #[test]
    fn reject_empty_braces() {
        let mut id_gen = NodeIdGen::new();
        assert!(try_parse_flow_expression("{}", 0, &mut id_gen).is_none());
        assert!(try_parse_flow_expression("{   }", 0, &mut id_gen).is_none());
    }

    #[test]
    fn reject_non_expression() {
        let mut id_gen = NodeIdGen::new();
        assert!(try_parse_flow_expression("hello", 0, &mut id_gen).is_none());
        assert!(try_parse_flow_expression("# heading", 0, &mut id_gen).is_none());
    }

    #[test]
    fn reject_unbalanced_braces() {
        let mut id_gen = NodeIdGen::new();
        assert!(try_parse_flow_expression("{foo", 0, &mut id_gen).is_none());
        assert!(try_parse_flow_expression("foo}", 0, &mut id_gen).is_none());
    }

    #[test]
    fn inline_expression_simple() {
        let text = "hello {world} end";
        let result = find_inline_expression(text, 6);
        assert!(result.is_some());
        let (inner, end) = result.unwrap();
        assert_eq!(inner, "world");
        assert_eq!(end, 13);
    }

    #[test]
    fn inline_expression_nested() {
        let text = "x {a.map(b => {b})} y";
        let result = find_inline_expression(text, 2);
        assert!(result.is_some());
        let (inner, end) = result.unwrap();
        assert_eq!(inner, "a.map(b => {b})");
        assert_eq!(end, 19);
    }

    #[test]
    fn inline_expression_not_at_brace() {
        let text = "hello world";
        assert!(find_inline_expression(text, 0).is_none());
    }

    #[test]
    fn offset_preserved_in_span() {
        let mut id_gen = NodeIdGen::new();
        let result = try_parse_flow_expression("{x}", 42, &mut id_gen);
        if let Some(MdNode::MdxFlowExpression(expr)) = result {
            assert_eq!(expr.span.start, 42);
            assert_eq!(expr.span.end, 45);
        } else {
            panic!("expected flow expression");
        }
    }
}
