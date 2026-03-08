use crate::ast::common::{NodeIdGen, Span};
use crate::ast::hast::nodes::*;
use crate::util::small_map::SmallMap;

const fn is_emoji(c: char) -> bool {
    matches!(c,
        '\u{200D}'                |  // Zero Width Joiner
        '\u{231A}'..='\u{231B}'   |  // Watch, Hourglass
        '\u{23E9}'..='\u{23F3}'   |  // Various
        '\u{23F8}'..='\u{23FA}'   |  // Various
        '\u{25AA}'..='\u{25AB}'   |  // Squares
        '\u{25B6}' | '\u{25C0}'  |  // Play buttons
        '\u{25FB}'..='\u{25FE}'   |  // Squares
        '\u{2600}'..='\u{26FF}'   |  // Misc symbols
        '\u{2700}'..='\u{27BF}'   |  // Dingbats
        '\u{2934}'..='\u{2935}'   |
        '\u{2B05}'..='\u{2B07}'   |
        '\u{2B1B}'..='\u{2B1C}'   |
        '\u{2B50}' | '\u{2B55}'  |
        '\u{3030}' | '\u{303D}'  |
        '\u{3297}' | '\u{3299}'  |
        '\u{FE00}'..='\u{FE0F}'   |  // Variation Selectors
        '\u{1F1E0}'..='\u{1F1FF}' |  // Flags
        '\u{1F300}'..='\u{1F5FF}' |  // Misc Symbols and Pictographs
        '\u{1F600}'..='\u{1F64F}' |  // Emoticons
        '\u{1F680}'..='\u{1F6FF}' |  // Transport and Map
        '\u{1F900}'..='\u{1F9FF}' |  // Supplemental Symbols
        '\u{1FA00}'..='\u{1FA6F}' |  // Chess Symbols
        '\u{1FA70}'..='\u{1FAFF}'    // Symbols Extended-A
    )
}

fn emoji_label(emoji: &str) -> &'static str {
    match emoji {
        "\u{1F600}" => "grinning face",
        "\u{1F603}" => "grinning face with big eyes",
        "\u{1F604}" => "grinning face with smiling eyes",
        "\u{1F60A}" => "smiling face with smiling eyes",
        "\u{1F60D}" => "smiling face with heart-eyes",
        "\u{1F602}" => "face with tears of joy",
        "\u{1F923}" => "rolling on the floor laughing",
        "\u{1F622}" => "crying face",
        "\u{1F62D}" => "loudly crying face",
        "\u{1F631}" => "face screaming in fear",
        "\u{1F621}" => "enraged face",
        "\u{1F914}" => "thinking face",
        "\u{1F44D}" => "thumbs up",
        "\u{1F44E}" => "thumbs down",
        "\u{1F44F}" => "clapping hands",
        "\u{1F64C}" => "raising hands",
        "\u{1F64F}" => "folded hands",
        "\u{1F4AA}" => "flexed biceps",
        "\u{2764}\u{FE0F}" => "red heart",
        "\u{2764}" => "red heart",
        "\u{1F494}" => "broken heart",
        "\u{1F4AF}" => "hundred points",
        "\u{1F525}" => "fire",
        "\u{2B50}" => "star",
        "\u{1F31F}" => "glowing star",
        "\u{2728}" => "sparkles",
        "\u{1F389}" => "party popper",
        "\u{1F38A}" => "confetti ball",
        "\u{1F388}" => "balloon",
        "\u{1F381}" => "wrapped gift",
        "\u{2705}" => "check mark button",
        "\u{274C}" => "cross mark",
        "\u{26A0}\u{FE0F}" => "warning",
        "\u{26A0}" => "warning",
        "\u{1F680}" => "rocket",
        "\u{1F4A1}" => "light bulb",
        "\u{1F4DD}" => "memo",
        "\u{1F4CC}" => "pushpin",
        "\u{1F517}" => "link",
        "\u{1F4E7}" => "e-mail",
        "\u{1F4DE}" => "telephone receiver",
        "\u{1F512}" => "locked",
        "\u{1F511}" => "key",
        "\u{1F60E}" => "smiling face with sunglasses",
        "\u{1F609}" => "winking face",
        "\u{1F610}" => "neutral face",
        "\u{1F612}" => "unamused face",
        "\u{1F614}" => "pensive face",
        "\u{1F616}" => "confounded face",
        "\u{1F618}" => "face blowing a kiss",
        "\u{1F61C}" => "winking face with tongue",
        "\u{1F61D}" => "squinting face with tongue",
        "\u{1F620}" => "angry face",
        "\u{1F624}" => "face with steam from nose",
        "\u{1F625}" => "sad but relieved face",
        "\u{1F628}" => "fearful face",
        "\u{1F629}" => "weary face",
        "\u{1F62A}" => "sleepy face",
        "\u{1F62B}" => "tired face",
        "\u{1F630}" => "anxious face with sweat",
        "\u{1F632}" => "astonished face",
        "\u{1F633}" => "flushed face",
        "\u{1F634}" => "sleeping face",
        "\u{1F635}" => "face with crossed-out eyes",
        "\u{1F637}" => "face with medical mask",
        "\u{1F638}" => "grinning cat with smiling eyes",
        "\u{1F639}" => "cat with tears of joy",
        "\u{1F63A}" => "grinning cat",
        "\u{1F63B}" => "smiling cat with heart-eyes",
        "\u{1F63C}" => "cat with wry smile",
        "\u{1F63D}" => "kissing cat",
        "\u{1F63E}" => "pouting cat",
        "\u{1F63F}" => "crying cat",
        "\u{1F640}" => "weary cat",
        "\u{1F645}" => "person gesturing NO",
        "\u{1F646}" => "person gesturing OK",
        "\u{1F647}" => "person bowing",
        "\u{1F648}" => "see-no-evil monkey",
        "\u{1F649}" => "hear-no-evil monkey",
        "\u{1F64A}" => "speak-no-evil monkey",
        "\u{1F64B}" => "person raising hand",
        "\u{1F4A9}" => "pile of poo",
        "\u{1F47B}" => "ghost",
        "\u{1F47D}" => "alien",
        "\u{1F47E}" => "alien monster",
        "\u{1F916}" => "robot",
        "\u{1F480}" => "skull",
        "\u{1F4A5}" => "collision",
        "\u{1F4A8}" => "dashing away",
        "\u{1F440}" => "eyes",
        "\u{1F442}" => "ear",
        "\u{1F443}" => "nose",
        "\u{1F444}" => "mouth",
        "\u{1F445}" => "tongue",
        "\u{1F446}" => "backhand index pointing up",
        "\u{1F447}" => "backhand index pointing down",
        "\u{1F448}" => "backhand index pointing left",
        "\u{1F449}" => "backhand index pointing right",
        "\u{1F44A}" => "oncoming fist",
        "\u{1F44B}" => "waving hand",
        "\u{1F44C}" => "OK hand",
        "\u{270C}\u{FE0F}" => "victory hand",
        "\u{270C}" => "victory hand",
        _ => "emoji",
    }
}

pub fn apply_accessible_emoji(root: &mut HRoot, id_gen: &mut NodeIdGen) {
    let children = std::mem::take(&mut root.children);
    root.children = process_children(children, id_gen);
}

fn process_children(children: Vec<HNode>, id_gen: &mut NodeIdGen) -> Vec<HNode> {
    let mut result = Vec::new();
    for child in children {
        match child {
            HNode::Text(text) => {
                split_text_with_emoji(&text.value, text.span, &mut result, id_gen);
            }
            HNode::Element(mut elem) => {
                let inner = std::mem::take(&mut elem.children);
                elem.children = process_children(inner, id_gen);
                result.push(HNode::Element(elem));
            }
            HNode::Root(mut r) => {
                let inner = std::mem::take(&mut r.children);
                r.children = process_children(inner, id_gen);
                result.push(HNode::Root(r));
            }
            other => result.push(other),
        }
    }
    result
}

fn split_text_with_emoji(text: &str, span: Span, out: &mut Vec<HNode>, id_gen: &mut NodeIdGen) {
    let mut last_end = 0;
    let mut chars = text.char_indices().peekable();

    while let Some(&(i, c)) = chars.peek() {
        if is_emoji(c) {
            if i > last_end {
                out.push(HNode::Text(HText {
                    id: id_gen.next_id(),
                    span,
                    value: text[last_end..i].to_string(),
                }));
            }

            let emoji_start = i;
            let mut emoji_end = i + c.len_utf8();
            chars.next();

            while let Some(&(j, next_c)) = chars.peek() {
                if is_emoji(next_c) {
                    emoji_end = j + next_c.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }

            let emoji_str = &text[emoji_start..emoji_end];
            let label = emoji_label(emoji_str);

            let mut attrs = SmallMap::new();
            attrs.insert("role".to_string(), "img".to_string());
            attrs.insert("aria-label".to_string(), label.to_string());

            out.push(HNode::Element(HElement {
                id: id_gen.next_id(),
                span,
                tag: "span".to_string(),
                attributes: attrs,
                children: vec![HNode::Text(HText {
                    id: id_gen.next_id(),
                    span,
                    value: emoji_str.to_string(),
                })],
                self_closing: false,
            }));

            last_end = emoji_end;
        } else {
            chars.next();
        }
    }

    if last_end == 0 {
        out.push(HNode::Text(HText {
            id: id_gen.next_id(),
            span,
            value: text.to_string(),
        }));
    } else if last_end < text.len() {
        out.push(HNode::Text(HText {
            id: id_gen.next_id(),
            span,
            value: text[last_end..].to_string(),
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeIdGen;

    fn make_text(id_gen: &mut NodeIdGen, value: &str) -> HNode {
        HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::empty(),
            value: value.to_string(),
        })
    }

    fn make_root(id_gen: &mut NodeIdGen, children: Vec<HNode>) -> HRoot {
        HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children,
        }
    }

    #[test]
    fn wraps_emoji_in_span() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello \u{1F680} world");
        let mut root = make_root(&mut id_gen, vec![text]);

        apply_accessible_emoji(&mut root, &mut id_gen);

        assert_eq!(root.children.len(), 3);

        assert!(matches!(&root.children[0], HNode::Text(t) if t.value == "Hello "));

        if let HNode::Element(elem) = &root.children[1] {
            assert_eq!(elem.tag, "span");
            assert_eq!(
                elem.attributes.get(&"role".to_string()),
                Some(&"img".to_string())
            );
            assert_eq!(
                elem.attributes.get(&"aria-label".to_string()),
                Some(&"rocket".to_string())
            );
            if let HNode::Text(t) = &elem.children[0] {
                assert_eq!(t.value, "\u{1F680}");
            } else {
                panic!("expected text child in span");
            }
        } else {
            panic!("expected span element");
        }

        assert!(matches!(&root.children[2], HNode::Text(t) if t.value == " world"));
    }

    #[test]
    fn no_emoji_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "plain text");
        let mut root = make_root(&mut id_gen, vec![text]);

        apply_accessible_emoji(&mut root, &mut id_gen);

        assert_eq!(root.children.len(), 1);
        assert!(matches!(&root.children[0], HNode::Text(t) if t.value == "plain text"));
    }

    #[test]
    fn multiple_emoji() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "\u{1F525}\u{1F680}");
        let mut root = make_root(&mut id_gen, vec![text]);

        apply_accessible_emoji(&mut root, &mut id_gen);

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.tag, "span");
            assert_eq!(
                elem.attributes.get(&"role".to_string()),
                Some(&"img".to_string())
            );
        } else {
            panic!("expected span element for consecutive emoji");
        }
    }

    #[test]
    fn nested_emoji_in_element() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "hi \u{2B50}");
        let p = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::empty(),
            tag: "p".to_string(),
            attributes: SmallMap::new(),
            children: vec![text],
            self_closing: false,
        });
        let mut root = make_root(&mut id_gen, vec![p]);

        apply_accessible_emoji(&mut root, &mut id_gen);

        if let HNode::Element(p_elem) = &root.children[0] {
            assert_eq!(p_elem.children.len(), 2);
            assert!(matches!(&p_elem.children[0], HNode::Text(t) if t.value == "hi "));
            if let HNode::Element(span) = &p_elem.children[1] {
                assert_eq!(span.tag, "span");
                assert_eq!(
                    span.attributes.get(&"aria-label".to_string()),
                    Some(&"star".to_string())
                );
            } else {
                panic!("expected span element");
            }
        } else {
            panic!("expected p element");
        }
    }

    #[test]
    fn emoji_at_start() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "\u{1F525} hot");
        let mut root = make_root(&mut id_gen, vec![text]);

        apply_accessible_emoji(&mut root, &mut id_gen);

        assert_eq!(root.children.len(), 2);
        assert!(matches!(&root.children[0], HNode::Element(e) if e.tag == "span"));
        assert!(matches!(&root.children[1], HNode::Text(t) if t.value == " hot"));
    }

    #[test]
    fn emoji_at_end() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "cool \u{1F60E}");
        let mut root = make_root(&mut id_gen, vec![text]);

        apply_accessible_emoji(&mut root, &mut id_gen);

        assert_eq!(root.children.len(), 2);
        assert!(matches!(&root.children[0], HNode::Text(t) if t.value == "cool "));
        assert!(matches!(&root.children[1], HNode::Element(e) if e.tag == "span"));
    }

    #[test]
    fn unknown_emoji_gets_fallback_label() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "\u{1F9D9}");
        let mut root = make_root(&mut id_gen, vec![text]);

        apply_accessible_emoji(&mut root, &mut id_gen);

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(
                elem.attributes.get(&"aria-label".to_string()),
                Some(&"emoji".to_string())
            );
        } else {
            panic!("expected span element");
        }
    }

    #[test]
    fn preserves_non_text_nodes() {
        let mut id_gen = NodeIdGen::new();
        let comment = HNode::Comment(HComment {
            id: id_gen.next_id(),
            span: Span::empty(),
            value: "a comment".to_string(),
        });
        let mut root = make_root(&mut id_gen, vec![comment]);

        apply_accessible_emoji(&mut root, &mut id_gen);

        assert_eq!(root.children.len(), 1);
        assert!(matches!(&root.children[0], HNode::Comment(_)));
    }
}
