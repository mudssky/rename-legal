//! windows非法字符相关操作
use regex::Regex;
const ILLEGAL_REGEX: &str = r#"[\\/:*?"<>|]"#;

///获取字符串中所有的windows非法字符
pub fn get_windows_illegal_characters(text: &str) -> String {
    let windows_illegal_pattern = Regex::new(ILLEGAL_REGEX).unwrap();
    let res = windows_illegal_pattern.find_iter(text);
    let mut new_text = String::new();
    for i in res {
        new_text.push_str(i.as_str());
    }
    new_text
}

pub fn replace_win_illegal(filename: &str) -> String {
    // 匹配windows下的不合法字符
    let windows_illegal_pattern = Regex::new(ILLEGAL_REGEX).unwrap();
    // 去除windows下不合法的字符
    windows_illegal_pattern
        .replace_all(filename, "")
        .into_owned()
}

#[cfg(test)]
mod tests {
    use crate::win_illegal::{get_windows_illegal_characters, replace_win_illegal};

    #[test]
    fn test_get_windows_illegal_characters() {
        let test_cases = vec![
            ("foo?bar*|baz\"", "?*|\""),
            ("some text with no illegal characters", ""),
            ("<dir>/subdir", "<>/"),
            ("file\\name", "\\"),
        ];
        for (text, expected) in test_cases {
            assert_eq!(get_windows_illegal_characters(text), expected);
        }
    }

    #[test]
    fn test_replace_win_illegal() {
        let test_cases = vec![
            ("file\\name.txt", "filename.txt"),
            ("doc/somefile.docx", "docsomefile.docx"),
            ("text|file.txt", "textfile.txt"),
            ("<dir>/subdir", "dirsubdir"),
            ("noillegal", "noillegal"),
            ("", ""),
        ];
        for (filename, expected) in test_cases {
            assert_eq!(replace_win_illegal(filename), expected);
        }
    }
}
