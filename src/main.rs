mod emoji;
mod win_illegal;

use clap::{Parser, Subcommand};
use regex::Regex;

use crate::{
    emoji::{count_emojis, get_emojis, replace_emojis},
    win_illegal::{get_windows_illegal_characters, replace_win_illegal},
};

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
                let emoji_count = count_emojis(input_text);
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
                input_text = replace_win_illegal(&input_text);
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
