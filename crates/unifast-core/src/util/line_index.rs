use crate::ast::common::Position;

pub struct LineIndex {
    newlines: Vec<u32>, // byte offsets of each '\n'
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let newlines = source
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'\n')
            .map(|(i, _)| i as u32)
            .collect();
        Self { newlines }
    }

    pub fn line_col(&self, offset: u32) -> Position {
        let line = match self.newlines.binary_search(&offset) {
            Ok(idx) => idx + 1,  // offset is exactly on a newline
            Err(idx) => idx + 1, // offset is between newlines
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
        // 'h' is at offset 0, line 1, col 1
        assert_eq!(idx.line_col(0), Position { line: 1, column: 1 });
        // 'w' is at offset 6, line 2, col 1
        assert_eq!(idx.line_col(6), Position { line: 2, column: 1 });
        // 'f' is at offset 12, line 3, col 1
        assert_eq!(idx.line_col(12), Position { line: 3, column: 1 });
        // 'o' (second char of "foo") is at offset 13, line 3, col 2
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
        // '\n' at offset 2 is end of line 1
        assert_eq!(idx.line_col(2), Position { line: 1, column: 3 });
        // '\n' at offset 5 is end of line 2
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
        // 'l' of "line3" is at offset 12
        assert_eq!(idx.line_col(12), Position { line: 3, column: 1 });
        // '3' at end of "line3" is at offset 16
        assert_eq!(idx.line_col(16), Position { line: 3, column: 5 });
    }
}
