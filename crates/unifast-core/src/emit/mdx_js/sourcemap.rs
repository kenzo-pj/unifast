use super::printer::SourceMapping;
use crate::util::line_index::LineIndex;

#[must_use]
pub fn generate_sourcemap(file: &str, source_content: &str, mappings: &[SourceMapping]) -> String {
    let source_file = file.replace(".js", ".mdx");
    let line_index = LineIndex::new(source_content);

    let mut builder = sourcemap::SourceMapBuilder::new(Some(file));
    let src_id = builder.add_source(&source_file);
    builder.set_source_contents(src_id, Some(source_content));

    for mapping in mappings {
        let original_pos = line_index.line_col(mapping.original_offset);
        builder.add_raw(
            mapping.generated_line - 1,
            mapping.generated_column,
            original_pos.line - 1,
            original_pos.column - 1,
            Some(src_id),
            None,
            false,
        );
    }

    let sm = builder.into_sourcemap();
    let mut buf = Vec::new();
    sm.to_writer(&mut buf)
        .expect("sourcemap serialization failed");
    String::from_utf8(buf).expect("sourcemap is valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_mappings() {
        let map = generate_sourcemap("output.js", "# Hello\n", &[]);
        assert!(map.contains("\"version\":3"));
        assert!(map.contains("output.js"));
        assert!(map.contains("output.mdx"));
    }

    #[test]
    fn single_mapping() {
        let mappings = vec![SourceMapping {
            generated_line: 1,
            generated_column: 0,
            original_offset: 0,
        }];
        let map = generate_sourcemap("output.js", "# Hello\n", &mappings);
        assert!(map.contains("\"version\":3"));
        assert!(map.contains("\"mappings\""));
    }

    #[test]
    fn multiple_lines() {
        let mappings = vec![
            SourceMapping {
                generated_line: 1,
                generated_column: 0,
                original_offset: 0,
            },
            SourceMapping {
                generated_line: 3,
                generated_column: 4,
                original_offset: 10,
            },
        ];
        let map = generate_sourcemap("output.js", "hello\nworld\nfoo\n", &mappings);
        assert!(map.contains("\"mappings\""));
    }

    #[test]
    fn generate_sourcemap_with_content() {
        let map = generate_sourcemap("out.js", "hello\nworld\n", &[]);
        assert!(map.contains("hello"));
    }
}
