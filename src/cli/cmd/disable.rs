use std::process;

use clap::ArgMatches;
use colored::*;
use exit_code::DATA_ERROR;

use crate::cli::cmd::{get_session_or_exit, write_session_file_or_exit};
use crate::core::{get_desktop_text, get_sessions, SesOption};

pub fn disable(xsession_dir: &str, options: Option<&ArgMatches>) {
  let session = get_session_or_exit(xsession_dir, options);
  let use_journal = !options.unwrap().is_present("no-journald");

  if !session.is_active() {
    eprintln!("{} Session with key '{}' is disabled. Nothing to be done.",
              "Error:".red(), session.key().green());
  }

  // Check if last active session
  let all_valid = get_sessions(&xsession_dir, SesOption::Valid);
  let active = all_valid.iter().filter(|&s| s.is_active()).count();
  if active <= 1 {
    let session_key = options.unwrap().value_of("session_key").unwrap();
    eprintln!("{} the session '{}' is the last active one. Can't do that sorry.",
              "Error:".red(), session_key);
    process::exit(DATA_ERROR);
  }

  let text = get_desktop_text(&session, false);
  write_session_file_or_exit(&session, text);

  use systemd::journal;
  if use_journal {
    journal::print(1, &format!("Session '{}' is disabled.", session.name().unwrap_or_default()));
  }
}




