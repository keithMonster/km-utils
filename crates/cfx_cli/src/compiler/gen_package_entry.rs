use crate::common;
use crate::options;
use std::fs;

#[allow(dead_code)]
// pro-button -> ProButton
fn convert_to_component_name(s: &str) -> String {
  s.split('-')
    .map(|part| {
      let mut chars = part.chars();
      match chars.next() {
        None => String::new(),
        Some(first_char) => {
          let mut result = String::new();
          result.push(first_char.to_uppercase().next().unwrap());
          result.push_str(chars.as_str());
          result
        }
      }
    })
    .collect::<Vec<_>>()
    .join("")
}

pub fn run(user_config: &options::UserConfig, output_path: &str) {
  let names = common::get_components_names(user_config);
  // let components: Vec<String> = names
  //   .into_iter()
  //   .map(|name| convert_to_component_name(&name))
  //   .collect();
  let exports_str = names
    .iter()
    .map(|name| format!("export * from './{}';\n", name))
    .collect::<Vec<_>>()
    .join("");

  println!("{:?}", output_path);
  match fs::write(output_path, exports_str) {
    Ok(_) => (),
    Err(_) => (),
  }
}
