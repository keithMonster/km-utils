#![deny(clippy::all)]
use std::path::Path;

fn extname_rs(path: &str) -> Option<&str> {
  Path::new(path).extension().and_then(|ext| ext.to_str())
}

fn basename_rs(path: &str, ext: Option<&str>) -> String {
  let mut base = Path::new(path)
    .file_name()
    .unwrap()
    .to_str()
    .unwrap_or_default()
    .to_string();

  if let Some(ext) = ext {
    if base.ends_with(ext) {
      base.truncate(base.len() - ext.len());
    }
  }

  base
}

#[napi]
pub struct PathUtils {}

#[napi]
impl PathUtils {
  #[napi]
  pub fn basename(path: String, ext: Option<String>) -> String {
    basename_rs(path.as_str(), ext.as_deref())
  }

  #[napi]
  pub fn extname(path: String) -> Option<String> {
    match extname_rs(path.as_str()) {
      Some(ext) => Some(ext.to_string()),
      None => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn test_dir() {
    let mut file_path = String::new();
    // let mut src_path = String::new();

    if let Ok(current_dir) = env::current_dir() {
      // src_path = format!("{}/src", current_dir.display().to_string());
      file_path = format!("{}/src/path_utils.rs", current_dir.display().to_string());
    } else {
      eprintln!("Error: Failed to get current directory");
    }

    test_extname(file_path.as_str());
    text_basename();
  }

  fn test_extname(file_path: &str) {
    let ext = PathUtils::extname(file_path.to_string());
    assert!(ext.unwrap() == "rs");
  }

  fn text_basename() {
    let file_path = "src/cc/path_utils.rs";
    let ext = ".rs".to_string();
    let base = PathUtils::basename(file_path.to_string(), Some(ext));
    assert!(base == "path_utils");
  }
}
