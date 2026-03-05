use crate::ast::mdast::nodes::{Document, MdNode};
use std::collections::HashMap;
use std::sync::LazyLock;

static EMOJI_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("smile", "\u{1F604}");
    m.insert("laughing", "\u{1F606}");
    m.insert("joy", "\u{1F602}");
    m.insert("rofl", "\u{1F923}");
    m.insert("grinning", "\u{1F600}");
    m.insert("wink", "\u{1F609}");
    m.insert("blush", "\u{1F60A}");
    m.insert("heart_eyes", "\u{1F60D}");
    m.insert("kissing_heart", "\u{1F618}");
    m.insert("thinking", "\u{1F914}");
    m.insert("neutral_face", "\u{1F610}");
    m.insert("expressionless", "\u{1F611}");
    m.insert("unamused", "\u{1F612}");
    m.insert("sweat", "\u{1F613}");
    m.insert("pensive", "\u{1F614}");
    m.insert("confused", "\u{1F615}");
    m.insert("disappointed", "\u{1F61E}");
    m.insert("worried", "\u{1F61F}");
    m.insert("angry", "\u{1F620}");
    m.insert("rage", "\u{1F621}");
    m.insert("cry", "\u{1F622}");
    m.insert("sob", "\u{1F62D}");
    m.insert("scream", "\u{1F631}");
    m.insert("sunglasses", "\u{1F60E}");
    m.insert("nerd_face", "\u{1F913}");
    m.insert("clown_face", "\u{1F921}");
    m.insert("skull", "\u{1F480}");
    m.insert("ghost", "\u{1F47B}");
    m.insert("alien", "\u{1F47D}");
    m.insert("robot", "\u{1F916}");
    m.insert("poop", "\u{1F4A9}");
    m.insert("+1", "\u{1F44D}");
    m.insert("thumbsup", "\u{1F44D}");
    m.insert("-1", "\u{1F44E}");
    m.insert("thumbsdown", "\u{1F44E}");
    m.insert("ok_hand", "\u{1F44C}");
    m.insert("wave", "\u{1F44B}");
    m.insert("clap", "\u{1F44F}");
    m.insert("raised_hands", "\u{1F64C}");
    m.insert("pray", "\u{1F64F}");
    m.insert("muscle", "\u{1F4AA}");
    m.insert("point_up", "\u{261D}\u{FE0F}");
    m.insert("point_down", "\u{1F447}");
    m.insert("point_left", "\u{1F448}");
    m.insert("point_right", "\u{1F449}");
    m.insert("middle_finger", "\u{1F595}");
    m.insert("v", "\u{270C}\u{FE0F}");
    m.insert("metal", "\u{1F918}");
    m.insert("crossed_fingers", "\u{1F91E}");
    m.insert("heart", "\u{2764}\u{FE0F}");
    m.insert("broken_heart", "\u{1F494}");
    m.insert("sparkling_heart", "\u{1F496}");
    m.insert("star", "\u{2B50}");
    m.insert("star2", "\u{1F31F}");
    m.insert("sparkles", "\u{2728}");
    m.insert("fire", "\u{1F525}");
    m.insert("boom", "\u{1F4A5}");
    m.insert("zap", "\u{26A1}");
    m.insert("100", "\u{1F4AF}");
    m.insert("tada", "\u{1F389}");
    m.insert("trophy", "\u{1F3C6}");
    m.insert("medal_sports", "\u{1F3C5}");
    m.insert("rocket", "\u{1F680}");
    m.insert("bulb", "\u{1F4A1}");
    m.insert("gear", "\u{2699}\u{FE0F}");
    m.insert("wrench", "\u{1F527}");
    m.insert("hammer", "\u{1F528}");
    m.insert("link", "\u{1F517}");
    m.insert("lock", "\u{1F512}");
    m.insert("key", "\u{1F511}");
    m.insert("mag", "\u{1F50D}");
    m.insert("bell", "\u{1F514}");
    m.insert("mega", "\u{1F4E3}");
    m.insert("memo", "\u{1F4DD}");
    m.insert("pencil", "\u{270F}\u{FE0F}");
    m.insert("book", "\u{1F4D6}");
    m.insert("books", "\u{1F4DA}");
    m.insert("computer", "\u{1F4BB}");
    m.insert("iphone", "\u{1F4F1}");
    m.insert("email", "\u{1F4E7}");
    m.insert("package", "\u{1F4E6}");
    m.insert("white_check_mark", "\u{2705}");
    m.insert("x", "\u{274C}");
    m.insert("warning", "\u{26A0}\u{FE0F}");
    m.insert("construction", "\u{1F6A7}");
    m.insert("no_entry", "\u{26D4}");
    m.insert("question", "\u{2753}");
    m.insert("exclamation", "\u{2757}");
    m.insert("information_source", "\u{2139}\u{FE0F}");
    m.insert("sunny", "\u{2600}\u{FE0F}");
    m.insert("cloud", "\u{2601}\u{FE0F}");
    m.insert("umbrella", "\u{2602}\u{FE0F}");
    m.insert("rainbow", "\u{1F308}");
    m.insert("snowflake", "\u{2744}\u{FE0F}");
    m.insert("ocean", "\u{1F30A}");
    m.insert("earth_americas", "\u{1F30E}");
    m.insert("coffee", "\u{2615}");
    m.insert("pizza", "\u{1F355}");
    m.insert("beer", "\u{1F37A}");
    m.insert("cake", "\u{1F370}");
    m.insert("arrow_up", "\u{2B06}\u{FE0F}");
    m.insert("arrow_down", "\u{2B07}\u{FE0F}");
    m.insert("arrow_left", "\u{2B05}\u{FE0F}");
    m.insert("arrow_right", "\u{27A1}\u{FE0F}");
    m.insert("arrow_right_hook", "\u{21AA}\u{FE0F}");
    m.insert("recycle", "\u{267B}\u{FE0F}");
    m.insert("bug", "\u{1F41B}");
    m.insert("art", "\u{1F3A8}");
    m.insert("eyes", "\u{1F440}");
    m.insert("see_no_evil", "\u{1F648}");
    m.insert("heavy_check_mark", "\u{2714}\u{FE0F}");
    m.insert("heavy_plus_sign", "\u{2795}");
    m.insert("heavy_minus_sign", "\u{2796}");
    m
});

pub fn apply_emoji(doc: &mut Document) {
    apply_to_children(&mut doc.children);
}

fn apply_to_children(children: &mut [MdNode]) {
    for child in children.iter_mut() {
        if let MdNode::Text(text) = child
            && text.value.contains(':')
        {
            text.value = replace_emoji_shortcodes(&text.value);
        }
        if matches!(child, MdNode::Code(_) | MdNode::InlineCode(_)) {
            continue;
        }
        if let Some(kids) = child.children_mut() {
            apply_to_children(kids);
        }
    }
}

fn replace_emoji_shortcodes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        if ch == ':' {
            let _start = i;
            let mut name = String::new();
            let mut found = false;
            for (j, ch2) in chars.by_ref() {
                if ch2 == ':' {
                    if let Some(emoji) = EMOJI_MAP.get(name.as_str()) {
                        result.push_str(emoji);
                        found = true;
                    } else {
                        result.push(':');
                        result.push_str(&name);
                        result.push(':');
                        found = true;
                    }
                    break;
                } else if ch2.is_alphanumeric() || ch2 == '_' || ch2 == '+' || ch2 == '-' {
                    name.push(ch2);
                } else {
                    result.push(':');
                    result.push_str(&input[_start + 1..=j]);
                    found = true;
                    break;
                }
            }
            if !found {
                result.push(':');
                result.push_str(&name);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_known_shortcode() {
        assert_eq!(replace_emoji_shortcodes(":smile:"), "\u{1F604}");
        assert_eq!(replace_emoji_shortcodes(":rocket:"), "\u{1F680}");
        assert_eq!(replace_emoji_shortcodes(":+1:"), "\u{1F44D}");
    }

    #[test]
    fn keeps_unknown_shortcode() {
        assert_eq!(replace_emoji_shortcodes(":unknown:"), ":unknown:");
    }

    #[test]
    fn mixed_text() {
        let result = replace_emoji_shortcodes("Hello :wave: world :rocket:!");
        assert!(result.contains("\u{1F44B}"));
        assert!(result.contains("\u{1F680}"));
        assert!(result.contains("Hello"));
    }

    #[test]
    fn handles_no_shortcodes() {
        assert_eq!(replace_emoji_shortcodes("no emoji here"), "no emoji here");
    }
}
