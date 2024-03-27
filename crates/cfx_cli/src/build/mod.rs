use fs_extra::dir;
use std::env;
use std::fs;

use crate::constant;

fn copy_dir(from: &str, to: &str) -> Result<(), std::io::Error> {
  let current_dir = env::current_dir()?;
  let from_path = current_dir.join(from);
  let to_path = current_dir.join(to);

  let options = dir::CopyOptions {
    content_only: true,
    ..Default::default()
  };

  if from_path.exists() {
    if !to_path.exists() {
      fs::create_dir(&to_path)?;
    }
    dir::copy(&from_path, &to_path, &options).unwrap();
  }
  Ok(())
}

fn copy_source_code() -> Result<(), std::io::Error> {
  copy_dir(constant::SRC_DIR, constant::ES_DIR)?;
  copy_dir(constant::SRC_DIR, constant::LIB_DIR)?;
  Ok(())
}

pub fn run() -> Result<(), std::io::Error> {
  copy_source_code()
}
