use crate::ast::mdast::nodes::{Document, MdNode};

static EMOJI_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "smile" => "\u{1F604}",
    "laughing" => "\u{1F606}",
    "joy" => "\u{1F602}",
    "rofl" => "\u{1F923}",
    "grinning" => "\u{1F600}",
    "wink" => "\u{1F609}",
    "blush" => "\u{1F60A}",
    "heart_eyes" => "\u{1F60D}",
    "kissing_heart" => "\u{1F618}",
    "thinking" => "\u{1F914}",
    "neutral_face" => "\u{1F610}",
    "expressionless" => "\u{1F611}",
    "unamused" => "\u{1F612}",
    "sweat" => "\u{1F613}",
    "pensive" => "\u{1F614}",
    "confused" => "\u{1F615}",
    "disappointed" => "\u{1F61E}",
    "worried" => "\u{1F61F}",
    "angry" => "\u{1F620}",
    "rage" => "\u{1F621}",
    "cry" => "\u{1F622}",
    "sob" => "\u{1F62D}",
    "scream" => "\u{1F631}",
    "sunglasses" => "\u{1F60E}",
    "nerd_face" => "\u{1F913}",
    "clown_face" => "\u{1F921}",
    "skull" => "\u{1F480}",
    "ghost" => "\u{1F47B}",
    "alien" => "\u{1F47D}",
    "robot" => "\u{1F916}",
    "poop" => "\u{1F4A9}",
    "+1" => "\u{1F44D}",
    "thumbsup" => "\u{1F44D}",
    "-1" => "\u{1F44E}",
    "thumbsdown" => "\u{1F44E}",
    "ok_hand" => "\u{1F44C}",
    "wave" => "\u{1F44B}",
    "clap" => "\u{1F44F}",
    "raised_hands" => "\u{1F64C}",
    "pray" => "\u{1F64F}",
    "muscle" => "\u{1F4AA}",
    "point_up" => "\u{261D}\u{FE0F}",
    "point_down" => "\u{1F447}",
    "point_left" => "\u{1F448}",
    "point_right" => "\u{1F449}",
    "middle_finger" => "\u{1F595}",
    "v" => "\u{270C}\u{FE0F}",
    "metal" => "\u{1F918}",
    "crossed_fingers" => "\u{1F91E}",
    "heart" => "\u{2764}\u{FE0F}",
    "broken_heart" => "\u{1F494}",
    "sparkling_heart" => "\u{1F496}",
    "star" => "\u{2B50}",
    "star2" => "\u{1F31F}",
    "sparkles" => "\u{2728}",
    "fire" => "\u{1F525}",
    "boom" => "\u{1F4A5}",
    "zap" => "\u{26A1}",
    "100" => "\u{1F4AF}",
    "tada" => "\u{1F389}",
    "trophy" => "\u{1F3C6}",
    "medal_sports" => "\u{1F3C5}",
    "rocket" => "\u{1F680}",
    "bulb" => "\u{1F4A1}",
    "gear" => "\u{2699}\u{FE0F}",
    "wrench" => "\u{1F527}",
    "hammer" => "\u{1F528}",
    "link" => "\u{1F517}",
    "lock" => "\u{1F512}",
    "key" => "\u{1F511}",
    "mag" => "\u{1F50D}",
    "bell" => "\u{1F514}",
    "mega" => "\u{1F4E3}",
    "memo" => "\u{1F4DD}",
    "pencil" => "\u{270F}\u{FE0F}",
    "book" => "\u{1F4D6}",
    "books" => "\u{1F4DA}",
    "computer" => "\u{1F4BB}",
    "iphone" => "\u{1F4F1}",
    "email" => "\u{1F4E7}",
    "package" => "\u{1F4E6}",
    "white_check_mark" => "\u{2705}",
    "x" => "\u{274C}",
    "warning" => "\u{26A0}\u{FE0F}",
    "construction" => "\u{1F6A7}",
    "no_entry" => "\u{26D4}",
    "question" => "\u{2753}",
    "exclamation" => "\u{2757}",
    "information_source" => "\u{2139}\u{FE0F}",
    "sunny" => "\u{2600}\u{FE0F}",
    "cloud" => "\u{2601}\u{FE0F}",
    "umbrella" => "\u{2602}\u{FE0F}",
    "rainbow" => "\u{1F308}",
    "snowflake" => "\u{2744}\u{FE0F}",
    "ocean" => "\u{1F30A}",
    "earth_americas" => "\u{1F30E}",
    "coffee" => "\u{2615}",
    "pizza" => "\u{1F355}",
    "beer" => "\u{1F37A}",
    "cake" => "\u{1F370}",
    "arrow_up" => "\u{2B06}\u{FE0F}",
    "arrow_down" => "\u{2B07}\u{FE0F}",
    "arrow_left" => "\u{2B05}\u{FE0F}",
    "arrow_right" => "\u{27A1}\u{FE0F}",
    "arrow_right_hook" => "\u{21AA}\u{FE0F}",
    "recycle" => "\u{267B}\u{FE0F}",
    "bug" => "\u{1F41B}",
    "art" => "\u{1F3A8}",
    "eyes" => "\u{1F440}",
    "see_no_evil" => "\u{1F648}",
    "heavy_check_mark" => "\u{2714}\u{FE0F}",
    "heavy_plus_sign" => "\u{2795}",
    "heavy_minus_sign" => "\u{2796}",
};

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
