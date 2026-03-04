/// Returns `true` if the given tag name is an HTML void element (self-closing, no end tag).
pub fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn void_elements() {
        assert!(is_void_element("br"));
        assert!(is_void_element("hr"));
        assert!(is_void_element("img"));
        assert!(is_void_element("input"));
        assert!(is_void_element("meta"));
        assert!(is_void_element("link"));
        assert!(is_void_element("area"));
        assert!(is_void_element("base"));
        assert!(is_void_element("col"));
        assert!(is_void_element("embed"));
        assert!(is_void_element("param"));
        assert!(is_void_element("source"));
        assert!(is_void_element("track"));
        assert!(is_void_element("wbr"));
    }

    #[test]
    fn non_void_elements() {
        assert!(!is_void_element("div"));
        assert!(!is_void_element("p"));
        assert!(!is_void_element("a"));
        assert!(!is_void_element("span"));
        assert!(!is_void_element("table"));
        assert!(!is_void_element("script"));
    }
}
