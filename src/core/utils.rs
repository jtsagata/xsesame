use std::path::Path;

pub fn generate_path_key(f: &str) -> String {
  let p = Path::new(f).with_extension("");
  String::from(p.file_name().unwrap().to_str().unwrap()).to_lowercase()
}
