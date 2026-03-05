use crate::ast::common::Position;

pub struct LineIndex {
    newlines: Vec<u32>,
}

impl LineIndex {
    #[must_use]
    pub fn new(source: &str) -> Self {
        let newlines = source
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'\n')
            .map(|(i, _)| i as u32)
            .collect();
        Self { newlines }
    }

    #[must_use]
    pub fn line_col(&self, offset: u32) -> Position {
        let line = match self.newlines.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx + 1,
        };
        let line_start = if line <= 1 {
            0
        } else {
            self.newlines[line - 2] + 1
        };
        Position {
            line: line as u32,
            column: offset - line_start + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_multiline() {
        let source = "hello\nworld\nfoo";
        let idx = LineIndex::new(source);
        assert_eq!(idx.line_col(0), Position { line: 1, column: 1 });
        assert_eq!(idx.line_col(6), Position { line: 2, column: 1 });
        assert_eq!(idx.line_col(12), Position { line: 3, column: 1 });
        assert_eq!(idx.line_col(13), Position { line: 3, column: 2 });
    }

    #[test]
    fn empty_string() {
        let source = "";
        let idx = LineIndex::new(source);
        assert_eq!(idx.line_col(0), Position { line: 1, column: 1 });
    }

    #[test]
    fn offset_at_newline() {
        let source = "ab\ncd\nef";
        let idx = LineIndex::new(source);
        assert_eq!(idx.line_col(2), Position { line: 1, column: 3 });
        assert_eq!(idx.line_col(5), Position { line: 2, column: 3 });
    }

    #[test]
    fn offset_at_start() {
        let source = "abc";
        let idx = LineIndex::new(source);
        assert_eq!(idx.line_col(0), Position { line: 1, column: 1 });
    }

    #[test]
    fn last_line() {
        let source = "line1\nline2\nline3";
        let idx = LineIndex::new(source);
        assert_eq!(idx.line_col(12), Position { line: 3, column: 1 });
        assert_eq!(idx.line_col(16), Position { line: 3, column: 5 });
    }
}
