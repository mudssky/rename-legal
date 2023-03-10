//! emoji相关操作，支持unicode组合字符emoji

use unicode_segmentation::UnicodeSegmentation;

///使用unicode字符串匹配单unicode字符的emoji
pub fn is_emoji(c: char) -> bool {
    match c {
        '\u{1F300}'..='\u{1F5FF}'
        | '\u{1F600}'..='\u{1F64F}'
        | '\u{1F680}'..='\u{1F6FF}'
        | '\u{2600}'..='\u{26FF}'
        | '\u{2700}'..='\u{27BF}'
        | '\u{1F900}'..='\u{1F9FF}' => true,
        _ => false,
    }
}

/// 对字符串中的emoji数量计数
pub fn count_emojis(s: &str) -> usize {
    UnicodeSegmentation::graphemes(s, true)
        // filter 过滤掉不是emoji的字符，组合unicode字符页考虑到了，因为其中只要包含emoji的字符，就是emoji
        .filter(|g| g.chars().any(is_emoji))
        .count()
}

#[allow(dead_code)]
pub fn has_emojis(s: &str) -> bool {
    count_emojis(s) > 0
}
///收集一个字符串中所有的emoji，拼接起来
pub fn get_emojis(s: &str) -> String {
    UnicodeSegmentation::graphemes(s, true)
        .filter(|g| g.chars().any(is_emoji))
        .collect()
}

/// 返回一个过滤掉emoji的字符串
pub fn replace_emojis(s: &str) -> String {
    UnicodeSegmentation::graphemes(s, true)
        .filter(|g| !g.chars().any(is_emoji))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_emojis;
    #[test]
    fn test_get_emojis() {
        assert_eq!(get_emojis("我"), "");
        assert_eq!(get_emojis("hello"), "");
        assert_eq!(get_emojis("👋 Hello, 🌎!"), "👋🌎");
        assert_eq!(get_emojis("😃🌈"), "😃🌈");
        assert_eq!(get_emojis("👍+😃=😍"), "👍😃😍");
        assert_eq!(get_emojis("❤️"), "❤️");
        assert_eq!(get_emojis("🏳️‍🌈"), "🏳️‍🌈");
        assert_eq!(get_emojis("🐱‍🏍"), "🐱‍🏍");
    }

    #[test]
    fn test_replace_emojis() {
        assert_eq!(replace_emojis("Hello, 🌎! 🏳️‍🌈"), "Hello, ! ");
        assert_eq!(replace_emojis("No emojis here"), "No emojis here");
        assert_eq!(replace_emojis(""), "");
        // 目前还无法处理多宽度字符的表情
        assert_eq!(
            replace_emojis("Emoji 1: ❤️, Emoji 2: 🚀, Emoji 3: 🐱‍🏍"),
            "Emoji 1: , Emoji 2: , Emoji 3: "
        );
    }
    #[test]

    fn test_count_emojis() {
        assert_eq!(count_emojis(""), 0);
        assert_eq!(count_emojis("This is a test string"), 0);
        assert_eq!(count_emojis("👨‍👩‍👧‍👦🎉🎂🎈"), 4);
        assert_eq!(count_emojis("🐱‍🏍"), 1);
    }

    #[test]
    fn test_has_emojis() {
        // 测试包含表情符号的字符串
        assert_eq!(has_emojis("Hello, 😊!"), true);
        // 测试不包含表情符号的字符串
        assert_eq!(has_emojis("Hello, world!"), false);
        // 测试包含多个表情符号的字符串
        assert_eq!(has_emojis("👋, 😊, 🌍"), true);
        // 测试只包含表情符号的字符串
        assert_eq!(has_emojis("😊😊😊"), true);
        // 测试空字符串
        assert_eq!(has_emojis(""), false);
    }
}
