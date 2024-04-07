use std::collections::HashSet;

use std::collections::HashMap;
use std::fs;

fn get_deps(file_path: &str, deps_map: &mut HashMap<String, Vec<String>>) -> Vec<String> {
  if let Some(deps) = deps_map.get(file_path) {
    return deps.clone();
  }

  let code = fs::read_to_string(file_path).expect("Failed to read file");
  let imports: Vec<String> = match_imports(&code);
  let paths: Vec<String> = imports
    .iter()
    .filter_map(|item| get_path_by_import(item, file_path))
    .collect();

  deps_map.insert(file_path.to_string(), paths.clone());

  for path in &paths {
    get_deps(path, deps_map);
  }

  paths
}

fn analyze_component_deps(components: Vec<String>, component: &str) -> Vec<String> {
  let mut deps: Vec<String> = Vec::new();
  let mut visited: HashSet<String> = HashSet::new();

  fn search(
    visited: &mut HashSet<String>,
    deps: &mut Vec<String>,
    components: &Vec<String>,
    component: &str,
    key: &str,
  ) {
    visited.insert(key.to_string());

    for dep in get_deps(key) {
      if visited.contains(dep) {
        continue;
      }

      for item in components.iter().filter(|&item| match_path(&dep, item)) {
        if !deps.contains(item) && item != component {
          deps.push(item.clone());
        }
      }
      search(visited, deps, components, component, &dep);
    }
  }

  let component_entry = fill_ext(&join(SRC_DIR, component, "index"));
  search(
    &mut visited,
    &mut deps,
    &components,
    component,
    &component_entry,
  );

  deps
    .into_iter()
    .filter(|item| check_style_exists(item))
    .collect()
}
