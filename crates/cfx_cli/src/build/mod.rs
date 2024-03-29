use fs_extra::dir;
use std::env;
use std::fs;

use crate::compiler;
use crate::constant;
use crate::options;

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

fn build_package_script_entry(user_config: options::UserConfig) {
  compiler::gen_package_entry::run(user_config, "ess");
}

pub fn run(user_config: options::UserConfig) -> Result<(), std::io::Error> {
  println!("Copy Source Code.");
  copy_source_code()?;
  println!("Build Package Script Entry.");
  build_package_script_entry(user_config);
  Ok(())
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_run() {
//     compiler::gen_package_entry::run("ess");
//   }
// }
