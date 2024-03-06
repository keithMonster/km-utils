#![deny(clippy::all)]

use regex::Regex;

fn replace_text_rs(original: &str, pattern: &str, replacement: &str) -> String {
  // 创建正则表达式
  let re = Regex::new(pattern).unwrap();

  // 使用正则表达式进行替换
  let replaced = re.replace_all(original, replacement);

  // 返回替换后的字符串
  replaced.to_string()
}

#[napi]
pub fn replace_text(original: String, pattern: String, replacement: String) -> String {
  let replaced_str = replace_text_rs(original.as_str(), pattern.as_str(), replacement.as_str());

  replaced_str
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let original_str = r#"import { chatPageInitSdk } from '@shared/chat-page'
  import { chatPageInitSdk } from '@shared/chat-page2'"#;

    let pattern = r#"from '@shared/([^']+)'"#;
    let replacement = "from './_shared/$1'";

    assert_eq!(
      replace_text(
        original_str.to_string(),
        pattern.to_string(),
        replacement.to_string()
      ),
      r#"import { chatPageInitSdk } from './_shared/chat-page'
  import { chatPageInitSdk } from './_shared/chat-page2'"#
    );
  }
}
