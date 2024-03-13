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

#[napi]
pub fn mk_dir(path: String) -> Option<bool> {
  match fs::create_dir(path) {
    Ok(_) => Some(true),
    Err(_) => None,
  }
}

#[napi]
pub fn readdir(path: String) -> Vec<String> {
  match fs::read_dir(path) {
    Ok(entries) => {
      let mut result = Vec::new();
      for entry in entries {
        if let Ok(entry) = entry {
          let file_name = entry.file_name().to_str().unwrap().to_string();
          result.push(file_name);
        }
      }
      result
    }
    Err(_) => Vec::new(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn test_dir() {
    let mut file_path = String::new();
    let mut src_path = String::new();

    if let Ok(current_dir) = env::current_dir() {
      src_path = format!("{}/src", current_dir.display().to_string());
      file_path = format!("{}/src/test", current_dir.display().to_string());
    } else {
      eprintln!("Error: Failed to get current directory");
    }

    test_mk_dir(file_path.as_str());
    test_rm_dir(file_path.as_str());
    test_readdir(src_path.as_str());
  }

  fn test_mk_dir(file_path: &str) {
    mk_dir(file_path.to_string());
    assert!(Path::new(file_path).exists());
  }

  fn test_rm_dir(file_path: &str) {
    rm_dir(file_path.to_string());
    assert!(!Path::new(file_path).exists());
  }

  fn test_readdir(path: &str) {
    let entries = readdir(path.to_string());
    assert!(!entries.is_empty());
  }
}
