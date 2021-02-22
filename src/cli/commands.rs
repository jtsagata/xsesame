use crate::core::{get_sessions, SesOption};

/// Export sessions as json
pub fn cmd_export_json(xsession_dir: &str) -> String {
  let sessions = get_sessions(&xsession_dir, SesOption::All);
  serde_json::to_string(&sessions).unwrap()
}
