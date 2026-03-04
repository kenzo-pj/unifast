/// Check if text starts with a task list marker: `[ ] ` or `[x] ` or `[X] `.
/// Returns `Some((is_checked, bytes_consumed))` if a task marker is found.
/// The `bytes_consumed` count includes the trailing space after the bracket.
pub fn parse_task_marker(text: &str) -> Option<(bool, usize)> {
    let bytes = text.as_bytes();
    if bytes.len() < 4 {
        return None;
    }
    if bytes[0] != b'[' {
        return None;
    }
    let checked = match bytes[1] {
        b' ' => false,
        b'x' | b'X' => true,
        _ => return None,
    };
    if bytes[2] != b']' {
        return None;
    }
    if bytes[3] != b' ' {
        return None;
    }
    Some((checked, 4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unchecked_marker() {
        let result = parse_task_marker("[ ] todo item");
        assert_eq!(result, Some((false, 4)));
    }

    #[test]
    fn test_checked_marker_lowercase() {
        let result = parse_task_marker("[x] done item");
        assert_eq!(result, Some((true, 4)));
    }

    #[test]
    fn test_checked_marker_uppercase() {
        let result = parse_task_marker("[X] DONE item");
        assert_eq!(result, Some((true, 4)));
    }

    #[test]
    fn test_no_marker() {
        assert!(parse_task_marker("regular text").is_none());
    }

    #[test]
    fn test_no_space_after_bracket() {
        assert!(parse_task_marker("[x]no space").is_none());
    }

    #[test]
    fn test_invalid_check_char() {
        assert!(parse_task_marker("[o] other").is_none());
    }

    #[test]
    fn test_too_short() {
        assert!(parse_task_marker("[x]").is_none());
        assert!(parse_task_marker("").is_none());
    }
}
