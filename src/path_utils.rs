#![deny(clippy::all)]
use std::path::Path;

fn extname_rs(path: &str) -> Option<&str> {
  Path::new(path).extension().and_then(|ext| ext.to_str())
}

#[napi]
pub fn extname(path: String) -> Option<String> {
  match extname_rs(path.as_str()) {
    Some(ext) => Some(ext.to_string()),
    None => None,
  }
}

pub fn basename_rs(path: &str, ext: Option<&str>) -> String {
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
pub fn basename(path: String, ext: String) -> String {
  basename_rs(path.as_str(), Some(ext.as_str())).to_string()
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
    let ext = extname(file_path.to_string());
    assert!(ext.unwrap() == "rs");
  }

  fn text_basename() {
    let file_path = "src/path_utils.rs";
    let ext = ".rs";
    let base = basename(file_path.to_string(), ext.to_string());
    assert!(base == "path_utils");
  }
}
