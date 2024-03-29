use cfx_cli;
#[napi]
pub struct CliUtils {}

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
  pub fn build() -> () {
    match cfx_cli::build::run() {
      Ok(_) => (),
      Err(e) => panic!("{}", e),
    }
  }
}
