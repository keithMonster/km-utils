use crate::options;

pub fn run(user_config: &options::UserConfig) {
  println!("Hello, world!{:?}", user_config);
}
