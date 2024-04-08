// 没写完，写到一半发现不需要这个函数
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
  // 定义正则表达式匹配 import 语句
  let import_re = Regex::new(
    r#"import\s+?(?:(?:(?:[\w*\s{},]*)\s+from(\s+)?)|)(?:(?:".*?")|(?:'.*?'))[\s]*?(?:;|$|)"#,
  )
  .unwrap();
  // 从代码中提取 import 语句
  let imports: Vec<&str> = import_re.find_iter(code).map(|m| m.as_str()).collect();
  // 过滤掉包含 'import type' 的行
  imports
    .into_iter()
    .filter(|line| !line.contains("import type"))
    .map(String::from)
    .collect()
}

// 根据import 的代码内容获取路径
fn get_path_by_import(code: &str, path: &PathBuf) -> Option<String> {
  let divider = if code.contains("\"") { "\"" } else { "'" };
  let relative_path = code.split(divider).nth(1).unwrap();

  if relative_path.contains(".") {
    let mut new_path = path.clone();
    // 比如./就是同级的，所以路径得从父级拼接
    new_path.pop();

    let mut full_path = PathBuf::from(new_path);

    if constant::SCRIPT_EXTS.iter().any(|ext| {
      // 拼接完整的文件名
      let file_name = format!(
        "{}{}",
        PathBuf::from(relative_path)
          .file_name()
          .unwrap()
          .to_str()
          .unwrap(),
        ext
      );
      // 使用 join 方法拼接路径
      if full_path.join(&file_name).exists() {
        full_path = full_path.join(&file_name);
        return true;
      } else {
        return false;
      }
    }) {
      Some(full_path.to_string_lossy().to_string())
    } else {
      return None;
    }
  } else {
    None
  }
}
// 获取单个组件的依赖关系
fn get_deps(file_path: PathBuf, deps_map: &mut HashMap<String, Vec<String>>) -> Vec<String> {
  let code = fs::read_to_string(&file_path).unwrap();
  let imports: Vec<String> = match_imports(&code);

  println!("get_deps:imports,{:?}", imports);

  let result: Vec<String> = imports
    .iter()
    .filter_map(|item| get_path_by_import(item, &file_path))
    .collect();

  deps_map.insert(file_path.to_string_lossy().to_string(), result.clone());

  println!("get_deps:result,{:?}", result);
  for dep in result.iter() {
    println!("get_deps:dep,{}", dep);
    get_deps(PathBuf::from(dep), deps_map);
  }

  // println!("get_deps:deps_map,{:?}", deps_map);
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

    for dep in get_deps(file_path, deps_map) {
      if visited.contains(&dep) {
        continue;
      }

      // 如果依赖的组件是组件库自身的组件，则加入到依赖文件
      for item in component_names
        .iter()
        .filter(|&name| match_path(&dep, name))
      {
        // 没缓存过，且组件不是当前组件
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
