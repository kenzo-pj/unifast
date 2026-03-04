/// Find a strikethrough span starting at `pos` in `text`.
/// Expects `~~` at `pos`. Returns `Some((content_start, end_pos))` where
/// `content_start` is the index after the opening `~~` and `end_pos` is
/// the index after the closing `~~`.
pub fn find_strikethrough(text: &str, pos: usize) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    if pos + 1 >= bytes.len() {
        return None;
    }
    if bytes[pos] != b'~' || bytes[pos + 1] != b'~' {
        return None;
    }

    let content_start = pos + 2;
    if content_start >= bytes.len() {
        return None;
    }

    // Content must not start with a space.
    if bytes[content_start] == b' ' || bytes[content_start] == b'\n' {
        return None;
    }

    // Search for closing `~~`.
    let mut i = content_start;
    while i + 1 < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            i += 2;
            continue;
        }
        if bytes[i] == b'~' && bytes[i + 1] == b'~' {
            // Content must not end with a space.
            if i > content_start && bytes[i - 1] != b' ' && bytes[i - 1] != b'\n' {
                return Some((content_start, i + 2));
            }
        }
        i += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_strikethrough() {
        let text = "~~deleted~~";
        let result = find_strikethrough(text, 0);
        assert_eq!(result, Some((2, 11)));
        assert_eq!(&text[2..9], "deleted");
    }

    #[test]
    fn test_strikethrough_in_middle() {
        let text = "hello ~~world~~ end";
        let result = find_strikethrough(text, 6);
        assert_eq!(result, Some((8, 15)));
        assert_eq!(&text[8..13], "world");
    }

    #[test]
    fn test_no_closing() {
        assert!(find_strikethrough("~~no close", 0).is_none());
    }

    #[test]
    fn test_space_after_opening() {
        assert!(find_strikethrough("~~ space~~", 0).is_none());
    }

    #[test]
    fn test_space_before_closing() {
        assert!(find_strikethrough("~~space ~~", 0).is_none());
    }

    #[test]
    fn test_not_tilde() {
        assert!(find_strikethrough("hello", 0).is_none());
    }

    #[test]
    fn test_single_tilde() {
        assert!(find_strikethrough("~not strike~", 0).is_none());
    }
}
