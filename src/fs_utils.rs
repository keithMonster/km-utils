#![deny(clippy::all)]
use std::fs;
use std::path::Path;

fn rm_dir_rs(path: &str) -> Result<(), std::io::Error> {
  let dir_path = Path::new(path);
  if dir_path.is_dir() {
    fs::remove_dir_all(dir_path)?;
  }
  Ok(())
}

#[napi]
pub fn rm_dir(path: String) -> Option<bool> {
  match rm_dir_rs(path.as_str()) {
    Ok(_) => Some(true),
    Err(_) => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rm_dir() {
    let file_path = "/Users/xuke/githubProject/km-utils/src/test";
    rm_dir(file_path.to_string());
    assert!(!Path::new(file_path).exists());
  }
}
