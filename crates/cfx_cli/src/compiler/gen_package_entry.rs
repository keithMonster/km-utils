use crate::common;

use crate::options;
// pub trait PathResolver {
//   fn resolve(&self, path: &str) -> &str;
// }

// path_resolver: Option<impl Fn(&str) -> &str>
pub fn run(user_config: options::UserConfig, output_path: &str) {
  common::get_components_names(user_config)
    .iter()
    .for_each(|component| {
      println!("{}", component);
    });
}
