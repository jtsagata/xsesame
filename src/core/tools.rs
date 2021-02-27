use std::path::Path;

/// Helper to get the executable name without the path
pub fn program_name() -> Option<String> {
  use std::ffi::OsStr;

  std::env::current_exe().ok()
    .as_ref()
    .map(Path::new)
    .and_then(Path::file_name)
    .and_then(OsStr::to_str)
    .map(String::from)
}

//// Helper to get a PathBuf pointing to the session file
// pub fn get_filename_from_key(xsession_dir: &str, key: &str) -> Result<PathBuf, String> {
//   let sessions = get_sessions(xsession_dir);
//
//   // Session key must exist
//   if !sessions.contains_key(key) {
//     return Err(format!("'{}' is not a valid session key", key.green()));
//   }
//
//   // File must exist
//   let file_name = sessions.get(key).unwrap().path();
//   let orig = Path::new(&file_name).to_owned();
//   if !orig.exists() {
//     return Err(format!("File name {}' does not exist", file_name.green()));
//   }
//
//   Ok(orig)
// }
