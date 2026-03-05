#[must_use]
pub fn try_match_url(text: &str, pos: usize) -> Option<(String, usize)> {
    if !text.is_char_boundary(pos) {
        return None;
    }
    let rest = &text[pos..];

    let has_scheme = rest.starts_with("http://") || rest.starts_with("https://");
    let has_www = rest.starts_with("www.");

    if !has_scheme && !has_www {
        return None;
    }

    let end = find_url_end(rest);
    if end == 0 {
        return None;
    }

    let raw_url = &rest[..end];

    if has_scheme {
        let scheme_len = if rest.starts_with("https://") { 8 } else { 7 };
        if end <= scheme_len {
            return None;
        }
    } else if end <= 4 {
        return None;
    }

    let url = if has_www && !has_scheme {
        format!("http://{raw_url}")
    } else {
        raw_url.to_string()
    };

    Some((url, end))
}

#[must_use]
pub fn try_match_email(text: &str, pos: usize) -> Option<(String, usize)> {
    if !text.is_char_boundary(pos) {
        return None;
    }
    let rest = &text[pos..];
    let bytes = rest.as_bytes();

    if bytes.is_empty() {
        return None;
    }

    if !bytes[0].is_ascii_alphanumeric() {
        return None;
    }

    let at_pos = bytes.iter().position(|&b| b == b'@')?;
    if at_pos == 0 {
        return None;
    }

    let local = &rest[..at_pos];
    if !local
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || b == b'+' || b == b'-')
    {
        return None;
    }

    let domain_start = at_pos + 1;
    if domain_start >= bytes.len() {
        return None;
    }

    if !bytes[domain_start].is_ascii_alphanumeric() {
        return None;
    }

    let mut end = domain_start;
    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_alphanumeric() || b == b'.' || b == b'-' {
            end += 1;
        } else {
            break;
        }
    }

    let domain = &rest[domain_start..end];
    if !domain.contains('.') {
        return None;
    }

    let last = domain.as_bytes().last().copied()?;
    if last == b'.' || last == b'-' {
        let trimmed_end = domain
            .bytes()
            .rev()
            .position(|b| b != b'.' && b != b'-')
            .map_or(domain_start, |p| end - p);
        if trimmed_end <= domain_start {
            return None;
        }
        let email = &rest[..trimmed_end];
        if !email[domain_start..].contains('.') {
            return None;
        }
        return Some((format!("mailto:{email}"), trimmed_end));
    }

    let email = &rest[..end];
    Some((format!("mailto:{email}"), end))
}

fn find_url_end(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut end = 0;

    while end < bytes.len() {
        let b = bytes[end];
        if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' || b < 0x20 {
            break;
        }
        if b == b'<' {
            break;
        }
        end += 1;
    }

    while end > 0 {
        let last = bytes[end - 1];
        if last == b'.'
            || last == b','
            || last == b';'
            || last == b':'
            || last == b'!'
            || last == b'?'
            || last == b'\''
            || last == b'"'
        {
            end -= 1;
        } else if last == b')' {
            let open = bytes[..end].iter().filter(|&&b| b == b'(').count();
            let close = bytes[..end].iter().filter(|&&b| b == b')').count();
            if close > open {
                end -= 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_url() {
        let (url, len) = try_match_url("http://example.com rest", 0).unwrap();
        assert_eq!(url, "http://example.com");
        assert_eq!(len, 18);
    }

    #[test]
    fn test_https_url() {
        let (url, len) = try_match_url("https://example.com/path", 0).unwrap();
        assert_eq!(url, "https://example.com/path");
        assert_eq!(len, 24);
    }

    #[test]
    fn test_www_url() {
        let (url, len) = try_match_url("www.example.com more", 0).unwrap();
        assert_eq!(url, "http://www.example.com");
        assert_eq!(len, 15);
    }

    #[test]
    fn test_url_trailing_punctuation() {
        let (url, len) = try_match_url("http://example.com.", 0).unwrap();
        assert_eq!(url, "http://example.com");
        assert_eq!(len, 18);
    }

    #[test]
    fn test_no_url() {
        assert!(try_match_url("just text", 0).is_none());
        assert!(try_match_url("ftp://other.com", 0).is_none());
    }

    #[test]
    fn test_http_scheme_only() {
        assert!(try_match_url("http:// ", 0).is_none());
    }

    #[test]
    fn test_email_basic() {
        let (url, len) = try_match_email("user@example.com rest", 0).unwrap();
        assert_eq!(url, "mailto:user@example.com");
        assert_eq!(len, 16);
    }

    #[test]
    fn test_email_with_dots() {
        let (url, _) = try_match_email("first.last@example.co.uk", 0).unwrap();
        assert_eq!(url, "mailto:first.last@example.co.uk");
    }

    #[test]
    fn test_email_no_domain_dot() {
        assert!(try_match_email("user@localhost", 0).is_none());
    }

    #[test]
    fn test_not_email() {
        assert!(try_match_email("not-an-email", 0).is_none());
        assert!(try_match_email("@nope", 0).is_none());
    }

    #[test]
    fn test_www_alone_not_url() {
        assert!(try_match_url("www. more", 0).is_none());
    }

    #[test]
    fn test_url_non_char_boundary_returns_none() {
        let text = "—http://example.com";
        assert!(try_match_url(text, 1).is_none());
        assert!(try_match_url(text, 2).is_none());
    }

    #[test]
    fn test_email_non_char_boundary_returns_none() {
        let text = "—user@example.com";
        assert!(try_match_email(text, 1).is_none());
        assert!(try_match_email(text, 2).is_none());
    }

    #[test]
    fn test_url_after_multibyte_char() {
        let text = "—http://example.com";
        let (url, len) = try_match_url(text, 3).unwrap();
        assert_eq!(url, "http://example.com");
        assert_eq!(len, 18);
    }
}
