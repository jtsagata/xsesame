/// The default place of session data
#[cfg(debug_assertions)]
pub const XSESSION_DIR: &str = "test/samples";
#[cfg(not(debug_assertions))]
pub const XSESSION_DIR: &str = "/usr/share/xsessions";

/// Get the default session dir
pub fn get_default_session_dir() -> &'static str {
  XSESSION_DIR
}

pub fn generate_path_key(f: &str) -> String {
  use std::path::Path;

  let p = Path::new(f).with_extension("");
  String::from(p.file_name().unwrap().to_str().unwrap()).to_lowercase()
}


