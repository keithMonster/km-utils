use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;

use crate::common;
use crate::constant;
use crate::options;

fn match_path(target: &str, component_name: &str) -> bool {
  target
    .split(MAIN_SEPARATOR)
    .collect::<Vec<&str>>()
    .contains(&component_name)
}

fn match_imports(code: &str) -> Vec<String> {
  let import_re = Regex::new(
    r#"import\s+?(?:(?:(?:[\w*\s{},]*)\s+from(\s+)?)|)(?:(?:".*?")|(?:'.*?'))[\s]*?(?:;|$|)"#,
  )
  .unwrap();
  let imports: Vec<&str> = import_re.find_iter(code).map(|m| m.as_str()).collect();
  imports
    .into_iter()
    .filter(|line| !line.contains("import type"))
    .map(String::from)
    .collect()
}
// 获取单个组件的依赖关系
fn get_deps(file_path: PathBuf, deps_map: &mut HashMap<String, Vec<String>>) -> Vec<String> {
  let code = fs::read_to_string(file_path).unwrap();
  let imports: Vec<String> = match_imports(&code);

  println!("get_deps:code,{:?}", code);
  println!("get_deps:imports,{:?}", imports);
  println!("get_deps:deps_map,{:?}", deps_map);
  Vec::new()
}

// 分析组件依赖关系
fn get_component_style_deps(
  component_name: &str,
  component_names: &Vec<String>,
  deps_map: &mut HashMap<String, Vec<String>>,
) -> Vec<String> {
  let mut deps: Vec<String> = Vec::new();
  let mut visited: HashSet<String> = HashSet::new();

  let mut search = |visited: &mut HashSet<String>,
                    deps: &mut Vec<String>,
                    component_name: &str,
                    component_names: &Vec<String>| {
    visited.insert(component_name.to_string());

    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir
      .join(constant::SRC_DIR)
      .join(component_name)
      .join("index.ts");

    println!("get_component_style_deps:file_path,{:?}", file_path);
    for dep in get_deps(file_path, deps_map) {
      if visited.contains(&dep) {
        continue;
      }

      for item in component_names
        .iter()
        .filter(|&name| match_path(&dep, name))
      {
        if !deps.contains(item) && item != component_name {
          deps.push(item.clone());
        }
      }

      // search(visited, deps, &dep, component_names);
    }
  };
  search(&mut visited, &mut deps, component_name, component_names);

  deps
}

pub fn run(user_config: &options::UserConfig) {
  let components_names = common::get_components_names(user_config);
  if components_names.is_empty() {
    println!("没有需要编译的组件");
    return;
  }

  // 先获取组件的样式依赖关系，就是当前组件依赖了哪些组件
  let mut style_deps_map: HashMap<&String, Vec<String>> = HashMap::new();
  let mut deps_map: HashMap<String, Vec<String>> = HashMap::new();

  for component_name in &components_names {
    let component_style_deps =
      get_component_style_deps(&component_name, &components_names, &mut deps_map);
    style_deps_map.insert(component_name, component_style_deps);
  }
}
