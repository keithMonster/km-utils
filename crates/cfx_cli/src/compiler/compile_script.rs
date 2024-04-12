use std::path::PathBuf;

pub struct ScriptCompiler {
  // ... 其他字段
  pub path: PathBuf,
}
impl ScriptCompiler {
  pub fn new(path: PathBuf) -> ScriptCompiler {
    ScriptCompiler { path }
  }
  pub fn compile(&self) -> () {
    println!("{:?}", &self.path)
  }
}

pub fn run(file_path: &str, compile_type: &str) -> Result<(), std::io::Error> {
  println!("{}", file_path);
  println!("{}", compile_type);
  // PathBuf::from(file_path);
  ScriptCompiler::new(PathBuf::from(file_path));
  Ok(())
}
