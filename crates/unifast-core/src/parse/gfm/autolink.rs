/// Try to match an extended autolink URL starting at `pos` in `text`.
/// Handles `http://`, `https://`, and `www.` prefixes.
/// Returns `Some((url, bytes_consumed))` if a URL is found.
pub fn try_match_url(text: &str, pos: usize) -> Option<(String, usize)> {
    if !text.is_char_boundary(pos) {
        return None;
    }
    let rest = &text[pos..];

    // Check for http:// or https://
    let has_scheme = rest.starts_with("http://") || rest.starts_with("https://");
    let has_www = rest.starts_with("www.");

    if !has_scheme && !has_www {
        return None;
    }

    // Find the end of the URL.
    let end = find_url_end(rest);
    if end == 0 {
        return None;
    }

    let raw_url = &rest[..end];

    // Must have at least something after the scheme or www.
    if has_scheme {
        let scheme_len = if rest.starts_with("https://") { 8 } else { 7 };
        if end <= scheme_len {
            return None;
        }
    } else if end <= 4 {
        // "www." alone is not a URL.
        return None;
    }

    let url = if has_www && !has_scheme {
        format!("http://{raw_url}")
    } else {
        raw_url.to_string()
    };

    Some((url, end))
}

/// Try to match an extended autolink email starting at `pos` in `text`.
/// Matches patterns like `user@example.com`.
/// Returns `Some((mailto_url, bytes_consumed))` if an email is found.
pub fn try_match_email(text: &str, pos: usize) -> Option<(String, usize)> {
    if !text.is_char_boundary(pos) {
        return None;
    }
    let rest = &text[pos..];
    let bytes = rest.as_bytes();

    if bytes.is_empty() {
        return None;
    }

    // The local part must start with an alphanumeric character.
    if !bytes[0].is_ascii_alphanumeric() {
        return None;
    }

    // Find the `@` sign.
    let at_pos = bytes.iter().position(|&b| b == b'@')?;
    if at_pos == 0 {
        return None;
    }

    // Validate local part (before @): alphanumeric, `.`, `_`, `+`, `-`
    let local = &rest[..at_pos];
    if !local
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || b == b'+' || b == b'-')
    {
        return None;
    }

    // After the `@`, find the domain.
    let domain_start = at_pos + 1;
    if domain_start >= bytes.len() {
        return None;
    }

    // Domain must start with alphanumeric.
    if !bytes[domain_start].is_ascii_alphanumeric() {
        return None;
    }

    // Find end of domain.
    let mut end = domain_start;
    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_alphanumeric() || b == b'.' || b == b'-' {
            end += 1;
        } else {
            break;
        }
    }

    // Domain must contain at least one dot.
    let domain = &rest[domain_start..end];
    if !domain.contains('.') {
        return None;
    }

    // Domain must not end with a dot or hyphen.
    let last = domain.as_bytes().last().copied()?;
    if last == b'.' || last == b'-' {
        // Trim trailing dots/hyphens.
        let trimmed_end = domain
            .bytes()
            .rev()
            .position(|b| b != b'.' && b != b'-')
            .map(|p| end - p)
            .unwrap_or(domain_start);
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

/// Find the end of a URL in text, handling trailing punctuation stripping.
fn find_url_end(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut end = 0;

    while end < bytes.len() {
        let b = bytes[end];
        // URL ends at whitespace or control chars.
        if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' || b < 0x20 {
            break;
        }
        // Also stop at certain characters that typically end URLs in prose.
        if b == b'<' {
            break;
        }
        end += 1;
    }

    // Strip trailing punctuation that is unlikely part of the URL.
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
            // Only strip trailing `)` if there are more `)` than `(` in the URL.
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
        // "—" is U+2014 (3 bytes: 0xE2 0x80 0x94).
        // Passing pos=1 is inside the multi-byte char → should return None, not panic.
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
        // pos=3 is the char boundary after "—", should find the URL.
        let text = "—http://example.com";
        let (url, len) = try_match_url(text, 3).unwrap();
        assert_eq!(url, "http://example.com");
        assert_eq!(len, 18);
    }
}
