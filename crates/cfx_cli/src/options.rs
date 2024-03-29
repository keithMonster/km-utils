#[derive(Debug)]
pub struct BuildConfig {
  pub ignore: Vec<String>,
  // ... 其他字段
}

#[derive(Debug)]
pub struct UserConfig {
  pub name: String,
  pub build: BuildConfig,
  // ... 其他字段
}
