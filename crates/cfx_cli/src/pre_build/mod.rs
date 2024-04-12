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

fn remove_ignore_source_code(user_config: &options::UserConfig) -> Result<(), std::io::Error> {
  if user_config.build.ignore.len() > 0 {
    for ignore in &user_config.build.ignore {
      let current_dir = env::current_dir()?;
      let ignore_path = current_dir.join(constant::ES_DIR).join(ignore);
      if ignore_path.exists() {
        fs::remove_dir_all(ignore_path)?;
        fs::remove_dir_all(current_dir.join(constant::LIB_DIR).join(ignore))?;
      }
    }
  }
  Ok(())
}

fn remove_demo_and_test_source_code() -> Result<(), std::io::Error> {
  let current_dir = env::current_dir()?;
  let es_dir = current_dir.join(constant::ES_DIR);
  let lib_dir = current_dir.join(constant::LIB_DIR);
  // 循环一次，获取到组件文件
  fs::read_dir(es_dir).unwrap().for_each(|entry| {
    if let Ok(entry) = entry {
      let component_path = entry.path();

      if component_path.is_dir() {
        // 再来一次获取demo和test
        fs::read_dir(&component_path)
          .unwrap()
          .for_each(|component_entry| {
            if let Ok(component_entry) = component_entry {
              let target_path = component_entry.path();
              if target_path.is_dir() {
                if target_path.ends_with("demo") || target_path.ends_with("test") {
                  fs::remove_dir_all(&target_path).unwrap();
                  fs::remove_dir_all(
                    lib_dir
                      .join(component_path.file_name().unwrap())
                      .join(target_path.file_name().unwrap()),
                  )
                  .unwrap();
                }
              }
            }
          })
      }
    }
  });
  Ok(())
}

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

  // 然后本来是构建整个包的统一入口的样式文件的，也不需要了

  // 接着删除被过滤的文件
  println!("Remove ignor Source Code");
  remove_ignore_source_code(&user_config)?;

  // 然后删除Demo 和 Test 文件
  println!("Remove Demo and Test Source Code");
  remove_demo_and_test_source_code()?;

  // // 然后构建类型文件
  // println!("Build TypeScript Definition");

  // // 然后编译es格式的文件
  // println!("Build ESModule Outputs");

  // // 然后编译cjs格式的文件
  // println!("Build CommonJS Outputs");

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
