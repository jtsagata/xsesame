use clap::ArgMatches;
use colored::*;

use crate::cli::cmd::{get_session_or_exit, write_session_file_or_exit};
use crate::core::get_desktop_text;

pub fn enable(xsession_dir: &str, options: Option<&ArgMatches>) {
  let session = get_session_or_exit(xsession_dir, options);
  let use_journal = !options.unwrap().is_present("no-journald");

  if session.is_active() {
    eprintln!("{} Session with key '{}' is enabled. Nothing to be done",
              "Error:".red(), session.key().green());
  }

  let text = get_desktop_text(&session, true);
  write_session_file_or_exit(&session, text);

  use systemd::journal;
  if use_journal {
    journal::print(1, &format!("Session '{}' is enabled.", session.name().unwrap_or_default()));
  }
}
