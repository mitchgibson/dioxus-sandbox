use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

pub fn get_all_file_paths(dir: &str) -> Vec<PathBuf> {
  let mut file_paths = Vec::new();
  
  if let Ok(entries) = fs::read_dir(dir) {
      for entry in entries {
          if let Ok(entry) = entry {
              let path = entry.path();
              if path.is_dir() {
                  // Recursively get files from subdirectories
                  file_paths.extend(get_all_file_paths(path.to_str().unwrap()));
              } else if path.extension().map_or(false, |ext| ext == "vue") {
                  // Only include files with .vue extension
                  file_paths.push(path);
              }
          }
      }
  }
  
  file_paths
}

// Example usage
pub fn print_all_file_paths() {
    let dir = "/Users/mitchdelachevrotiere/dev/knak/packages/builder/src";
    let file_paths = get_all_file_paths(dir);
    
    for path in file_paths {
        println!("{}", path.display());
    }
}

pub fn crawl(dir: &str) -> HashMap<String, Vec<ComponentCount>> {
    println!("Crawling directory: {:?}", dir);
    let file_paths = get_all_file_paths(dir);
    let mut results: HashMap<String, Vec<ComponentCount>> = HashMap::new();

    for file_path in file_paths {
        let content = fs::read_to_string(&file_path).expect("Unable to read file");
        let template_block = extract_template_block(&content);

        let mut component_counts: HashMap<String, usize> = HashMap::new();

        for component in extract_components(&template_block) {
            *component_counts.entry(component).or_insert(0) += 1;
        }

        let result: Vec<ComponentCount> = component_counts.into_iter()
            .map(|(name, count)| ComponentCount { name, count })
            .collect();

        // println!("File: {:?}, Components: {:?}", file_path, result);
        results.insert(file_path.display().to_string(), result);
    }

    results
}

fn extract_template_block(content: &str) -> String {
    // template block should look like this in the file <template>...</template>
    let start_tag = "<template>";
    let end_tag = "</template>";

    // if no template block is found, return an empty string
    if !content.contains(start_tag) || !content.contains(end_tag) {
        return String::new();
    }
    let start_index = content.find(start_tag).expect("Template block not found") + start_tag.len();
    let end_index = content.find(end_tag).expect("Template block not found");

    content[start_index..end_index].to_string()
}

fn extract_components(template_block: &str) -> Vec<String> {
    // components are any kebab-case strings that are preceded by '<' and followed by '>'
    // should only include component name, not any attributes
    let mut components = Vec::new();
    let mut start_index = 0;

    while let Some(start) = template_block[start_index..].find('<') {
        let start = start_index + start;
        let end = start + template_block[start..].find('>').expect("Component tag not closed");

        let component = &template_block[start + 1..end];
        let component = component.split_whitespace().next().expect("Component name not found");

        if component.starts_with("!") {
            start_index = end;
            continue;
        }

        components.push(component.to_string());
        start_index = end;
    }

    components
}

#[derive(Debug)]
pub struct ComponentCount {
    pub name: String,
    pub count: usize,
}