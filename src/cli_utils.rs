use cfx_cli;
#[napi]
pub struct CliUtils {}

#[napi(object)]
pub struct BuildConfig {
  pub ignore: Vec<String>,
  // ... 其他字段
}

#[napi(object)]
pub struct UserConfig {
  pub name: String,
  pub build: BuildConfig, // ... 其他字段
}

#[napi]
impl CliUtils {
  #[napi]
  pub fn clear() -> () {
    match cfx_cli::clear::run() {
      Ok(_) => (),
      Err(e) => panic!("{}", e),
    }
  }
  #[napi]
  pub fn pre_build(user_config: UserConfig) -> () {
    let user_config = cfx_cli::options::UserConfig {
      name: user_config.name,
      build: cfx_cli::options::BuildConfig {
        ignore: user_config.build.ignore,
      },
    };
    match cfx_cli::pre_build::run(user_config) {
      Ok(_) => (),
      Err(e) => panic!("{}", e),
    }
  }
  #[napi]
  pub fn compile_script(path: String, compile_type: String) -> () {
    match cfx_cli::compiler::compile_script::run(&path, &compile_type) {
      Ok(_) => (),
      Err(e) => panic!("{}", e),
    }
  }
}
