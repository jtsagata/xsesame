use clap::ArgMatches;

use crate::cli::cmd::{get_session_or_exit, write_session_file_or_exit};
use crate::core::get_desktop_text;

pub fn toggle(xsession_dir: &str, options: Option<&ArgMatches>) {
  let session = get_session_or_exit(xsession_dir, options);
  let use_journal = !options.unwrap().is_present("no-journald");

  let new_state = match session.is_active() {
    true => { false }
    false => { true }
  };

  let text = get_desktop_text(&session, new_state);
  write_session_file_or_exit(&session, text);

  use systemd::journal;
  if use_journal {
    match new_state {
      true => {
        journal::print(1, &format!("Session '{}' is enabled.", session.name().unwrap_or_default()));
      }
      false => {
        journal::print(1, &format!("Session '{}' is disabled.", session.name().unwrap_or_default()));
      }
    }
  }
}
