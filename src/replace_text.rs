#![deny(clippy::all)]

use regex::Regex;

fn replace_shared_imports(original: &str, pattern: &str, replacement: &str) -> String {
  // 创建正则表达式
  let re = Regex::new(pattern).unwrap();

  // 使用正则表达式进行替换
  let replaced = re.replace_all(original, replacement);

  // 返回替换后的字符串
  replaced.to_string()
}

#[napi]
pub(crate) fn replace_text(original: String, pattern: String, replacement: String) -> String {
  let replaced_str =
    replace_shared_imports(original.as_str(), pattern.as_str(), replacement.as_str());

  replaced_str
}
