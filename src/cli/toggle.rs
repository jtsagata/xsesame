use crate::core::{get_sessions, SesOption};

// This will drive and display the errors
pub fn cmd_toggle(path_key: &str, xsession_dir: &str) {
  println!("Request toggle on {} at {}", path_key, xsession_dir);
  toggle_session(path_key, xsession_dir);
  // Display errors
  // Then implement enable/disable using toggle
}


fn toggle_session(path_key: &str, xsession_dir: &str) {
  println!("Doing toggle on {} at {}", path_key, xsession_dir);

  let session = get_sessions(xsession_dir, SesOption::All);

  // get SessionInfo (if exist)
  // check if valid
  // if enable proceed
  // if disable
  //     Check if it is the last one
  // read desktop file
  // Change key
  // Write back
  // Check if it is written
  // Ok(())
}
