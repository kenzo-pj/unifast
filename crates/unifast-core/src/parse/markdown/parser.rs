use std::collections::HashMap;

use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::*;
use crate::diagnostics::sink::DiagnosticSink;
use crate::parse::gfm;

// ── Public entry point ───────────────────────────────────────────────

/// Parse a Markdown source string into a `Document` AST node.
pub fn parse(source: &str, id_gen: &mut NodeIdGen, diagnostics: &mut DiagnosticSink) -> Document {
    let mut parser = Parser::new(source, id_gen, diagnostics);
    parser.parse_document()
}

/// Parse a Markdown source string starting from a byte offset.
///
/// This is used when frontmatter has been extracted from the beginning of the
/// input.  By keeping the original source and simply starting from `offset`,
/// all `Span` values remain correct relative to the original input.
pub fn parse_from_offset(
    source: &str,
    offset: usize,
    id_gen: &mut NodeIdGen,
    diagnostics: &mut DiagnosticSink,
) -> Document {
    if offset == 0 {
        return parse(source, id_gen, diagnostics);
    }
    let mut parser = Parser::new(source, id_gen, diagnostics);
    parser.parse_document_from(offset)
}

// ── ASCII punctuation for backslash escapes ──────────────────────────

const ESCAPABLE: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

fn is_escapable(ch: u8) -> bool {
    ESCAPABLE.contains(&ch)
}

// ── Parser ───────────────────────────────────────────────────────────

struct Parser<'a> {
    source: &'a str,
    pos: usize,
    id_gen: &'a mut NodeIdGen,
    diagnostics: &'a mut DiagnosticSink,
    /// Link reference definitions: label (lower-cased) -> (url, optional title).
    definitions: HashMap<String, (String, Option<String>)>,
}

impl<'a> Parser<'a> {
    fn new(
        source: &'a str,
        id_gen: &'a mut NodeIdGen,
        diagnostics: &'a mut DiagnosticSink,
    ) -> Self {
        Self {
            source,
            pos: 0,
            id_gen,
            diagnostics,
            definitions: HashMap::new(),
        }
    }

    // ── Document ─────────────────────────────────────────────────────

    fn parse_document(&mut self) -> Document {
        // First pass: collect link reference definitions.
        self.collect_definitions();

        // Reset position for the real parse.
        self.pos = 0;

        let children = self.parse_blocks();
        let end = self.source.len() as u32;
        Document {
            id: self.id_gen.next_id(),
            span: Span::new(0, end),
            children,
        }
    }

    /// Parse the document starting from a byte offset (used after frontmatter
    /// extraction so that spans remain relative to the original source).
    fn parse_document_from(&mut self, offset: usize) -> Document {
        // Collect link reference definitions starting from the offset.
        self.pos = offset;
        while self.pos < self.source.len() {
            let line_start = self.pos;
            let line = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };
            if self.try_collect_definition(&line, line_start) {
                // consumed
            } else {
                self.advance_line();
            }
        }

        // Reset to the offset for the real parse.
        self.pos = offset;

        let children = self.parse_blocks();
        let end = self.source.len() as u32;
        Document {
            id: self.id_gen.next_id(),
            span: Span::new(offset as u32, end),
            children,
        }
    }

    // ── Definition collection pass ───────────────────────────────────

    fn collect_definitions(&mut self) {
        self.pos = 0;
        while self.pos < self.source.len() {
            let line_start = self.pos;
            // Get the line content as an owned String to avoid borrow conflict.
            let line = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };
            if self.try_collect_definition(&line, line_start) {
                // consumed, pos already advanced inside
            } else {
                self.advance_line();
            }
        }
    }

    /// Try to parse a link reference definition starting at `line_start`.
    /// Returns true if we consumed it (and advanced pos past it).
    fn try_collect_definition(&mut self, line: &str, line_start: usize) -> bool {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        if indent > 3 {
            return false;
        }
        if !trimmed.starts_with('[') {
            return false;
        }
        // We need to parse: [label]: <url> "title"
        // across potentially multiple lines. Work on the remaining source.
        let src = &self.source[line_start + indent..];
        if let Some((label, url, title, consumed)) = parse_link_reference_definition(src) {
            let key = normalize_label(&label);
            // Only store first definition for a given label.
            self.definitions.entry(key).or_insert((url, title));
            self.pos = line_start + indent + consumed;
            // Skip trailing whitespace/newline if any
            if self.pos < self.source.len() && self.source.as_bytes()[self.pos] == b'\n' {
                self.pos += 1;
            }
            true
        } else {
            false
        }
    }

    // ── Block parsing ────────────────────────────────────────────────

    fn parse_blocks(&mut self) -> Vec<MdNode> {
        let mut blocks: Vec<MdNode> = Vec::new();
        // Accumulator for paragraph lines.
        let mut para_lines: Vec<(usize, usize)> = Vec::new(); // (start, end) byte offsets

        while self.pos < self.source.len() {
            let line_start = self.pos;
            let line = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };

            // Blank line
            if Self::is_blank_line(&line) {
                if !para_lines.is_empty() {
                    // Check if next non-blank line is a setext underline -- if so, don't flush yet.
                    // Actually, setext needs to be checked differently.
                    blocks.push(self.flush_paragraph(&mut para_lines));
                }
                self.advance_line();
                continue;
            }

            // Skip link reference definitions (already collected).
            if self.is_definition_line(&line, line_start) {
                if !para_lines.is_empty() {
                    blocks.push(self.flush_paragraph(&mut para_lines));
                }
                self.skip_definition(line_start);
                continue;
            }

            // Setext heading: must be checked BEFORE thematic break when we have
            // accumulated paragraph lines, because `---` is setext h2 in that context.
            if !para_lines.is_empty()
                && let Some(depth) = Self::setext_underline_depth(&line)
            {
                self.advance_line();
                let node = self.flush_setext_heading(&mut para_lines, depth, line_start, self.pos);
                blocks.push(node);
                continue;
            }

            // Thematic break (must be checked before list because `---` could look like list marker)
            if let Some(node) = self.try_parse_thematic_break(&line, line_start) {
                if !para_lines.is_empty() {
                    blocks.push(self.flush_paragraph(&mut para_lines));
                }
                blocks.push(node);
                continue;
            }

            // ATX heading
            if let Some(node) = self.try_parse_atx_heading(&line, line_start) {
                if !para_lines.is_empty() {
                    blocks.push(self.flush_paragraph(&mut para_lines));
                }
                blocks.push(node);
                continue;
            }

            // Code fence
            if let Some(node) = self.try_parse_code_fence(&line, line_start) {
                if !para_lines.is_empty() {
                    blocks.push(self.flush_paragraph(&mut para_lines));
                }
                blocks.push(node);
                continue;
            }

            // Indented code block (4+ leading spaces, but not inside a paragraph continuation).
            if para_lines.is_empty()
                && let Some(node) = self.try_parse_indented_code(&line, line_start)
            {
                blocks.push(node);
                continue;
            }

            // Blockquote
            if let Some(node) = self.try_parse_blockquote(&line, line_start) {
                if !para_lines.is_empty() {
                    blocks.push(self.flush_paragraph(&mut para_lines));
                }
                blocks.push(node);
                continue;
            }

            // List
            if let Some(node) = self.try_parse_list(&line, line_start) {
                if !para_lines.is_empty() {
                    blocks.push(self.flush_paragraph(&mut para_lines));
                }
                blocks.push(node);
                continue;
            }

            // HTML block
            if para_lines.is_empty()
                && let Some(node) = self.try_parse_html_block(&line, line_start)
            {
                blocks.push(node);
                continue;
            }

            // GFM: Footnote definition `[^id]: content`
            if para_lines.is_empty()
                && let Some(node) = self.try_parse_footnote_definition(&line, line_start)
            {
                blocks.push(node);
                continue;
            }

            // GFM: Table detection.
            // If the current line contains `|` and the next line is a table separator,
            // parse it as a table.
            if para_lines.is_empty()
                && gfm::tables::could_be_table_row(&line)
                && let Some(node) = self.try_parse_table(&line, line_start)
            {
                blocks.push(node);
                continue;
            }
            // Also check if we have exactly one accumulated paragraph line and the
            // current line is a separator (handles case where header was already accumulated).
            if para_lines.len() == 1 && gfm::tables::is_table_separator(&line).is_some() {
                let header_range = para_lines[0];
                let header_line = self.source[header_range.0..header_range.1]
                    .trim_end_matches('\n')
                    .to_string();
                if gfm::tables::could_be_table_row(&header_line) {
                    para_lines.clear();
                    if let Some(node) =
                        self.parse_table_from_header(&header_line, &line, header_range.0)
                    {
                        blocks.push(node);
                        continue;
                    }
                }
            }

            // Otherwise: paragraph continuation line.
            let line_end = self.pos + line.len();
            para_lines.push((line_start, line_end));
            self.advance_line();
        }

        if !para_lines.is_empty() {
            blocks.push(self.flush_paragraph(&mut para_lines));
        }

        blocks
    }

    // ── ATX heading ──────────────────────────────────────────────────

    fn try_parse_atx_heading(&mut self, line: &str, line_start: usize) -> Option<MdNode> {
        let trimmed = line.trim_start();
        let leading_spaces = line.len() - trimmed.len();
        if leading_spaces > 3 {
            return None;
        }

        let bytes = trimmed.as_bytes();
        if bytes.is_empty() || bytes[0] != b'#' {
            return None;
        }

        let mut level = 0usize;
        while level < bytes.len() && bytes[level] == b'#' {
            level += 1;
        }
        if level > 6 {
            return None;
        }

        // After the `#`s there must be a space or end-of-line.
        if level < bytes.len() && bytes[level] != b' ' && bytes[level] != b'\t' {
            return None;
        }

        self.advance_line();
        let line_end = self.pos;

        // Extract content: skip `# ` prefix and optional closing `#`s.
        let content_start_in_trimmed = if level < trimmed.len() {
            level + 1
        } else {
            level
        };
        let mut content = trimmed[content_start_in_trimmed..].to_string();

        // Remove trailing closing #s.
        let stripped = content.trim_end();
        if stripped.ends_with('#') {
            let without_trailing = stripped.trim_end_matches('#');
            if without_trailing.is_empty() || without_trailing.ends_with(' ') {
                content = without_trailing.trim_end().to_string();
            }
        }
        let content = content.trim().to_string();

        let span = Span::new(line_start as u32, line_end as u32);
        let content_offset = line_start + leading_spaces + content_start_in_trimmed;
        let children = if content.is_empty() {
            vec![]
        } else {
            self.parse_inlines(&content, content_offset)
        };

        Some(MdNode::Heading(Heading {
            id: self.id_gen.next_id(),
            span,
            depth: level as u8,
            children,
            slug: None,
        }))
    }

    // ── Setext heading ───────────────────────────────────────────────

    fn setext_underline_depth(line: &str) -> Option<u8> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }
        let ch = trimmed.as_bytes()[0];
        if ch != b'=' && ch != b'-' {
            return None;
        }
        if trimmed.bytes().all(|b| b == ch) {
            if ch == b'=' {
                Some(1)
            } else {
                // Needs at least 1 char. A single `-` could be ambiguous, but CommonMark
                // recognises it as setext heading when preceded by paragraph text.
                Some(2)
            }
        } else {
            None
        }
    }

    fn flush_setext_heading(
        &mut self,
        para_lines: &mut Vec<(usize, usize)>,
        depth: u8,
        _underline_start: usize,
        underline_end: usize,
    ) -> MdNode {
        let start = para_lines[0].0;
        let raw = self.join_para_lines(para_lines);
        para_lines.clear();

        let children = self.parse_inlines(&raw, start);
        MdNode::Heading(Heading {
            id: self.id_gen.next_id(),
            span: Span::new(start as u32, underline_end as u32),
            depth,
            children,
            slug: None,
        })
    }

    // ── Thematic break ───────────────────────────────────────────────

    fn try_parse_thematic_break(&mut self, line: &str, line_start: usize) -> Option<MdNode> {
        let trimmed = line.trim();
        if trimmed.len() < 3 {
            return None;
        }
        let first = trimmed.as_bytes()[0];
        if first != b'-' && first != b'*' && first != b'_' {
            return None;
        }
        let count = trimmed.bytes().filter(|&b| b == first).count();
        let all_match = trimmed.bytes().all(|b| b == first || b == b' ');
        if count >= 3 && all_match {
            self.advance_line();
            Some(MdNode::ThematicBreak(ThematicBreak {
                id: self.id_gen.next_id(),
                span: Span::new(line_start as u32, self.pos as u32),
            }))
        } else {
            None
        }
    }

    // ── Fenced code block ────────────────────────────────────────────

    fn try_parse_code_fence(&mut self, line: &str, line_start: usize) -> Option<MdNode> {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        if indent > 3 {
            return None;
        }

        let fence_char = trimmed.as_bytes().first().copied()?;
        if fence_char != b'`' && fence_char != b'~' {
            return None;
        }

        let fence_len = trimmed.bytes().take_while(|&b| b == fence_char).count();
        if fence_len < 3 {
            return None;
        }

        // Backtick fences cannot have backticks in the info string.
        let info = trimmed[fence_len..].trim();
        if fence_char == b'`' && info.contains('`') {
            return None;
        }

        let (lang, meta) = if info.is_empty() {
            (None, None)
        } else {
            let parts: Vec<&str> = info.splitn(2, char::is_whitespace).collect();
            let lang = Some(parts[0].to_string());
            let meta = if parts.len() > 1 {
                let m = parts[1].trim();
                if m.is_empty() {
                    None
                } else {
                    Some(m.to_string())
                }
            } else {
                None
            };
            (lang, meta)
        };

        self.advance_line();

        let mut code_content = String::new();
        let mut closed = false;
        while self.pos < self.source.len() {
            let cl = self.peek_line_raw().unwrap_or("");
            let cl_trimmed = cl.trim_start();
            let cl_indent = cl.len() - cl_trimmed.len();
            // Closing fence: same char, at least same length, only fence chars + optional trailing spaces.
            if cl_indent <= 3
                && cl_trimmed.bytes().take_while(|&b| b == fence_char).count() >= fence_len
                && cl_trimmed.trim().bytes().all(|b| b == fence_char)
            {
                self.advance_line();
                closed = true;
                break;
            }
            // Strip up to `indent` leading spaces from content lines.
            let stripped = strip_indent(cl, indent);
            code_content.push_str(stripped);
            code_content.push('\n');
            self.advance_line();
        }

        if !closed {
            // Unclosed code fence -- still valid, just goes to end.
        }

        // Remove trailing newline from content.
        if code_content.ends_with('\n') {
            code_content.pop();
        }

        let span = Span::new(line_start as u32, self.pos as u32);
        Some(MdNode::Code(Code {
            id: self.id_gen.next_id(),
            span,
            value: code_content,
            lang,
            meta,
        }))
    }

    // ── Indented code block ──────────────────────────────────────────

    fn try_parse_indented_code(&mut self, line: &str, line_start: usize) -> Option<MdNode> {
        if !line.starts_with("    ") && !line.starts_with('\t') {
            return None;
        }

        let mut code_lines: Vec<String> = Vec::new();
        // Don't consume indented code if line looks like it could be a list continuation.

        while self.pos < self.source.len() {
            let cl = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };
            if Self::is_blank_line(&cl) {
                code_lines.push(String::new());
                self.advance_line();
                continue;
            }
            if let Some(rest) = cl.strip_prefix("    ").or_else(|| cl.strip_prefix('\t')) {
                code_lines.push(rest.to_string());
                self.advance_line();
            } else {
                break;
            }
        }

        // Remove trailing blank lines.
        while code_lines.last().is_some_and(|l| l.is_empty()) {
            code_lines.pop();
        }

        if code_lines.is_empty() {
            return None;
        }

        let value = code_lines.join("\n");
        let span = Span::new(line_start as u32, self.pos as u32);
        Some(MdNode::Code(Code {
            id: self.id_gen.next_id(),
            span,
            value,
            lang: None,
            meta: None,
        }))
    }

    // ── Blockquote ───────────────────────────────────────────────────

    fn try_parse_blockquote(&mut self, line: &str, line_start: usize) -> Option<MdNode> {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('>') {
            return None;
        }

        let mut inner_lines = String::new();
        while self.pos < self.source.len() {
            let cl = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };
            let ct = cl.trim_start();
            if let Some(after) = ct.strip_prefix('>') {
                // Strip optional following space.
                let stripped = after.strip_prefix(' ').unwrap_or(after);
                inner_lines.push_str(stripped);
                inner_lines.push('\n');
                self.advance_line();
            } else if Self::is_blank_line(&cl) {
                break;
            } else {
                // Lazy continuation is allowed for paragraphs, but for simplicity
                // we stop at non-`>` lines.
                break;
            }
        }

        let span = Span::new(line_start as u32, self.pos as u32);
        // Recursively parse the inner content using a sub-parser.
        let children = self.parse_sub_blocks(&inner_lines);

        Some(MdNode::Blockquote(Blockquote {
            id: self.id_gen.next_id(),
            span,
            children,
        }))
    }

    // ── List ─────────────────────────────────────────────────────────

    fn try_parse_list(&mut self, line: &str, line_start: usize) -> Option<MdNode> {
        let (marker_info, _) = Self::list_marker_info(line)?;

        let ordered = marker_info.ordered;
        let start_number = marker_info.number;
        let marker_char = marker_info.marker_char;

        let mut items: Vec<MdNode> = Vec::new();
        let mut had_blank_between = false;

        loop {
            if self.pos >= self.source.len() {
                break;
            }
            let item_start = self.pos;
            let current_line = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };

            let (mi, content_indent) = match Self::list_marker_info(&current_line) {
                Some(x) => x,
                None => break,
            };

            // Must be same list type.
            if mi.ordered != ordered {
                break;
            }
            if !ordered && mi.marker_char != marker_char {
                break;
            }

            // Gather all lines of this item.
            let mut item_lines = String::new();
            // First line: content after marker.
            let first_content = &current_line[content_indent..];
            item_lines.push_str(first_content);
            item_lines.push('\n');
            self.advance_line();

            // Continuation lines: indented at least `content_indent` spaces, or blank.
            let mut item_has_blank = false;
            while self.pos < self.source.len() {
                let cl = match self.peek_line_raw() {
                    Some(l) => l.to_string(),
                    None => break,
                };
                if Self::is_blank_line(&cl) {
                    item_has_blank = true;
                    item_lines.push('\n');
                    self.advance_line();
                    continue;
                }
                // Check indent.
                let cl_indent = cl.len() - cl.trim_start().len();
                if cl_indent >= content_indent {
                    item_lines.push_str(&cl[content_indent..]);
                    item_lines.push('\n');
                    self.advance_line();
                } else {
                    // Could be next list item or end of list.
                    break;
                }
            }

            if item_has_blank {
                had_blank_between = true;
            }

            // GFM: Check for task list marker at the start of item content.
            let checked = if let Some((is_checked, consumed)) =
                gfm::task_list::parse_task_marker(&item_lines)
            {
                // Strip the task marker from the content so it doesn't
                // appear as text in the children.
                item_lines = item_lines[consumed..].to_string();
                Some(is_checked)
            } else {
                None
            };

            // Parse item content using a sub-parser.
            let children = self.parse_sub_blocks(&item_lines);

            let item_span = Span::new(item_start as u32, self.pos as u32);
            items.push(MdNode::ListItem(ListItem {
                id: self.id_gen.next_id(),
                span: item_span,
                spread: item_has_blank,
                checked,
                children,
            }));
        }

        if items.is_empty() {
            return None;
        }

        let span = Span::new(line_start as u32, self.pos as u32);
        Some(MdNode::List(List {
            id: self.id_gen.next_id(),
            span,
            ordered,
            start: if ordered { Some(start_number) } else { None },
            spread: had_blank_between,
            children: items,
        }))
    }

    /// Returns (MarkerInfo, content_indent) if the line starts with a list marker.
    fn list_marker_info(line: &str) -> Option<(ListMarkerInfo, usize)> {
        let leading = line.len() - line.trim_start().len();
        if leading > 3 {
            return None;
        }
        let rest = &line[leading..];
        let bytes = rest.as_bytes();
        if bytes.is_empty() {
            return None;
        }

        // Unordered: `- `, `* `, `+ `
        if (bytes[0] == b'-' || bytes[0] == b'*' || bytes[0] == b'+')
            && bytes.len() > 1
            && bytes[1] == b' '
        {
            // Make sure this is not a thematic break.
            let trimmed = line.trim();
            if trimmed.len() >= 3 {
                let ch = trimmed.as_bytes()[0];
                if (ch == b'-' || ch == b'*')
                    && trimmed.bytes().all(|b| b == ch || b == b' ')
                    && trimmed.bytes().filter(|&b| b == ch).count() >= 3
                {
                    return None;
                }
            }
            return Some((
                ListMarkerInfo {
                    ordered: false,
                    number: 0,
                    marker_char: bytes[0],
                },
                leading + 2,
            ));
        }

        // Ordered: `1. `, `1) `
        let mut i = 0;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i > 0
            && i <= 9
            && i < bytes.len()
            && (bytes[i] == b'.' || bytes[i] == b')')
            && i + 1 < bytes.len()
            && bytes[i + 1] == b' '
        {
            let num: u32 = rest[..i].parse().ok()?;
            return Some((
                ListMarkerInfo {
                    ordered: true,
                    number: num,
                    marker_char: bytes[i],
                },
                leading + i + 2,
            ));
        }

        None
    }

    // ── HTML block ───────────────────────────────────────────────────

    fn try_parse_html_block(&mut self, line: &str, line_start: usize) -> Option<MdNode> {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('<') {
            return None;
        }

        // Must be followed by a letter, `/`, or `!` to look like an HTML tag.
        let second = trimmed.as_bytes().get(1).copied()?;
        if !second.is_ascii_alphabetic() && second != b'/' && second != b'!' {
            return None;
        }

        // Reject autolinks: if the content between `<` and `>` contains `://` or `@`,
        // it's an autolink, not an HTML block. Also reject if no `>` on the line (not
        // a self-contained tag start).
        if let Some(close_angle) = trimmed.find('>') {
            let between = &trimmed[1..close_angle];
            if between.contains("://") || between.contains('@') {
                return None;
            }
        }

        // Additionally, the tag must start with a known block-level HTML tag or
        // be an HTML comment/processing instruction. For simplicity, we check
        // that the text after `<` (and optional `/`) starts with an ASCII letter
        // and is followed by tag-name characters. We already checked that `second`
        // is alphabetic or `/` or `!`.
        // Require that this actually looks like a block-level construct:
        // must be a known block-level HTML element or start with `<!` or `<?`.
        if second.is_ascii_alphabetic() {
            let tag_name = extract_tag_name(trimmed);
            if !is_block_html_tag(&tag_name) {
                return None;
            }
        }

        let mut html_content = String::new();
        while self.pos < self.source.len() {
            let cl = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };
            html_content.push_str(&cl);
            html_content.push('\n');
            self.advance_line();
            if Self::is_blank_line(&cl) {
                break;
            }
        }

        // Trim trailing newlines.
        while html_content.ends_with('\n') {
            html_content.pop();
        }

        let span = Span::new(line_start as u32, self.pos as u32);
        Some(MdNode::Html(Html {
            id: self.id_gen.next_id(),
            span,
            value: html_content,
        }))
    }

    // ── GFM: Table parsing ─────────────────────────────────────────────

    /// Try to parse a table starting at the current line. The current line
    /// is the potential header row; we peek ahead for a separator.
    fn try_parse_table(&mut self, header_line: &str, line_start: usize) -> Option<MdNode> {
        // Save position so we can peek at the next line.
        let saved_pos = self.pos;
        self.advance_line(); // advance past header line

        let sep_line = match self.peek_line_raw() {
            Some(l) => l.to_string(),
            None => {
                self.pos = saved_pos;
                return None;
            }
        };

        let aligns = match gfm::tables::is_table_separator(&sep_line) {
            Some(a) => a,
            None => {
                self.pos = saved_pos;
                return None;
            }
        };

        // We have a valid table. Parse header, separator is consumed, then body rows.
        self.advance_line(); // consume separator line
        self.parse_table_body(header_line, &aligns, line_start)
    }

    /// Parse a table when the header line was already accumulated in para_lines
    /// and the current line (not yet consumed) is the separator.
    fn parse_table_from_header(
        &mut self,
        header_line: &str,
        _sep_line: &str,
        table_start: usize,
    ) -> Option<MdNode> {
        // Current pos is at the separator line. Peek it for alignment.
        let sep_line = match self.peek_line_raw() {
            Some(l) => l.to_string(),
            None => return None,
        };

        let aligns = gfm::tables::is_table_separator(&sep_line)?;
        self.advance_line(); // consume separator line

        self.parse_table_body(header_line, &aligns, table_start)
    }

    /// Shared helper: given a header line and alignments, consume body rows
    /// and produce a Table node.
    fn parse_table_body(
        &mut self,
        header_line: &str,
        aligns: &[AlignKind],
        table_start: usize,
    ) -> Option<MdNode> {
        let col_count = aligns.len();

        // Parse header row.
        let header_cells = gfm::tables::parse_table_row(header_line);
        let mut header_cell_nodes: Vec<MdNode> = Vec::new();
        for (idx, cell_text) in header_cells.iter().enumerate() {
            if idx >= col_count {
                break;
            }
            let children = self.parse_inlines(cell_text, table_start);
            header_cell_nodes.push(MdNode::TableCell(TableCell {
                id: self.id_gen.next_id(),
                span: Span::new(table_start as u32, self.pos as u32),
                children,
            }));
        }
        // Pad with empty cells if fewer cells than columns.
        while header_cell_nodes.len() < col_count {
            header_cell_nodes.push(MdNode::TableCell(TableCell {
                id: self.id_gen.next_id(),
                span: Span::new(table_start as u32, self.pos as u32),
                children: vec![],
            }));
        }

        let header_row = MdNode::TableRow(TableRow {
            id: self.id_gen.next_id(),
            span: Span::new(table_start as u32, self.pos as u32),
            is_header: true,
            children: header_cell_nodes,
        });

        let mut rows = vec![header_row];

        // Parse body rows.
        while self.pos < self.source.len() {
            let body_line = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };

            // Stop at blank lines or lines that don't contain pipes.
            if Self::is_blank_line(&body_line) || !gfm::tables::could_be_table_row(&body_line) {
                break;
            }

            self.advance_line();

            let body_cells = gfm::tables::parse_table_row(&body_line);
            let mut body_cell_nodes: Vec<MdNode> = Vec::new();
            for (idx, cell_text) in body_cells.iter().enumerate() {
                if idx >= col_count {
                    break;
                }
                let children = self.parse_inlines(cell_text, table_start);
                body_cell_nodes.push(MdNode::TableCell(TableCell {
                    id: self.id_gen.next_id(),
                    span: Span::new(table_start as u32, self.pos as u32),
                    children,
                }));
            }
            // Pad with empty cells.
            while body_cell_nodes.len() < col_count {
                body_cell_nodes.push(MdNode::TableCell(TableCell {
                    id: self.id_gen.next_id(),
                    span: Span::new(table_start as u32, self.pos as u32),
                    children: vec![],
                }));
            }

            rows.push(MdNode::TableRow(TableRow {
                id: self.id_gen.next_id(),
                span: Span::new(table_start as u32, self.pos as u32),
                is_header: false,
                children: body_cell_nodes,
            }));
        }

        let span = Span::new(table_start as u32, self.pos as u32);
        Some(MdNode::Table(Table {
            id: self.id_gen.next_id(),
            span,
            align: aligns.to_vec(),
            children: rows,
        }))
    }

    // ── GFM: Footnote definition parsing ─────────────────────────────

    fn try_parse_footnote_definition(&mut self, line: &str, line_start: usize) -> Option<MdNode> {
        let (identifier, first_content) = gfm::footnotes::is_footnote_definition(line)?;
        let identifier = identifier.to_string();
        let mut content = first_content.to_string();
        content.push('\n');

        self.advance_line();

        // Collect continuation lines (indented by 4+ spaces or 1+ tab).
        while self.pos < self.source.len() {
            let cl = match self.peek_line_raw() {
                Some(l) => l.to_string(),
                None => break,
            };
            if Self::is_blank_line(&cl) {
                content.push('\n');
                self.advance_line();
                continue;
            }
            // Continuation: must be indented.
            let indent = cl.len() - cl.trim_start().len();
            if indent >= 2 {
                content.push_str(cl.trim_start());
                content.push('\n');
                self.advance_line();
            } else {
                break;
            }
        }

        let children = self.parse_sub_blocks(&content);
        let span = Span::new(line_start as u32, self.pos as u32);
        Some(MdNode::FootnoteDefinition(FootnoteDefinition {
            id: self.id_gen.next_id(),
            span,
            identifier: identifier.clone(),
            label: Some(identifier),
            children,
        }))
    }

    // ── Link reference definition detection ──────────────────────────

    fn is_definition_line(&self, line: &str, line_start: usize) -> bool {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        if indent > 3 || !trimmed.starts_with('[') {
            return false;
        }
        let src = &self.source[line_start + indent..];
        parse_link_reference_definition(src).is_some()
    }

    fn skip_definition(&mut self, line_start: usize) {
        let line = self.peek_line_raw().unwrap_or("");
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        let src = &self.source[line_start + indent..];
        if let Some((_, _, _, consumed)) = parse_link_reference_definition(src) {
            self.pos = line_start + indent + consumed;
            if self.pos < self.source.len() && self.source.as_bytes()[self.pos] == b'\n' {
                self.pos += 1;
            }
        } else {
            self.advance_line();
        }
    }

    // ── Paragraph flushing ───────────────────────────────────────────

    fn flush_paragraph(&mut self, para_lines: &mut Vec<(usize, usize)>) -> MdNode {
        let start = para_lines[0].0;
        let raw = self.join_para_lines(para_lines);
        let end = para_lines.last().unwrap().1;
        para_lines.clear();

        let children = self.parse_inlines(&raw, start);
        let span_end = if end <= self.source.len() {
            end
        } else {
            self.source.len()
        };
        MdNode::Paragraph(Paragraph {
            id: self.id_gen.next_id(),
            span: Span::new(start as u32, span_end as u32),
            children,
        })
    }

    fn join_para_lines(&self, lines: &[(usize, usize)]) -> String {
        let mut s = String::new();
        for (i, &(start, end)) in lines.iter().enumerate() {
            let e = end.min(self.source.len());
            let slice = &self.source[start..e];
            // Strip trailing newline from each line slice.
            let trimmed = slice.trim_end_matches('\n');
            s.push_str(trimmed);
            if i + 1 < lines.len() {
                s.push('\n');
            }
        }
        s
    }

    // ── Inline parsing ───────────────────────────────────────────────

    fn parse_inlines(&mut self, text: &str, base_offset: usize) -> Vec<MdNode> {
        let mut result = Vec::new();
        let bytes = text.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        let mut text_start = 0;

        while i < len {
            match bytes[i] {
                b'\\' if i + 1 < len => {
                    if is_escapable(bytes[i + 1]) {
                        // Flush preceding text.
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        // Escaped character becomes literal text.
                        let ch = text[i + 1..i + 2].to_string();
                        result.push(self.make_text(&ch, base_offset + i, base_offset + i + 2));
                        i += 2;
                        text_start = i;
                        continue;
                    } else if i + 1 < len && bytes[i + 1] == b'\n' {
                        // Hard line break with backslash.
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        result.push(MdNode::Break(Break {
                            id: self.id_gen.next_id(),
                            span: Span::new((base_offset + i) as u32, (base_offset + i + 2) as u32),
                        }));
                        i += 2;
                        text_start = i;
                        continue;
                    }
                    i += 1;
                }
                b'`' => {
                    // Inline code.
                    let backtick_count = bytes[i..].iter().take_while(|&&b| b == b'`').count();
                    // Find matching closing backticks.
                    let after_open = i + backtick_count;
                    if let Some(close_pos) =
                        find_closing_backticks(bytes, after_open, backtick_count)
                    {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        let mut code_content = text[after_open..close_pos].to_string();
                        // Collapse internal newlines to spaces.
                        code_content = code_content.replace('\n', " ");
                        // Strip one leading and one trailing space if both present and
                        // content is not all spaces.
                        if code_content.len() >= 2
                            && code_content.starts_with(' ')
                            && code_content.ends_with(' ')
                            && code_content.trim().len() < code_content.len() - 2 + code_content.trim().len()
                            // Simpler check: not all spaces
                            && !code_content.chars().all(|c| c == ' ')
                        {
                            code_content = code_content[1..code_content.len() - 1].to_string();
                        }
                        let end_pos = close_pos + backtick_count;
                        result.push(MdNode::InlineCode(InlineCode {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + end_pos) as u32,
                            ),
                            value: code_content,
                        }));
                        i = end_pos;
                        text_start = i;
                        continue;
                    }
                    // No closing backticks: treat as literal.
                    i += backtick_count;
                }
                b'*' | b'_' => {
                    // Emphasis / Strong.
                    let delim_char = bytes[i];
                    let delim_count = bytes[i..].iter().take_while(|&&b| b == delim_char).count();

                    // Try strong first (** or __), then emphasis (* or _).
                    if delim_count >= 2
                        && let Some((inner, consumed)) =
                            self.try_parse_delimited(text, i, delim_char, 2)
                    {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        let children = self.parse_inlines(&inner, base_offset + i + 2);
                        result.push(MdNode::Strong(Strong {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + i + consumed) as u32,
                            ),
                            children,
                        }));
                        i += consumed;
                        text_start = i;
                        continue;
                    }
                    if let Some((inner, consumed)) =
                        self.try_parse_delimited(text, i, delim_char, 1)
                    {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        let children = self.parse_inlines(&inner, base_offset + i + 1);
                        result.push(MdNode::Emphasis(Emphasis {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + i + consumed) as u32,
                            ),
                            children,
                        }));
                        i += consumed;
                        text_start = i;
                        continue;
                    }
                    // Not a valid delimiter run; treat as literal.
                    i += 1;
                }
                b'!' if i + 1 < len && bytes[i + 1] == b'[' => {
                    // Image: ![alt](url "title")
                    if let Some((alt, url, title, consumed)) =
                        self.try_parse_image_or_link(text, i + 1, true)
                    {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        result.push(MdNode::Image(Image {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + i + 1 + consumed) as u32,
                            ),
                            url,
                            title,
                            alt,
                        }));
                        i += 1 + consumed;
                        text_start = i;
                        continue;
                    }
                    i += 1;
                }
                b'[' => {
                    // GFM: Footnote reference [^id]
                    if let Some((fn_id, consumed)) = gfm::footnotes::is_footnote_reference(text, i)
                    {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        let fn_id = fn_id.to_string();
                        result.push(MdNode::FootnoteReference(FootnoteReference {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + i + consumed) as u32,
                            ),
                            identifier: fn_id.clone(),
                            label: Some(fn_id),
                        }));
                        i += consumed;
                        text_start = i;
                        continue;
                    }
                    // Link: [text](url "title") or reference link [text][ref]
                    if let Some((link_text, url, title, consumed)) =
                        self.try_parse_image_or_link(text, i, false)
                    {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        let children = self.parse_inlines(&link_text, base_offset + i + 1);
                        result.push(MdNode::Link(Link {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + i + consumed) as u32,
                            ),
                            url,
                            title,
                            children,
                        }));
                        i += consumed;
                        text_start = i;
                        continue;
                    }
                    i += 1;
                }
                b'<' => {
                    // Autolink or inline HTML.
                    if let Some((url, consumed)) = try_parse_autolink(&text[i..]) {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        let label_text = url.clone();
                        let children = vec![self.make_text(
                            &label_text,
                            base_offset + i + 1,
                            base_offset + i + 1 + label_text.len(),
                        )];
                        result.push(MdNode::Link(Link {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + i + consumed) as u32,
                            ),
                            url,
                            title: None,
                            children,
                        }));
                        i += consumed;
                        text_start = i;
                        continue;
                    }
                    // Inline HTML tag.
                    if let Some(consumed) = try_parse_html_tag(&text[i..]) {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        result.push(MdNode::Html(Html {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + i + consumed) as u32,
                            ),
                            value: text[i..i + consumed].to_string(),
                        }));
                        i += consumed;
                        text_start = i;
                        continue;
                    }
                    i += 1;
                }
                b'&' => {
                    // HTML entities.
                    if let Some((decoded, consumed)) = try_parse_entity(&text[i..]) {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        result.push(self.make_text(
                            &decoded,
                            base_offset + i,
                            base_offset + i + consumed,
                        ));
                        i += consumed;
                        text_start = i;
                        continue;
                    }
                    i += 1;
                }
                b'\n' => {
                    // Check for hard line break: two or more spaces before newline.
                    // We need to look backwards in the accumulated text.
                    let trailing_spaces = text[text_start..i]
                        .bytes()
                        .rev()
                        .take_while(|&b| b == b' ')
                        .count();
                    if trailing_spaces >= 2 {
                        let text_end = i - trailing_spaces;
                        if text_end > text_start {
                            result.push(self.make_text(
                                &text[text_start..text_end],
                                base_offset + text_start,
                                base_offset + text_end,
                            ));
                        }
                        result.push(MdNode::Break(Break {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + text_end) as u32,
                                (base_offset + i + 1) as u32,
                            ),
                        }));
                        i += 1;
                        text_start = i;
                        continue;
                    }
                    // Soft line break: replace newline with space in text.
                    if i > text_start {
                        result.push(self.make_text(
                            &text[text_start..i],
                            base_offset + text_start,
                            base_offset + i,
                        ));
                    }
                    // Represent the soft break as a space in the text node that follows
                    // (or just skip it, producing separate text nodes).
                    i += 1;
                    text_start = i;
                    continue;
                }
                b'~' => {
                    // GFM: Strikethrough ~~text~~
                    if let Some((content_start, end_pos)) =
                        gfm::strikethrough::find_strikethrough(text, i)
                    {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        let inner = &text[content_start..end_pos - 2];
                        let children = self.parse_inlines(inner, base_offset + content_start);
                        result.push(MdNode::Delete(Delete {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + end_pos) as u32,
                            ),
                            children,
                        }));
                        i = end_pos;
                        text_start = i;
                        continue;
                    }
                    i += 1;
                }
                _ => {
                    // GFM: Extended autolink for URLs (http://, https://, www.)
                    // and emails (user@domain.com).
                    if let Some((url, consumed)) = self.try_gfm_autolink(text, i) {
                        if i > text_start {
                            result.push(self.make_text(
                                &text[text_start..i],
                                base_offset + text_start,
                                base_offset + i,
                            ));
                        }
                        let display = &text[i..i + consumed];
                        let children = vec![self.make_text(
                            display,
                            base_offset + i,
                            base_offset + i + consumed,
                        )];
                        result.push(MdNode::Link(Link {
                            id: self.id_gen.next_id(),
                            span: Span::new(
                                (base_offset + i) as u32,
                                (base_offset + i + consumed) as u32,
                            ),
                            url,
                            title: None,
                            children,
                        }));
                        i += consumed;
                        text_start = i;
                        continue;
                    }
                    i += 1;
                }
            }
        }

        // Flush remaining text.
        if text_start < len {
            result.push(self.make_text(
                &text[text_start..len],
                base_offset + text_start,
                base_offset + len,
            ));
        }

        result
    }

    // ── Emphasis/Strong delimiter matching ────────────────────────────

    /// Try to find a matching closing delimiter for `delim_char` repeated `count` times
    /// starting at position `start` in `text`. Returns (inner_content, total_consumed).
    fn try_parse_delimited(
        &self,
        text: &str,
        start: usize,
        delim_char: u8,
        count: usize,
    ) -> Option<(String, usize)> {
        let bytes = text.as_bytes();
        let open_end = start + count;
        if open_end >= bytes.len() {
            return None;
        }

        // The character right after the opening delimiter must not be a space.
        if bytes[open_end] == b' ' || bytes[open_end] == b'\n' {
            return None;
        }

        // Search for matching closing delimiter.
        let mut j = open_end;
        let mut depth = 0;
        while j < bytes.len() {
            if bytes[j] == b'\\' && j + 1 < bytes.len() {
                j += 2;
                continue;
            }
            if bytes[j] == delim_char {
                let run = bytes[j..].iter().take_while(|&&b| b == delim_char).count();
                if run >= count {
                    // Check that character before closing delimiter is not a space.
                    if j > open_end && bytes[j - 1] != b' ' && bytes[j - 1] != b'\n' {
                        if depth == 0 {
                            let inner = &text[open_end..j];
                            let consumed = j + count - start;
                            return Some((inner.to_string(), consumed));
                        }
                        depth -= 1;
                    }
                }
                j += run;
                continue;
            }
            j += 1;
        }
        None
    }

    // ── Link / Image parsing ─────────────────────────────────────────

    /// Parse `[text](url "title")` or `[text][ref]` or `[text][]` or `[text]`.
    /// `start` points to the `[`. For images, the caller already stripped the `!`.
    /// Returns (label_text, url, title, consumed_from_start).
    fn try_parse_image_or_link(
        &self,
        text: &str,
        start: usize,
        is_image: bool,
    ) -> Option<(String, String, Option<String>, usize)> {
        let bytes = text.as_bytes();
        if start >= bytes.len() || bytes[start] != b'[' {
            return None;
        }

        // Find closing `]`.
        let label_start = start + 1;
        let close_bracket = find_unescaped_char(bytes, label_start, b']')?;
        let label = text[label_start..close_bracket].to_string();

        let after_bracket = close_bracket + 1;

        // Inline link: `](url "title")`
        if after_bracket < bytes.len()
            && bytes[after_bracket] == b'('
            && let Some((url, title, paren_consumed)) =
                parse_inline_link_dest(&text[after_bracket..])
        {
            let consumed = after_bracket + paren_consumed - start;
            let label_for_return = if is_image { label } else { label.clone() };
            return Some((label_for_return, url, title, consumed));
        }

        // Full reference link: `[text][ref]`
        if after_bracket < bytes.len() && bytes[after_bracket] == b'[' {
            let ref_start = after_bracket + 1;
            if let Some(ref_close) = find_unescaped_char(bytes, ref_start, b']') {
                let ref_label = &text[ref_start..ref_close];
                let key = if ref_label.is_empty() {
                    normalize_label(&label)
                } else {
                    normalize_label(ref_label)
                };
                if let Some((url, title)) = self.definitions.get(&key) {
                    let consumed = ref_close + 1 - start;
                    return Some((label, url.clone(), title.clone(), consumed));
                }
            }
        }

        // Collapsed reference link: `[text][]`
        // (already handled above when ref_label is empty)

        // Shortcut reference link: `[text]`
        let key = normalize_label(&label);
        if let Some((url, title)) = self.definitions.get(&key) {
            let consumed = after_bracket - start;
            return Some((label, url.clone(), title.clone(), consumed));
        }

        None
    }

    // ── Sub-parser for nested blocks ─────────────────────────────────

    /// Parse blocks from a sub-string using a fresh sub-parser, sharing the
    /// same `id_gen` and `definitions`.
    fn parse_sub_blocks(&mut self, inner: &str) -> Vec<MdNode> {
        let mut sub_diag = DiagnosticSink::new();
        let mut sub_parser = Parser {
            source: inner,
            pos: 0,
            id_gen: self.id_gen,
            diagnostics: &mut sub_diag,
            definitions: self.definitions.clone(),
        };
        let children = sub_parser.parse_blocks();
        // Merge diagnostics.
        for d in sub_diag.into_diagnostics() {
            self.diagnostics.push(d);
        }
        children
    }

    // ── GFM: Extended autolink helper ────────────────────────────────

    /// Try to match a GFM extended autolink (URL or email) at position `pos`.
    /// Returns `(url, bytes_consumed)` if matched.
    fn try_gfm_autolink(&self, text: &str, pos: usize) -> Option<(String, usize)> {
        // Try URL first (http://, https://, www.).
        if let Some(result) = gfm::autolink::try_match_url(text, pos) {
            return Some(result);
        }
        // Try email.
        gfm::autolink::try_match_email(text, pos)
    }

    // ── Helpers ──────────────────────────────────────────────────────

    fn make_text(&mut self, value: &str, start: usize, end: usize) -> MdNode {
        MdNode::Text(Text {
            id: self.id_gen.next_id(),
            span: Span::new(start as u32, end as u32),
            value: value.to_string(),
        })
    }

    /// Peek at the current line without advancing. Returns the line content *without* the
    /// trailing `\n` but the `advance_line` call will skip past the `\n`.
    fn peek_line_raw(&self) -> Option<&str> {
        if self.pos >= self.source.len() {
            return None;
        }
        let rest = &self.source[self.pos..];
        let end = rest.find('\n').unwrap_or(rest.len());
        Some(&rest[..end])
    }

    fn advance_line(&mut self) {
        if self.pos >= self.source.len() {
            return;
        }
        let rest = &self.source[self.pos..];
        let end = rest.find('\n').map_or(rest.len(), |p| p + 1);
        self.pos += end;
    }

    fn is_blank_line(line: &str) -> bool {
        line.trim().is_empty()
    }
}

// ── List marker helper struct ────────────────────────────────────────

struct ListMarkerInfo {
    ordered: bool,
    number: u32,
    marker_char: u8,
}

// ── Free-standing helper functions ───────────────────────────────────

/// Extract the tag name from an HTML-like string starting with `<`.
fn extract_tag_name(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut i = 1; // skip `<`
    // Skip optional `/` for closing tags.
    if i < bytes.len() && bytes[i] == b'/' {
        i += 1;
    }
    let start = i;
    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-') {
        i += 1;
    }
    s[start..i].to_lowercase()
}

/// Check if a tag name is a block-level HTML element per CommonMark spec.
fn is_block_html_tag(tag: &str) -> bool {
    matches!(
        tag,
        "address"
            | "article"
            | "aside"
            | "base"
            | "basefont"
            | "blockquote"
            | "body"
            | "caption"
            | "center"
            | "col"
            | "colgroup"
            | "dd"
            | "details"
            | "dialog"
            | "dir"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "frame"
            | "frameset"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "head"
            | "header"
            | "hr"
            | "html"
            | "iframe"
            | "legend"
            | "li"
            | "link"
            | "main"
            | "menu"
            | "menuitem"
            | "nav"
            | "noframes"
            | "ol"
            | "optgroup"
            | "option"
            | "p"
            | "param"
            | "pre"
            | "script"
            | "section"
            | "select"
            | "source"
            | "style"
            | "summary"
            | "table"
            | "tbody"
            | "td"
            | "template"
            | "textarea"
            | "tfoot"
            | "th"
            | "thead"
            | "title"
            | "tr"
            | "track"
            | "ul"
    )
}

fn strip_indent(line: &str, max_strip: usize) -> &str {
    let bytes = line.as_bytes();
    let mut stripped = 0;
    let mut pos = 0;
    while pos < bytes.len() && stripped < max_strip {
        if bytes[pos] == b' ' || bytes[pos] == b'\t' {
            stripped += 1;
            pos += 1;
        } else {
            break;
        }
    }
    &line[pos..]
}

fn find_closing_backticks(bytes: &[u8], start: usize, count: usize) -> Option<usize> {
    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == b'`' {
            let run = bytes[i..].iter().take_while(|&&b| b == b'`').count();
            if run == count {
                return Some(i);
            }
            i += run;
        } else {
            i += 1;
        }
    }
    None
}

fn find_unescaped_char(bytes: &[u8], start: usize, target: u8) -> Option<usize> {
    let mut i = start;
    let mut depth = 0u32;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            i += 2;
            continue;
        }
        if bytes[i] == b'[' && target == b']' {
            depth += 1;
            i += 1;
            continue;
        }
        if bytes[i] == target {
            if depth == 0 {
                return Some(i);
            }
            depth -= 1;
            i += 1;
            continue;
        }
        i += 1;
    }
    None
}

/// Parse `(url "title")` returning (url, title, bytes_consumed_including_parens).
fn parse_inline_link_dest(s: &str) -> Option<(String, Option<String>, usize)> {
    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes[0] != b'(' {
        return None;
    }
    let mut i = 1;
    // Skip whitespace.
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\n') {
        i += 1;
    }
    if i >= bytes.len() {
        return None;
    }

    // Parse URL.
    let (url, url_end) = if bytes[i] == b'<' {
        // Angle-bracket URL.
        let start = i + 1;
        let close = bytes[start..].iter().position(|&b| b == b'>')?;
        (s[start..start + close].to_string(), start + close + 1)
    } else {
        // Bare URL -- read until whitespace or `)`.
        let start = i;
        let mut paren_depth = 0i32;
        while i < bytes.len() {
            match bytes[i] {
                b' ' | b'\n' | b'\t' => break,
                b')' if paren_depth == 0 => break,
                b'(' => {
                    paren_depth += 1;
                    i += 1;
                }
                b')' => {
                    paren_depth -= 1;
                    i += 1;
                }
                _ => i += 1,
            }
        }
        (s[start..i].to_string(), i)
    };
    i = url_end;

    // Skip whitespace.
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\n') {
        i += 1;
    }

    // Optional title.
    let title = if i < bytes.len() && (bytes[i] == b'"' || bytes[i] == b'\'' || bytes[i] == b'(') {
        let close_char = if bytes[i] == b'(' { b')' } else { bytes[i] };
        let t_start = i + 1;
        let t_end = bytes[t_start..].iter().position(|&b| b == close_char)?;
        let t = s[t_start..t_start + t_end].to_string();
        i = t_start + t_end + 1;
        Some(t)
    } else {
        None
    };

    // Skip whitespace then expect `)`.
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\n') {
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b')' {
        return None;
    }
    i += 1; // consume `)`

    Some((url, title, i))
}

/// Parse a link reference definition: `[label]: url "title"`
/// Returns (label, url, title, consumed_bytes).
fn parse_link_reference_definition(s: &str) -> Option<(String, String, Option<String>, usize)> {
    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes[0] != b'[' {
        return None;
    }
    // Find `]:`.
    let mut i = 1;
    while i < bytes.len() && bytes[i] != b']' {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            i += 2;
        } else {
            i += 1;
        }
    }
    if i >= bytes.len() || bytes[i] != b']' {
        return None;
    }
    let label = s[1..i].to_string();
    i += 1; // skip `]`
    if i >= bytes.len() || bytes[i] != b':' {
        return None;
    }
    i += 1; // skip `:`

    // Skip whitespace (including optional newline).
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'\n' {
        i += 1;
        while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
    }

    // Parse URL.
    if i >= bytes.len() {
        return None;
    }
    let (url, url_end) = if bytes[i] == b'<' {
        let start = i + 1;
        let close = bytes[start..].iter().position(|&b| b == b'>')?;
        (s[start..start + close].to_string(), start + close + 1)
    } else {
        let start = i;
        while i < bytes.len() && bytes[i] != b' ' && bytes[i] != b'\n' && bytes[i] != b'\t' {
            i += 1;
        }
        if i == start {
            return None;
        }
        (s[start..i].to_string(), i)
    };
    i = url_end;

    // Skip whitespace.
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }

    // Optional title on same line.
    let title = if i < bytes.len() && (bytes[i] == b'"' || bytes[i] == b'\'' || bytes[i] == b'(') {
        let close_char = if bytes[i] == b'(' { b')' } else { bytes[i] };
        let t_start = i + 1;
        let t_end_pos = bytes[t_start..].iter().position(|&b| b == close_char);
        if let Some(t_end) = t_end_pos {
            let t = s[t_start..t_start + t_end].to_string();
            i = t_start + t_end + 1;
            Some(t)
        } else {
            None
        }
    } else {
        None
    };

    // Skip trailing whitespace.
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    // Must be at end-of-line or end-of-string.
    if i < bytes.len() && bytes[i] != b'\n' {
        return None;
    }

    Some((label, url, title, i))
}

fn normalize_label(label: &str) -> String {
    label
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Try to parse an autolink: `<scheme:...>` or `<email@domain>`.
fn try_parse_autolink(s: &str) -> Option<(String, usize)> {
    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes[0] != b'<' {
        return None;
    }
    let close = bytes[1..].iter().position(|&b| b == b'>')?;
    let inner = &s[1..1 + close];
    // Contains no spaces or `<`.
    if inner.contains(' ') || inner.contains('<') || inner.contains('\n') {
        return None;
    }
    // Check for URI autolink (contains `://` or well-known scheme).
    if inner.contains("://") || inner.starts_with("mailto:") {
        return Some((inner.to_string(), close + 2));
    }
    // Check for email autolink.
    if inner.contains('@') && !inner.starts_with('@') && !inner.ends_with('@') {
        let url = format!("mailto:{inner}");
        return Some((url, close + 2));
    }
    None
}

/// Try to parse an HTML tag starting at s (which starts with `<`).
/// Returns number of bytes consumed if successful.
fn try_parse_html_tag(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    if bytes.len() < 3 || bytes[0] != b'<' {
        return None;
    }
    // Must be followed by letter, `/`, or `!`.
    let second = bytes[1];
    if !second.is_ascii_alphabetic() && second != b'/' && second != b'!' {
        return None;
    }
    // Find matching `>`.
    let close = bytes[1..].iter().position(|&b| b == b'>')?;
    Some(close + 2)
}

/// Try to parse an HTML entity: `&name;`, `&#digits;`, `&#xhex;`.
fn try_parse_entity(s: &str) -> Option<(String, usize)> {
    let bytes = s.as_bytes();
    if bytes.len() < 3 || bytes[0] != b'&' {
        return None;
    }
    let semi = bytes[1..].iter().position(|&b| b == b';')?;
    if semi == 0 || semi > 31 {
        return None;
    }
    let entity_body = &s[1..1 + semi];

    if let Some(num_str) = entity_body.strip_prefix('#') {
        // Numeric entity.
        let code_point = if let Some(hex) = num_str
            .strip_prefix('x')
            .or_else(|| num_str.strip_prefix('X'))
        {
            u32::from_str_radix(hex, 16).ok()?
        } else {
            num_str.parse::<u32>().ok()?
        };
        let ch = char::from_u32(code_point)?;
        Some((ch.to_string(), semi + 2))
    } else {
        // Named entity -- support a few common ones.
        let decoded = match entity_body {
            "amp" => "&",
            "lt" => "<",
            "gt" => ">",
            "quot" => "\"",
            "apos" => "'",
            "nbsp" => "\u{00A0}",
            "copy" => "\u{00A9}",
            "reg" => "\u{00AE}",
            "mdash" => "\u{2014}",
            "ndash" => "\u{2013}",
            "hellip" => "\u{2026}",
            "laquo" => "\u{00AB}",
            "raquo" => "\u{00BB}",
            "trade" => "\u{2122}",
            _ => return None,
        };
        Some((decoded.to_string(), semi + 2))
    }
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_markdown;

    // ── Block-level tests ────────────────────────────────────────────

    #[test]
    fn test_atx_heading_h1() {
        let r = parse_markdown("# Hello\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::Heading(h) => {
                assert_eq!(h.depth, 1);
                assert_eq!(h.children.len(), 1);
                match &h.children[0] {
                    MdNode::Text(t) => assert_eq!(t.value, "Hello"),
                    other => panic!("expected text, got {other:?}"),
                }
            }
            other => panic!("expected heading, got {other:?}"),
        }
    }

    #[test]
    fn test_atx_heading_h2_through_h6() {
        for depth in 2..=6u8 {
            let hashes = "#".repeat(depth as usize);
            let input = format!("{hashes} Level {depth}\n");
            let r = parse_markdown(&input);
            assert_eq!(r.document.children.len(), 1, "depth={depth}");
            match &r.document.children[0] {
                MdNode::Heading(h) => {
                    assert_eq!(h.depth, depth, "depth={depth}");
                }
                other => panic!("expected heading, got {other:?}"),
            }
        }
    }

    #[test]
    fn test_atx_heading_closing_hashes() {
        let r = parse_markdown("## Hello ##\n");
        match &r.document.children[0] {
            MdNode::Heading(h) => {
                assert_eq!(h.depth, 2);
                match &h.children[0] {
                    MdNode::Text(t) => assert_eq!(t.value, "Hello"),
                    other => panic!("expected text, got {other:?}"),
                }
            }
            other => panic!("expected heading, got {other:?}"),
        }
    }

    #[test]
    fn test_paragraph() {
        let r = parse_markdown("Hello world\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Text(t) => assert_eq!(t.value, "Hello world"),
                other => panic!("expected text, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_code_fence_with_lang() {
        let r = parse_markdown("```rust\nfn main() {}\n```\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::Code(c) => {
                assert_eq!(c.lang.as_deref(), Some("rust"));
                assert_eq!(c.value, "fn main() {}");
            }
            other => panic!("expected code, got {other:?}"),
        }
    }

    #[test]
    fn test_code_fence_tilde() {
        let r = parse_markdown("~~~\ncode\n~~~\n");
        match &r.document.children[0] {
            MdNode::Code(c) => {
                assert!(c.lang.is_none());
                assert_eq!(c.value, "code");
            }
            other => panic!("expected code, got {other:?}"),
        }
    }

    #[test]
    fn test_blockquote() {
        let r = parse_markdown("> Hello\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::Blockquote(bq) => {
                assert_eq!(bq.children.len(), 1);
                assert!(matches!(&bq.children[0], MdNode::Paragraph(_)));
            }
            other => panic!("expected blockquote, got {other:?}"),
        }
    }

    #[test]
    fn test_unordered_list() {
        let r = parse_markdown("- item 1\n- item 2\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::List(l) => {
                assert!(!l.ordered);
                assert_eq!(l.children.len(), 2);
            }
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_ordered_list() {
        let r = parse_markdown("1. first\n2. second\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::List(l) => {
                assert!(l.ordered);
                assert_eq!(l.start, Some(1));
                assert_eq!(l.children.len(), 2);
            }
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_thematic_break_dashes() {
        let r = parse_markdown("---\n");
        assert_eq!(r.document.children.len(), 1);
        assert!(matches!(&r.document.children[0], MdNode::ThematicBreak(_)));
    }

    #[test]
    fn test_thematic_break_asterisks() {
        let r = parse_markdown("***\n");
        assert!(matches!(&r.document.children[0], MdNode::ThematicBreak(_)));
    }

    #[test]
    fn test_thematic_break_underscores() {
        let r = parse_markdown("___\n");
        assert!(matches!(&r.document.children[0], MdNode::ThematicBreak(_)));
    }

    #[test]
    fn test_html_block() {
        let r = parse_markdown("<div>\nhello\n</div>\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::Html(h) => {
                assert!(h.value.contains("div"));
                assert!(h.value.contains("hello"));
            }
            other => panic!("expected html, got {other:?}"),
        }
    }

    #[test]
    fn test_indented_code() {
        let r = parse_markdown("    code line 1\n    code line 2\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::Code(c) => {
                assert!(c.lang.is_none());
                assert!(c.value.contains("code line 1"));
                assert!(c.value.contains("code line 2"));
            }
            other => panic!("expected code, got {other:?}"),
        }
    }

    #[test]
    fn test_setext_heading_h1() {
        let r = parse_markdown("Hello\n=====\n");
        assert_eq!(r.document.children.len(), 1);
        match &r.document.children[0] {
            MdNode::Heading(h) => {
                assert_eq!(h.depth, 1);
                match &h.children[0] {
                    MdNode::Text(t) => assert_eq!(t.value, "Hello"),
                    other => panic!("expected text, got {other:?}"),
                }
            }
            other => panic!("expected heading, got {other:?}"),
        }
    }

    #[test]
    fn test_setext_heading_h2() {
        let r = parse_markdown("Hello\n-----\n");
        match &r.document.children[0] {
            MdNode::Heading(h) => assert_eq!(h.depth, 2),
            other => panic!("expected heading, got {other:?}"),
        }
    }

    // ── Inline tests ─────────────────────────────────────────────────

    #[test]
    fn test_emphasis() {
        let r = parse_markdown("Hello *world*\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                assert_eq!(p.children.len(), 2);
                match &p.children[0] {
                    MdNode::Text(t) => assert_eq!(t.value, "Hello "),
                    other => panic!("expected text, got {other:?}"),
                }
                match &p.children[1] {
                    MdNode::Emphasis(e) => {
                        assert_eq!(e.children.len(), 1);
                        match &e.children[0] {
                            MdNode::Text(t) => assert_eq!(t.value, "world"),
                            other => panic!("expected text, got {other:?}"),
                        }
                    }
                    other => panic!("expected emphasis, got {other:?}"),
                }
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_strong() {
        let r = parse_markdown("Hello **world**\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                assert_eq!(p.children.len(), 2);
                match &p.children[1] {
                    MdNode::Strong(s) => match &s.children[0] {
                        MdNode::Text(t) => assert_eq!(t.value, "world"),
                        other => panic!("expected text, got {other:?}"),
                    },
                    other => panic!("expected strong, got {other:?}"),
                }
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_inline_code() {
        let r = parse_markdown("Use `code` here\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                assert_eq!(p.children.len(), 3);
                match &p.children[0] {
                    MdNode::Text(t) => assert_eq!(t.value, "Use "),
                    other => panic!("expected text, got {other:?}"),
                }
                match &p.children[1] {
                    MdNode::InlineCode(c) => assert_eq!(c.value, "code"),
                    other => panic!("expected inline code, got {other:?}"),
                }
                match &p.children[2] {
                    MdNode::Text(t) => assert_eq!(t.value, " here"),
                    other => panic!("expected text, got {other:?}"),
                }
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_link() {
        let r = parse_markdown("[Click](http://example.com)\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Link(l) => {
                    assert_eq!(l.url, "http://example.com");
                    assert!(l.title.is_none());
                    assert_eq!(l.children.len(), 1);
                    match &l.children[0] {
                        MdNode::Text(t) => assert_eq!(t.value, "Click"),
                        other => panic!("expected text, got {other:?}"),
                    }
                }
                other => panic!("expected link, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_link_with_title() {
        let r = parse_markdown("[Click](http://example.com \"Title\")\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Link(l) => {
                    assert_eq!(l.url, "http://example.com");
                    assert_eq!(l.title.as_deref(), Some("Title"));
                }
                other => panic!("expected link, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_image() {
        let r = parse_markdown("![alt text](image.png)\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Image(img) => {
                    assert_eq!(img.alt, "alt text");
                    assert_eq!(img.url, "image.png");
                    assert!(img.title.is_none());
                }
                other => panic!("expected image, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_escaped_chars() {
        let r = parse_markdown("Hello \\*world\\*\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                // Should produce: Text("Hello "), Text("*"), Text("world"), Text("*")
                let combined: String = p
                    .children
                    .iter()
                    .filter_map(|n| {
                        if let MdNode::Text(t) = n {
                            Some(t.value.as_str())
                        } else {
                            None
                        }
                    })
                    .collect();
                assert_eq!(combined, "Hello *world*");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_hard_line_break_spaces() {
        let r = parse_markdown("Hello  \nworld\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                // Expect: text("Hello"), break, text("world")
                let has_break = p.children.iter().any(|n| matches!(n, MdNode::Break(_)));
                assert!(
                    has_break,
                    "expected a hard line break, children: {:#?}",
                    p.children
                );
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_hard_line_break_backslash() {
        let r = parse_markdown("Hello\\\nworld\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let has_break = p.children.iter().any(|n| matches!(n, MdNode::Break(_)));
                assert!(has_break, "expected a hard line break");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_reference_link() {
        let r = parse_markdown("[hello][hw]\n\n[hw]: http://example.com\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Link(l) => {
                    assert_eq!(l.url, "http://example.com");
                }
                other => panic!("expected link, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_reference_link_collapsed() {
        let r = parse_markdown("[hello][]\n\n[hello]: http://example.com\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Link(l) => assert_eq!(l.url, "http://example.com"),
                other => panic!("expected link, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_reference_link_shortcut() {
        let r = parse_markdown("[hello]\n\n[hello]: http://example.com\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Link(l) => assert_eq!(l.url, "http://example.com"),
                other => panic!("expected link, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_multiple_blocks() {
        let r = parse_markdown("# Title\n\nParagraph text.\n\n---\n");
        assert_eq!(r.document.children.len(), 3);
        assert!(matches!(&r.document.children[0], MdNode::Heading(_)));
        assert!(matches!(&r.document.children[1], MdNode::Paragraph(_)));
        assert!(matches!(&r.document.children[2], MdNode::ThematicBreak(_)));
    }

    #[test]
    fn test_nested_blockquote() {
        let r = parse_markdown("> > nested\n");
        match &r.document.children[0] {
            MdNode::Blockquote(bq) => match &bq.children[0] {
                MdNode::Blockquote(inner) => {
                    assert!(!inner.children.is_empty());
                }
                other => panic!("expected inner blockquote, got {other:?}"),
            },
            other => panic!("expected blockquote, got {other:?}"),
        }
    }

    #[test]
    fn test_span_correctness() {
        let r = parse_markdown("# Hello\n");
        match &r.document.children[0] {
            MdNode::Heading(h) => {
                assert_eq!(h.span.start, 0);
                assert!(h.span.end > 0);
            }
            other => panic!("expected heading, got {other:?}"),
        }
    }

    #[test]
    fn test_empty_input() {
        let r = parse_markdown("");
        assert_eq!(r.document.children.len(), 0);
    }

    #[test]
    fn test_tight_list() {
        let r = parse_markdown("- a\n- b\n- c\n");
        match &r.document.children[0] {
            MdNode::List(l) => {
                assert!(!l.spread, "expected tight list");
                assert_eq!(l.children.len(), 3);
            }
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_loose_list() {
        let r = parse_markdown("- a\n\n- b\n\n- c\n");
        match &r.document.children[0] {
            MdNode::List(l) => {
                assert!(l.spread, "expected loose list");
            }
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_html_entity_numeric() {
        let r = parse_markdown("&#123;\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let combined: String = p
                    .children
                    .iter()
                    .filter_map(|n| {
                        if let MdNode::Text(t) = n {
                            Some(t.value.as_str())
                        } else {
                            None
                        }
                    })
                    .collect();
                assert!(combined.contains('{'), "expected '{{' in {combined:?}");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_html_entity_named() {
        let r = parse_markdown("&amp;\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let combined: String = p
                    .children
                    .iter()
                    .filter_map(|n| {
                        if let MdNode::Text(t) = n {
                            Some(t.value.as_str())
                        } else {
                            None
                        }
                    })
                    .collect();
                assert_eq!(combined, "&");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_autolink_url() {
        let r = parse_markdown("<http://example.com>\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Link(l) => {
                    assert_eq!(l.url, "http://example.com");
                }
                other => panic!("expected link, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_autolink_email() {
        let r = parse_markdown("<user@example.com>\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Link(l) => {
                    assert_eq!(l.url, "mailto:user@example.com");
                }
                other => panic!("expected link, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_emphasis_underscore() {
        let r = parse_markdown("_emphasized_\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Emphasis(e) => match &e.children[0] {
                    MdNode::Text(t) => assert_eq!(t.value, "emphasized"),
                    other => panic!("expected text, got {other:?}"),
                },
                other => panic!("expected emphasis, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_strong_underscore() {
        let r = parse_markdown("__bold__\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Strong(s) => match &s.children[0] {
                    MdNode::Text(t) => assert_eq!(t.value, "bold"),
                    other => panic!("expected text, got {other:?}"),
                },
                other => panic!("expected strong, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_inline_html() {
        let r = parse_markdown("Hello <em>world</em>\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let has_html = p.children.iter().any(|n| matches!(n, MdNode::Html(_)));
                assert!(has_html, "expected inline HTML nodes");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_code_fence_with_meta() {
        let r = parse_markdown("```js highlight\nconsole.log(1);\n```\n");
        match &r.document.children[0] {
            MdNode::Code(c) => {
                assert_eq!(c.lang.as_deref(), Some("js"));
                assert_eq!(c.meta.as_deref(), Some("highlight"));
                assert_eq!(c.value, "console.log(1);");
            }
            other => panic!("expected code, got {other:?}"),
        }
    }

    #[test]
    fn test_double_backtick_inline_code() {
        let r = parse_markdown("Use ``code ` here`` ok\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let has_code = p.children.iter().any(|n| {
                    if let MdNode::InlineCode(c) = n {
                        c.value.contains("code")
                    } else {
                        false
                    }
                });
                assert!(has_code, "expected inline code with backtick inside");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_document_span_covers_entire_input() {
        let input = "# Hello\n\nWorld\n";
        let r = parse_markdown(input);
        assert_eq!(r.document.span.start, 0);
        assert_eq!(r.document.span.end, input.len() as u32);
    }

    #[test]
    fn test_no_diagnostics_for_valid_input() {
        let r = parse_markdown("# Hello\n\nSome paragraph.\n");
        assert!(r.diagnostics.is_empty());
    }

    #[test]
    fn test_blockquote_with_paragraph() {
        let r = parse_markdown("> line one\n> line two\n");
        match &r.document.children[0] {
            MdNode::Blockquote(bq) => {
                assert_eq!(bq.children.len(), 1);
                match &bq.children[0] {
                    MdNode::Paragraph(p) => {
                        let text: String = p
                            .children
                            .iter()
                            .filter_map(|n| {
                                if let MdNode::Text(t) = n {
                                    Some(t.value.as_str())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("");
                        assert!(text.contains("line one"));
                        assert!(text.contains("line two"));
                    }
                    other => panic!("expected paragraph, got {other:?}"),
                }
            }
            other => panic!("expected blockquote, got {other:?}"),
        }
    }

    #[test]
    fn test_ordered_list_start_number() {
        let r = parse_markdown("3. third\n4. fourth\n");
        match &r.document.children[0] {
            MdNode::List(l) => {
                assert!(l.ordered);
                assert_eq!(l.start, Some(3));
            }
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_hex_entity() {
        let r = parse_markdown("&#x7B;\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let combined: String = p
                    .children
                    .iter()
                    .filter_map(|n| {
                        if let MdNode::Text(t) = n {
                            Some(t.value.as_str())
                        } else {
                            None
                        }
                    })
                    .collect();
                assert_eq!(combined, "{");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    // ── GFM integration tests ─────────────────────────────────────────

    #[test]
    fn test_gfm_simple_table() {
        let r = parse_markdown("| a | b |\n|---|---|\n| 1 | 2 |\n");
        match &r.document.children[0] {
            MdNode::Table(t) => {
                assert_eq!(t.align.len(), 2);
                assert_eq!(t.children.len(), 2); // header + 1 body row
                // Check header row.
                match &t.children[0] {
                    MdNode::TableRow(tr) => {
                        assert!(tr.is_header);
                        assert_eq!(tr.children.len(), 2);
                    }
                    other => panic!("expected table row, got {other:?}"),
                }
                // Check body row.
                match &t.children[1] {
                    MdNode::TableRow(tr) => {
                        assert!(!tr.is_header);
                        assert_eq!(tr.children.len(), 2);
                    }
                    other => panic!("expected table row, got {other:?}"),
                }
            }
            other => panic!("expected table, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_table_alignment() {
        let r = parse_markdown("| L | C | R | N |\n|:--|:--:|--:|---|\n| 1 | 2 | 3 | 4 |\n");
        match &r.document.children[0] {
            MdNode::Table(t) => {
                assert_eq!(
                    t.align,
                    vec![
                        AlignKind::Left,
                        AlignKind::Center,
                        AlignKind::Right,
                        AlignKind::None
                    ]
                );
            }
            other => panic!("expected table, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_table_cell_content() {
        let r = parse_markdown("| head |\n|------|\n| body |\n");
        match &r.document.children[0] {
            MdNode::Table(t) => {
                // Check header cell content.
                match &t.children[0] {
                    MdNode::TableRow(tr) => match &tr.children[0] {
                        MdNode::TableCell(tc) => {
                            assert!(!tc.children.is_empty());
                            match &tc.children[0] {
                                MdNode::Text(txt) => assert_eq!(txt.value, "head"),
                                other => panic!("expected text, got {other:?}"),
                            }
                        }
                        other => panic!("expected table cell, got {other:?}"),
                    },
                    other => panic!("expected table row, got {other:?}"),
                }
            }
            other => panic!("expected table, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_table_multiple_body_rows() {
        let r = parse_markdown("| h |\n|---|\n| r1 |\n| r2 |\n| r3 |\n");
        match &r.document.children[0] {
            MdNode::Table(t) => {
                assert_eq!(t.children.len(), 4); // 1 header + 3 body
            }
            other => panic!("expected table, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_table_no_trailing_pipe() {
        let r = parse_markdown("a | b\n---|---\n1 | 2\n");
        match &r.document.children[0] {
            MdNode::Table(t) => {
                assert_eq!(t.align.len(), 2);
                assert_eq!(t.children.len(), 2);
            }
            other => panic!("expected table, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_task_list_unchecked() {
        let r = parse_markdown("- [ ] todo\n");
        match &r.document.children[0] {
            MdNode::List(l) => match &l.children[0] {
                MdNode::ListItem(li) => {
                    assert_eq!(li.checked, Some(false));
                }
                other => panic!("expected list item, got {other:?}"),
            },
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_task_list_checked() {
        let r = parse_markdown("- [x] done\n");
        match &r.document.children[0] {
            MdNode::List(l) => match &l.children[0] {
                MdNode::ListItem(li) => {
                    assert_eq!(li.checked, Some(true));
                }
                other => panic!("expected list item, got {other:?}"),
            },
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_task_list_checked_uppercase() {
        let r = parse_markdown("- [X] DONE\n");
        match &r.document.children[0] {
            MdNode::List(l) => match &l.children[0] {
                MdNode::ListItem(li) => {
                    assert_eq!(li.checked, Some(true));
                }
                other => panic!("expected list item, got {other:?}"),
            },
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_task_list_mixed() {
        let r = parse_markdown("- [x] done\n- [ ] todo\n- regular\n");
        match &r.document.children[0] {
            MdNode::List(l) => {
                assert_eq!(l.children.len(), 3);
                match &l.children[0] {
                    MdNode::ListItem(li) => assert_eq!(li.checked, Some(true)),
                    other => panic!("expected list item, got {other:?}"),
                }
                match &l.children[1] {
                    MdNode::ListItem(li) => assert_eq!(li.checked, Some(false)),
                    other => panic!("expected list item, got {other:?}"),
                }
                match &l.children[2] {
                    MdNode::ListItem(li) => assert_eq!(li.checked, None),
                    other => panic!("expected list item, got {other:?}"),
                }
            }
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_strikethrough() {
        let r = parse_markdown("~~deleted~~\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                assert_eq!(p.children.len(), 1);
                match &p.children[0] {
                    MdNode::Delete(d) => {
                        assert_eq!(d.children.len(), 1);
                        match &d.children[0] {
                            MdNode::Text(t) => assert_eq!(t.value, "deleted"),
                            other => panic!("expected text, got {other:?}"),
                        }
                    }
                    other => panic!("expected delete, got {other:?}"),
                }
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_strikethrough_in_paragraph() {
        let r = parse_markdown("Hello ~~world~~ end\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                assert!(p.children.len() >= 3);
                match &p.children[0] {
                    MdNode::Text(t) => assert_eq!(t.value, "Hello "),
                    other => panic!("expected text, got {other:?}"),
                }
                match &p.children[1] {
                    MdNode::Delete(d) => match &d.children[0] {
                        MdNode::Text(t) => assert_eq!(t.value, "world"),
                        other => panic!("expected text, got {other:?}"),
                    },
                    other => panic!("expected delete, got {other:?}"),
                }
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_extended_autolink_http() {
        let r = parse_markdown("Visit http://example.com today\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                // Should contain: text("Visit "), link, text(" today")
                let has_link = p.children.iter().any(|n| {
                    if let MdNode::Link(l) = n {
                        l.url == "http://example.com"
                    } else {
                        false
                    }
                });
                assert!(has_link, "expected http autolink, got {:#?}", p.children);
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_extended_autolink_https() {
        let r = parse_markdown("See https://example.com/path for more\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let has_link = p.children.iter().any(|n| {
                    if let MdNode::Link(l) = n {
                        l.url == "https://example.com/path"
                    } else {
                        false
                    }
                });
                assert!(has_link, "expected https autolink");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_extended_autolink_www() {
        let r = parse_markdown("See www.example.com for more\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let has_link = p.children.iter().any(|n| {
                    if let MdNode::Link(l) = n {
                        l.url == "http://www.example.com"
                    } else {
                        false
                    }
                });
                assert!(has_link, "expected www autolink, got {:#?}", p.children);
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_extended_autolink_email() {
        let r = parse_markdown("Contact user@example.com for info\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let has_link = p.children.iter().any(|n| {
                    if let MdNode::Link(l) = n {
                        l.url == "mailto:user@example.com"
                    } else {
                        false
                    }
                });
                assert!(has_link, "expected email autolink, got {:#?}", p.children);
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_footnote_reference() {
        let r = parse_markdown("Text[^1] more\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let has_ref = p.children.iter().any(|n| {
                    if let MdNode::FootnoteReference(fr) = n {
                        fr.identifier == "1"
                    } else {
                        false
                    }
                });
                assert!(
                    has_ref,
                    "expected footnote reference, got {:#?}",
                    p.children
                );
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_footnote_definition() {
        let r = parse_markdown("[^1]: This is the footnote.\n");
        match &r.document.children[0] {
            MdNode::FootnoteDefinition(fd) => {
                assert_eq!(fd.identifier, "1");
                assert!(!fd.children.is_empty());
            }
            other => panic!("expected footnote definition, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_footnote_definition_and_reference() {
        let r = parse_markdown("Text[^1]\n\n[^1]: Footnote content\n");
        // Should have paragraph with FootnoteReference, and FootnoteDefinition.
        let has_para_with_ref = r.document.children.iter().any(|n| {
            if let MdNode::Paragraph(p) = n {
                p.children
                    .iter()
                    .any(|c| matches!(c, MdNode::FootnoteReference(_)))
            } else {
                false
            }
        });
        let has_fn_def = r
            .document
            .children
            .iter()
            .any(|n| matches!(n, MdNode::FootnoteDefinition(_)));
        assert!(
            has_para_with_ref,
            "expected paragraph with footnote reference"
        );
        assert!(has_fn_def, "expected footnote definition");
    }

    #[test]
    fn test_gfm_strikethrough_with_emphasis() {
        let r = parse_markdown("~~*bold deleted*~~\n");
        match &r.document.children[0] {
            MdNode::Paragraph(p) => match &p.children[0] {
                MdNode::Delete(d) => {
                    // The inner content should be parsed as emphasis.
                    assert!(!d.children.is_empty());
                    match &d.children[0] {
                        MdNode::Emphasis(_) => {} // ok
                        other => panic!("expected emphasis inside delete, got {other:?}"),
                    }
                }
                other => panic!("expected delete, got {other:?}"),
            },
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_table_with_inline_formatting() {
        let r = parse_markdown("| **bold** | *em* |\n|----------|------|\n| a | b |\n");
        match &r.document.children[0] {
            MdNode::Table(t) => {
                match &t.children[0] {
                    MdNode::TableRow(tr) => {
                        assert!(tr.is_header);
                        // First cell should contain strong.
                        match &tr.children[0] {
                            MdNode::TableCell(tc) => {
                                let has_strong =
                                    tc.children.iter().any(|n| matches!(n, MdNode::Strong(_)));
                                assert!(has_strong, "expected strong in table cell");
                            }
                            other => panic!("expected table cell, got {other:?}"),
                        }
                    }
                    other => panic!("expected table row, got {other:?}"),
                }
            }
            other => panic!("expected table, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_task_list_content_preserved() {
        let r = parse_markdown("- [ ] remaining task text\n");
        match &r.document.children[0] {
            MdNode::List(l) => match &l.children[0] {
                MdNode::ListItem(li) => {
                    assert_eq!(li.checked, Some(false));
                    // The children should contain a paragraph with "remaining task text".
                    let has_text = li.children.iter().any(|n| {
                        if let MdNode::Paragraph(p) = n {
                            p.children.iter().any(|c| {
                                if let MdNode::Text(t) = c {
                                    t.value.contains("remaining task text")
                                } else {
                                    false
                                }
                            })
                        } else {
                            false
                        }
                    });
                    assert!(has_text, "task list item should preserve text content");
                }
                other => panic!("expected list item, got {other:?}"),
            },
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_table_empty_body() {
        let r = parse_markdown("| h1 | h2 |\n|----|----|\n");
        match &r.document.children[0] {
            MdNode::Table(t) => {
                assert_eq!(t.children.len(), 1); // header only, no body rows
                match &t.children[0] {
                    MdNode::TableRow(tr) => assert!(tr.is_header),
                    other => panic!("expected table row, got {other:?}"),
                }
            }
            other => panic!("expected table, got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_not_a_table_without_separator() {
        // A line with pipes but no separator line should be a paragraph.
        let r = parse_markdown("| not a | table |\n");
        match &r.document.children[0] {
            MdNode::Paragraph(_) => {} // correct: no separator, so it's a paragraph
            other => panic!("expected paragraph (no table separator), got {other:?}"),
        }
    }

    #[test]
    fn test_gfm_footnote_with_alphanumeric_id() {
        let r = parse_markdown("[^note-1]: Some footnote\n");
        match &r.document.children[0] {
            MdNode::FootnoteDefinition(fd) => {
                assert_eq!(fd.identifier, "note-1");
            }
            other => panic!("expected footnote definition, got {other:?}"),
        }
    }

    // ── Frontmatter integration tests ─────────────────────────────────

    #[test]
    fn test_yaml_frontmatter_basic() {
        let input = "---\ntitle: Hello\nauthor: World\n---\n\n# Content\n";
        let r = parse_markdown(input);
        assert_eq!(r.frontmatter.get("title").unwrap(), "Hello");
        assert_eq!(r.frontmatter.get("author").unwrap(), "World");
        // Document should start with the heading, not frontmatter.
        assert!(matches!(&r.document.children[0], MdNode::Heading(_)));
    }

    #[test]
    fn test_yaml_frontmatter_with_types() {
        let input = "---\ntitle: Test\ncount: 42\ntags:\n  - a\n  - b\n---\n\nContent\n";
        let r = parse_markdown(input);
        assert_eq!(r.frontmatter["count"], 42);
        assert!(r.frontmatter["tags"].is_array());
        let tags = r.frontmatter["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 2);
    }

    #[test]
    fn test_toml_frontmatter() {
        let input = "+++\ntitle = \"Hello\"\ncount = 42\n+++\n\n# Content\n";
        let r = parse_markdown(input);
        assert_eq!(r.frontmatter.get("title").unwrap(), "Hello");
        assert_eq!(r.frontmatter["count"], 42);
        assert!(matches!(&r.document.children[0], MdNode::Heading(_)));
    }

    #[test]
    fn test_json_frontmatter() {
        let input = ";;;\n{\"title\": \"Hello\", \"count\": 42}\n;;;\n\n# Content\n";
        let r = parse_markdown(input);
        assert_eq!(r.frontmatter.get("title").unwrap(), "Hello");
        assert_eq!(r.frontmatter["count"], 42);
        assert!(matches!(&r.document.children[0], MdNode::Heading(_)));
    }

    #[test]
    fn test_no_frontmatter() {
        let r = parse_markdown("# Just content\n");
        assert!(r.frontmatter.is_empty());
        assert!(matches!(&r.document.children[0], MdNode::Heading(_)));
    }

    #[test]
    fn test_yaml_frontmatter_empty() {
        let input = "---\n---\n\n# Content\n";
        let r = parse_markdown(input);
        assert!(r.frontmatter.is_empty());
        assert!(matches!(&r.document.children[0], MdNode::Heading(_)));
    }

    #[test]
    fn test_frontmatter_does_not_appear_in_ast() {
        let input = "---\ntitle: Test\n---\n\nParagraph\n";
        let r = parse_markdown(input);
        for child in &r.document.children {
            assert!(
                !matches!(child, MdNode::Yaml(_) | MdNode::Toml(_) | MdNode::Json(_)),
                "frontmatter nodes should not appear in the AST"
            );
        }
    }

    #[test]
    fn test_frontmatter_span_correctness() {
        let input = "---\ntitle: Test\n---\n\n# Heading\n";
        let r = parse_markdown(input);
        // The first real content node should have its span pointing into
        // the correct bytes of the original input.
        match &r.document.children[0] {
            MdNode::Heading(h) => {
                let heading_text = &input[h.span.start as usize..h.span.end as usize];
                assert!(
                    heading_text.contains("Heading"),
                    "span should reference '# Heading' in the original input, got: {heading_text:?}"
                );
            }
            other => panic!("expected heading, got {other:?}"),
        }
    }

    #[test]
    fn test_yaml_frontmatter_with_nested_object() {
        let input = "---\nmeta:\n  key: value\n  count: 5\n---\n\nText\n";
        let r = parse_markdown(input);
        let meta = r.frontmatter.get("meta").unwrap();
        assert!(meta.is_object());
        assert_eq!(meta["key"], "value");
        assert_eq!(meta["count"], 5);
    }

    #[test]
    fn test_toml_frontmatter_does_not_appear_in_ast() {
        let input = "+++\ntitle = \"Test\"\n+++\n\nParagraph\n";
        let r = parse_markdown(input);
        for child in &r.document.children {
            assert!(
                !matches!(child, MdNode::Yaml(_) | MdNode::Toml(_) | MdNode::Json(_)),
                "frontmatter nodes should not appear in the AST"
            );
        }
    }

    #[test]
    fn test_json_frontmatter_does_not_appear_in_ast() {
        let input = ";;;\n{\"title\": \"Test\"}\n;;;\n\nParagraph\n";
        let r = parse_markdown(input);
        for child in &r.document.children {
            assert!(
                !matches!(child, MdNode::Yaml(_) | MdNode::Toml(_) | MdNode::Json(_)),
                "frontmatter nodes should not appear in the AST"
            );
        }
    }

    #[test]
    fn test_frontmatter_with_multiple_content_blocks() {
        let input = "---\ntitle: Hello\n---\n\n# Heading\n\nParagraph text.\n\n- item\n";
        let r = parse_markdown(input);
        assert_eq!(r.frontmatter.get("title").unwrap(), "Hello");
        assert!(
            r.document.children.len() >= 3,
            "should have heading, paragraph, and list"
        );
        assert!(matches!(&r.document.children[0], MdNode::Heading(_)));
        assert!(matches!(&r.document.children[1], MdNode::Paragraph(_)));
        assert!(matches!(&r.document.children[2], MdNode::List(_)));
    }

    #[test]
    fn test_frontmatter_yaml_boolean_values() {
        let input = "---\ndraft: true\npublished: false\n---\n\nContent\n";
        let r = parse_markdown(input);
        assert_eq!(r.frontmatter["draft"], true);
        assert_eq!(r.frontmatter["published"], false);
    }

    #[test]
    fn test_toml_frontmatter_span_correctness() {
        let input = "+++\ntitle = \"Test\"\n+++\n\n# Heading\n";
        let r = parse_markdown(input);
        match &r.document.children[0] {
            MdNode::Heading(h) => {
                let heading_text = &input[h.span.start as usize..h.span.end as usize];
                assert!(
                    heading_text.contains("Heading"),
                    "span should reference '# Heading' in the original input, got: {heading_text:?}"
                );
            }
            other => panic!("expected heading, got {other:?}"),
        }
    }

    #[test]
    fn test_json_frontmatter_span_correctness() {
        let input = ";;;\n{\"title\": \"Test\"}\n;;;\n\n# Heading\n";
        let r = parse_markdown(input);
        match &r.document.children[0] {
            MdNode::Heading(h) => {
                let heading_text = &input[h.span.start as usize..h.span.end as usize];
                assert!(
                    heading_text.contains("Heading"),
                    "span should reference '# Heading' in the original input, got: {heading_text:?}"
                );
            }
            other => panic!("expected heading, got {other:?}"),
        }
    }

    #[test]
    fn test_frontmatter_only_at_beginning() {
        // `---` in the middle of a document should not be treated as frontmatter
        let input = "# Title\n\n---\ntitle: Test\n---\n";
        let r = parse_markdown(input);
        assert!(
            r.frontmatter.is_empty(),
            "--- in the middle should not be frontmatter"
        );
    }

    #[test]
    fn test_frontmatter_with_link_reference_definitions() {
        let input = "---\ntitle: Hello\n---\n\n[click][hw]\n\n[hw]: http://example.com\n";
        let r = parse_markdown(input);
        assert_eq!(r.frontmatter.get("title").unwrap(), "Hello");
        // The link reference should still work correctly
        match &r.document.children[0] {
            MdNode::Paragraph(p) => {
                let has_link = p.children.iter().any(|c| matches!(c, MdNode::Link(_)));
                assert!(
                    has_link,
                    "reference link should be resolved after frontmatter"
                );
            }
            other => panic!("expected paragraph with link, got {other:?}"),
        }
    }
}
