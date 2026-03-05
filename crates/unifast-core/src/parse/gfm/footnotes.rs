#[must_use]
pub fn is_footnote_definition(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim_start();
    let indent = line.len() - trimmed.len();
    if indent > 3 {
        return None;
    }

    if !trimmed.starts_with("[^") {
        return None;
    }

    let bytes = trimmed.as_bytes();
    let mut i = 2;
    while i < bytes.len() && bytes[i] != b']' {
        let b = bytes[i];
        if !b.is_ascii_alphanumeric() && b != b'-' && b != b'_' {
            return None;
        }
        i += 1;
    }

    if i >= bytes.len() || bytes[i] != b']' {
        return None;
    }

    let identifier = &trimmed[2..i];
    if identifier.is_empty() {
        return None;
    }

    i += 1;
    if i >= bytes.len() || bytes[i] != b':' {
        return None;
    }
    i += 1;

    if i < bytes.len() && bytes[i] == b' ' {
        i += 1;
    }

    let content = &trimmed[i..];
    Some((identifier, content))
}

#[must_use]
pub fn is_footnote_reference(text: &str, pos: usize) -> Option<(&str, usize)> {
    let rest = &text[pos..];
    let bytes = rest.as_bytes();

    if bytes.len() < 4 {
        return None;
    }
    if bytes[0] != b'[' || bytes[1] != b'^' {
        return None;
    }

    let mut i = 2;
    while i < bytes.len() && bytes[i] != b']' {
        let b = bytes[i];
        if !b.is_ascii_alphanumeric() && b != b'-' && b != b'_' {
            return None;
        }
        i += 1;
    }

    if i >= bytes.len() || bytes[i] != b']' {
        return None;
    }

    let identifier = &rest[2..i];
    if identifier.is_empty() {
        return None;
    }

    let consumed = i + 1;
    if consumed < bytes.len()
        && (bytes[consumed] == b'(' || bytes[consumed] == b'[' || bytes[consumed] == b':')
    {
        return None;
    }

    Some((identifier, consumed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_footnote_definition_basic() {
        let (id, content) = is_footnote_definition("[^1]: This is the footnote.").unwrap();
        assert_eq!(id, "1");
        assert_eq!(content, "This is the footnote.");
    }

    #[test]
    fn test_footnote_definition_alphanumeric() {
        let (id, content) = is_footnote_definition("[^note-1]: Content here").unwrap();
        assert_eq!(id, "note-1");
        assert_eq!(content, "Content here");
    }

    #[test]
    fn test_footnote_definition_empty_content() {
        let (id, content) = is_footnote_definition("[^abc]:").unwrap();
        assert_eq!(id, "abc");
        assert_eq!(content, "");
    }

    #[test]
    fn test_not_footnote_definition() {
        assert!(is_footnote_definition("regular text").is_none());
        assert!(is_footnote_definition("[not]: a footnote").is_none());
        assert!(is_footnote_definition("[^]: empty id").is_none());
    }

    #[test]
    fn test_footnote_reference_basic() {
        let (id, consumed) = is_footnote_reference("text [^1] more", 5).unwrap();
        assert_eq!(id, "1");
        assert_eq!(consumed, 4);
    }

    #[test]
    fn test_footnote_reference_at_start() {
        let (id, consumed) = is_footnote_reference("[^note]", 0).unwrap();
        assert_eq!(id, "note");
        assert_eq!(consumed, 7);
    }

    #[test]
    fn test_footnote_reference_not_link() {
        assert!(is_footnote_reference("[^id](url)", 0).is_none());
    }

    #[test]
    fn test_footnote_reference_invalid() {
        assert!(is_footnote_reference("[^]", 0).is_none());
        assert!(is_footnote_reference("abc", 0).is_none());
    }
}
