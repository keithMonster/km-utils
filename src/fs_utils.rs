use std::fs;

#[napi]
pub struct FsUtils {}

#[napi]
impl FsUtils {
  #[napi]
  pub fn rmdir(path: String) -> Option<bool> {
    match fs::remove_dir_all(path) {
      Ok(_) => Some(true),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn mkdir(path: String) -> Option<bool> {
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

  #[napi]
  pub fn write(path: String, contents: String) -> bool {
    match fs::write(path, contents) {
      Ok(_) => true,
      Err(_) => false,
    }
  }

  #[napi]
  pub fn read(path: String) -> String {
    match fs::read_to_string(path) {
      Ok(contents) => contents,
      Err(_) => "".to_string(),
    }
  }

  #[napi]
  pub fn rm(path: String) -> Option<bool> {
    match fs::remove_file(path) {
      Ok(_) => Some(true),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn is_dir(path: String) -> Option<bool> {
    match fs::metadata(path) {
      Ok(metadata) => Some(metadata.is_dir()),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn is_file(path: String) -> Option<bool> {
    match fs::metadata(path) {
      Ok(metadata) => Some(metadata.is_file()),
      Err(_) => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;
  use std::path::Path;

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
    test_readdir(&src_path);
    test_is_dir(&src_path);
    test_is_file(&src_path);
  }

  fn test_mk_dir(file_path: &str) {
    FsUtils::mkdir(file_path.to_string());
    assert!(Path::new(file_path).exists());
  }

  fn test_rm_dir(file_path: &str) {
    FsUtils::rmdir(file_path.to_string());
    assert!(!Path::new(file_path).exists());
  }

  fn test_readdir(path: &str) {
    let entries = FsUtils::readdir(path.to_string());
    assert!(!entries.is_empty());
  }

  fn test_is_dir(path: &str) {
    let is_dir = FsUtils::is_dir(path.to_string());
    assert!(is_dir.unwrap());
  }

  fn test_is_file(path: &str) {
    let is_file = FsUtils::is_file(path.to_string());
    assert!(!is_file.unwrap());
  }
}
