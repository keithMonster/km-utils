use std::env;
use std::fs;

use crate::constant;

fn rm_dir(path: &str) -> Result<(), std::io::Error> {
  let current_dir = env::current_dir()?;
  if current_dir.join(path).exists() {
    fs::remove_dir_all(current_dir.join(path))?;
  }
  Ok(())
}

pub fn run() -> Result<(), std::io::Error> {
  rm_dir(constant::ES_DIR)?;
  rm_dir(constant::LIB_DIR)?;
  rm_dir(constant::DIST_DIR)?;
  rm_dir(constant::SITE_DIST_DIR)?;
  Ok(())
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_run() {
//     run()
//   }
// }
