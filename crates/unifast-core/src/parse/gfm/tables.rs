use crate::ast::mdast::nodes::AlignKind;

#[must_use]
pub fn is_table_separator(line: &str) -> Option<Vec<AlignKind>> {
    let trimmed = line.trim();
    if !trimmed.contains('-') {
        return None;
    }

    let cells = split_table_row(trimmed);
    if cells.is_empty() {
        return None;
    }

    let mut aligns = Vec::new();
    for cell in &cells {
        let c = cell.trim();
        if c.is_empty() {
            return None;
        }
        if !c.bytes().all(|b| b == b'-' || b == b':') {
            return None;
        }
        if !c.contains('-') {
            return None;
        }

        let left = c.starts_with(':');
        let right = c.ends_with(':');
        let align = match (left, right) {
            (true, true) => AlignKind::Center,
            (true, false) => AlignKind::Left,
            (false, true) => AlignKind::Right,
            (false, false) => AlignKind::None,
        };
        aligns.push(align);
    }

    Some(aligns)
}

#[must_use]
pub fn parse_table_row(line: &str) -> Vec<String> {
    split_table_row(line.trim())
        .into_iter()
        .map(|s| s.trim().to_string())
        .collect()
}

fn split_table_row(trimmed: &str) -> Vec<String> {
    let mut cells: Vec<String> = Vec::new();
    let mut current = String::new();
    let bytes = trimmed.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() && bytes[i + 1] == b'|' {
            current.push('|');
            i += 2;
        } else if bytes[i] == b'|' {
            cells.push(current.clone());
            current.clear();
            i += 1;
        } else {
            current.push(trimmed[i..].chars().next().unwrap());
            i += trimmed[i..].chars().next().unwrap().len_utf8();
        }
    }
    cells.push(current);

    if cells.first().is_some_and(|c| c.trim().is_empty()) {
        cells.remove(0);
    }
    if cells.last().is_some_and(|c| c.trim().is_empty()) {
        cells.pop();
    }

    cells
}

#[must_use]
pub fn could_be_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains('|')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separator_basic() {
        let aligns = is_table_separator("|---|---|").unwrap();
        assert_eq!(aligns.len(), 2);
        assert_eq!(aligns[0], AlignKind::None);
        assert_eq!(aligns[1], AlignKind::None);
    }

    #[test]
    fn test_separator_alignment_left() {
        let aligns = is_table_separator("|:---|").unwrap();
        assert_eq!(aligns[0], AlignKind::Left);
    }

    #[test]
    fn test_separator_alignment_center() {
        let aligns = is_table_separator("|:---:|").unwrap();
        assert_eq!(aligns[0], AlignKind::Center);
    }

    #[test]
    fn test_separator_alignment_right() {
        let aligns = is_table_separator("|---:|").unwrap();
        assert_eq!(aligns[0], AlignKind::Right);
    }

    #[test]
    fn test_separator_mixed_alignment() {
        let aligns = is_table_separator("|:--|:--:|--:|---| ").unwrap();
        assert_eq!(aligns.len(), 4);
        assert_eq!(aligns[0], AlignKind::Left);
        assert_eq!(aligns[1], AlignKind::Center);
        assert_eq!(aligns[2], AlignKind::Right);
        assert_eq!(aligns[3], AlignKind::None);
    }

    #[test]
    fn test_separator_invalid() {
        assert!(is_table_separator("not a separator").is_none());
        assert!(is_table_separator("|abc|").is_none());
        assert!(is_table_separator("| |").is_none());
    }

    #[test]
    fn test_parse_table_row_basic() {
        let cells = parse_table_row("| a | b | c |");
        assert_eq!(cells, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_table_row_no_outer_pipes() {
        let cells = parse_table_row("a | b | c");
        assert_eq!(cells, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_table_row_escaped_pipe() {
        let cells = parse_table_row("| a\\|b | c |");
        assert_eq!(cells, vec!["a|b", "c"]);
    }

    #[test]
    fn test_could_be_table_row() {
        assert!(could_be_table_row("| a | b |"));
        assert!(!could_be_table_row("just text"));
    }
}
