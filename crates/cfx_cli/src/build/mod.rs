// use fs_extra::dir;
use std::env;
// use std::fs;

use std::fs;
use std::path::PathBuf;

// use fs_extra::file;

// use crate::compiler;
use crate::constant;

fn is_script(file_path: &PathBuf) -> bool {
  ["js", "ts", "jsx", "tsx"].contains(&file_path.extension().unwrap().to_str().unwrap())
}
fn is_style(file_path: &PathBuf) -> bool {
  ["css", "less"].contains(&file_path.extension().unwrap().to_str().unwrap())
}
fn is_asset(file_path: &PathBuf) -> bool {
  ["png", "jpg", "jpeg", "gif", "svg", "webp"]
    .contains(&file_path.extension().unwrap().to_str().unwrap())
}

fn compile_script(file_path: &PathBuf, format: &str) {
  println!("{}", file_path.to_str().unwrap());
  println!("{}", format);
}
fn compile_style(file_path: &PathBuf) {
  if file_path.extension().unwrap().to_str().unwrap() == "less" {
    println!("less");
  }
}

fn compile_file(file_path: PathBuf, format: &str) -> Result<(), std::io::Error> {
  println!("{}", file_path.to_str().unwrap());
  if is_script(&file_path) {
    compile_script(&file_path, format);
  }
  if is_style(&file_path) {
    compile_style(&file_path);
  }
  if is_asset(&file_path) {
    return Ok(());
  }
  fs::remove_file(file_path)
}

fn compile_dir(dir_path: PathBuf, format: &str) -> Result<(), std::io::Error> {
  for entry in fs::read_dir(dir_path)? {
    let dir = entry?;
    if dir.path().is_dir() {
      compile_dir(dir.path(), format)?;
    } else {
      compile_file(dir.path(), format)?;
    }
  }
  Ok(())
}

pub fn run() -> Result<(), std::io::Error> {
  let current_dir = env::current_dir()?;
  let es_dir = current_dir.join(constant::ES_DIR);

  // 然后编译es格式的文件
  println!("Build ESModule Outputs");
  compile_dir(es_dir, "esm")?;

  // 然后编译cjs格式的文件
  println!("Build CommonJS Outputs");
  // let lib_dir = current_dir.join(constant::LIB_DIR);

  Ok(())
}
