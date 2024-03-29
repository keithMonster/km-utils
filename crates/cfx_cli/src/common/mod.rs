use std::env;
use std::fs;

use crate::constant;
use crate::options;

const EXCLUDES: [&str; 1] = [".DS_Store"];

pub fn get_components_names(user_config: options::UserConfig) -> Vec<String> {
  println!("user_config.build.ignore,{:?}", user_config.build.ignore);
  if let Ok(current_dir) = env::current_dir() {
    let dirs = current_dir.join(constant::SRC_DIR);
    // 先读取src下所有文件，过滤一次默认的，再过滤一次自定义的，然后判断下下面的index.ts 有 export default
    fs::read_dir(dirs)
      .unwrap()
      .map(|file| file.unwrap().file_name().into_string().unwrap())
      .filter(|file_name| !EXCLUDES.contains(&file_name.as_str()))
      .filter(|file_name| !user_config.build.ignore.contains(file_name))
      .filter(|file_name| -> bool {
        let file_path = current_dir
          .join(constant::SRC_DIR)
          .join(file_name)
          .join("index.ts");

        if let Ok(file_content) = fs::read_to_string(file_path) {
          file_content.contains("export default")
        } else {
          false
        }
      })
      .collect()
  } else {
    Vec::new()
  }
}
