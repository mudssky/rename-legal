use clap::{Parser, Subcommand};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

/// 对文件名字符串进行处理的程序,用于文件重命名的时候去除非法字符串和emojis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// 检查字符串是否符合匹配条件,比如是否含有emoji或者windows下的非法字符
    Check {
        /// 输入的字符串
        // #[arg(short, long)]
        input_text: String,
        /// 是否存在emoji
        #[arg(long, default_value_t = true)]
        has_emoji: bool,

        /// 是否存在windows中的非法字符 r#"[\\/:*?"<>|]"#
        #[arg(long, default_value_t = true)]
        has_windows_illegal_characters: bool,

        /// 显示更详细的信息
        #[arg(short, long)]
        verbose: bool,
    },
    /// 替换字符串中的非法字符
    Replace {
        /// 输入的字符串
        // #[arg(short, long)]
        input_text: String,
        /// 是否移除emojis
        #[arg(long, default_value_t = true)]
        remove_emoji: bool,
        /// 是否移除windows文件名不合法的字符
        #[arg(long, default_value_t = true)]
        remove_windows_illegal_characters: bool,
        ///  匹配移除的字符的正则
        #[arg(short = 'p', long)]
        remove_pattern: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Check {
            has_emoji,
            input_text,
            has_windows_illegal_characters,
            verbose,
        } => {
            // println!("检查分支");
            let mut check_flag = false;
            if *has_emoji {
                let emojis = get_emojis(input_text);
                let emoji_count = emojis.chars().count();
                if emoji_count > 0 {
                    check_flag = true;
                    if *verbose {
                        println!("存在{}个emojis:{}", emoji_count, emojis)
                    }
                }
            }
            if *has_windows_illegal_characters {
                let windows_illegal_characters = get_windows_illegal_characters(input_text);
                if windows_illegal_characters.len() > 0 {
                    check_flag = true;
                    if *verbose {
                        println!(
                            "存在{}个windows下的非法字符:{}",
                            windows_illegal_characters.len(),
                            windows_illegal_characters
                        )
                    }
                }
            }
            println!("{}", check_flag);
        }
        Commands::Replace {
            input_text,
            remove_emoji,
            remove_windows_illegal_characters,
            remove_pattern,
        } => {
            let mut input_text = input_text.clone();
            if *remove_windows_illegal_characters {
                input_text = windows_legaling_name(&input_text);
            }
            if *remove_emoji {
                input_text = replace_emojis(&input_text);
            }
            if let Some(remove_pattern) = remove_pattern {
                let remove_re = Regex::new(&remove_pattern).unwrap();
                input_text = remove_re.replace_all(&input_text, "").into_owned();
            }
            println!("{}", input_text);
        }
    }
}
fn windows_legaling_name(filename: &str) -> String {
    // 匹配windows下的不合法字符
    let windows_illegal_pattern = Regex::new(r#"[\\/:*?"<>|]"#).unwrap();
    // 去除windows下不合法的字符
    windows_illegal_pattern
        .replace_all(filename, "")
        .into_owned()
}

// 引入emojis包,遍历每个字符,判断是否为emoji,如果是就移除
fn replace_emojis(text: &str) -> String {
    let mut new_text = String::new();

    let grapheme_clusters = UnicodeSegmentation::graphemes(text, true).collect::<Vec<&str>>();
    for c in grapheme_clusters {
        // println!("{}", c);
        let chars = c.chars();
        let count = chars.count();
        // unicode字符数量大于1，说明是一个符合字符，可能是emoji
        if count > 1 {
            continue;
        }
        if let None = emojis::get(c) {
            new_text.push_str(c);
        }
    }
    new_text
}

#[allow(dead_code)]
fn check_emojis(text: &str) -> bool {
    let replaced_emojis = replace_emojis(text);
    if replaced_emojis.len() < text.len() {
        return true;
    }
    return false;
}

///获取字符串中的所有emoji，拼接emoji到新字符串
fn get_emojis(text: &str) -> String {
    let mut new_text = String::new();
    // 使用unicode分段器，将包含unicode字符串按照unicode分开成列表
    // 如果直接用text.chars(),有些emoji表情整体不能被当成一个字符，会被拆分开导致错误。
    let grapheme_clusters = UnicodeSegmentation::graphemes(text, true).collect::<Vec<&str>>();
    for c in grapheme_clusters {
        // println!("{}", c);
        let chars = c.chars();
        let count = chars.count();
        // unicode字符数量大于1，说明是一个符合字符，可能是emoji
        if count > 1 {
            new_text.push_str(c);
            continue;
        }
        if let Some(emoji) = emojis::get(c) {
            new_text.push_str(emoji.as_str())
        }
    }
    new_text
}

fn get_windows_illegal_characters(text: &str) -> String {
    let windows_illegal_pattern = Regex::new(r#"[\\/:*?"<>|]"#).unwrap();
    let res = windows_illegal_pattern.find_iter(text);
    let mut new_text = String::new();
    for i in res {
        new_text.push_str(i.as_str());
    }
    new_text
}

pub fn count_emojis(s: &str) -> usize {
    UnicodeSegmentation::graphemes(s, true)
        .filter(|g| g.chars().any(is_emoji))
        .count()
}

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
}
