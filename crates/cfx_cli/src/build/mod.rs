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

fn build_package_script_entry(user_config: &options::UserConfig) -> Result<(), std::io::Error> {
  let current_dir = env::current_dir()?;
  let output_path = current_dir.join(constant::ES_DIR).join("index.js");
  compiler::gen_package_entry::run(user_config, output_path.to_str().unwrap());
  fs::copy(
    current_dir.join(constant::ES_DIR).join("index.js"),
    current_dir.join(constant::LIB_DIR).join("index.js"),
  )?;
  Ok(())
}

// fn build_component_style_entry(user_config: &options::UserConfig) -> Result<(), std::io::Error> {
//   // let current_dir = env::current_dir()?;
//   compiler::gen_style_deps_map::run(user_config);
//   compiler::gen_component_style_entry::run(user_config);
//   Ok(())
// }

pub fn run(user_config: options::UserConfig) -> Result<(), std::io::Error> {
  // 先把源代码复制到目标文件
  println!("Copy Source Code.");
  copy_source_code()?;
  // 然后构建js桶文件，导出所有组件
  println!("Build Package Script Entry.");
  build_package_script_entry(&user_config)?;
  // // 接着构建每一个组件的样式文件入口---不需要了
  // println!("Build Component Style Entry.");
  // build_component_style_entry(&user_config)?;

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
