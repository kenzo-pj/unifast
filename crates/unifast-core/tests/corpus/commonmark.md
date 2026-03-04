# CommonMark Golden Corpus

## Headings

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

Setext Heading 1
================

Setext Heading 2
----------------

## Paragraphs

This is a paragraph with **bold**, *italic*, and `inline code`.

This is another paragraph with a [link](https://example.com "title")
and an ![image](img.png "alt").

## Emphasis

*single asterisks*
_single underscores_
**double asterisks**
__double underscores__
***triple asterisks***
___triple underscores___

## Code

Inline `code` here.

```
plain code block
```

```rust
fn main() {
    println!("Hello");
}
```

    indented code block
    line 2

## Blockquotes

> Simple blockquote.

> Multi-paragraph blockquote.
>
> Second paragraph.

> > Nested blockquote.

## Lists

- Unordered item 1
- Unordered item 2
- Unordered item 3

1. Ordered item 1
2. Ordered item 2
3. Ordered item 3

- Item with
  continuation line

- Item 1

  With paragraph continuation.

- Item 2

## Thematic Breaks

---

***

___

## Links

[Basic link](https://example.com)
[Link with title](https://example.com "Example")
<https://autolink.example.com>

[Reference link][ref1]

[ref1]: https://example.com "Reference"

## Images

![Alt text](image.png)
![Alt text](image.png "Title")

## HTML Entities

&amp; &lt; &gt; &copy;

## Escapes

\*not emphasis\*
\# not a heading
\[not a link\]

## Hard Line Breaks

Line with two spaces
continues here.

Line with backslash\
continues here.
