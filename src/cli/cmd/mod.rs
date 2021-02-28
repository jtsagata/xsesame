use std::fs::File;
use std::io::Write;
use std::process;

use clap::ArgMatches;
use colored::*;
use exit_code::{CANNOT_CREATE, DATA_ERROR, NO_PERMISSION};

pub use completion::completion as completion;
pub use disable::disable as disable;
pub use enable::enable as enable;
pub use export::export as export;
pub use list::list as list;
pub use toggle::toggle as toggle;

use crate::core::{get_sessions, SesOption, SessionInfo};

mod export;
mod completion;
mod disable;
mod enable;
mod list;
mod toggle;

const ASCII_STATES: [&str; 3] = ["+", "-", "x"];
const EMOJI_HEARTS: [&str; 3] = ["💚", "🤍", "💔"];
const EMOJI_STATES: [&str; 3] = [figures_rs::TICK, figures_rs::CROSS, figures_rs::WARNING];

fn get_session_or_exit(xsession_dir: &str, options: Option<&ArgMatches>) -> SessionInfo {
  let options = options.unwrap();
  let session_key = options.value_of("session_key").unwrap();

  let sessions = get_sessions(&xsession_dir, SesOption::Valid);
  let found = sessions.iter().find(|s| s.key() == session_key);

  if found.is_none() {
    eprintln!("{} There is no valid session with key '{}' in '{}.", "Error:".red(), session_key.green(), xsession_dir.green());
    process::exit(DATA_ERROR);
  }

  let session = found.unwrap();
  if session.is_valid().is_err() {
    let error = session.is_valid().err().unwrap();
    eprintln!("{} The session with key '{}' is invalid, can't enable", "Error:".red(), session_key);
    eprintln!("  Reason: {}", error.message());
    eprintln!("  Fix file  '{}' first", session.filename());
    process::exit(DATA_ERROR);
  }

  session.clone()
}


fn write_session_file_or_exit(session: &SessionInfo, text: String) {
  let file = File::create(session.filename());
  if file.is_err() {
    eprintln!("{} Can't create file '{}': {}", "Error:".red(),
              session.filename().green(),
              file.err().unwrap()
    );
    process::exit(CANNOT_CREATE);
  }

  let mut file = file.unwrap();
  let done = file.write_all(text.as_bytes());
  if done.is_err() {
    eprintln!("{} Can't write to file '{}': {}", "Error:".red(),
              session.filename().green(),
              done.err().unwrap()
    );
    process::exit(NO_PERMISSION);
  }
}

fn parse_what_opt(options: &ArgMatches) -> SesOption {
  match options.value_of("what").unwrap() {
    "all" => { SesOption::All }
    "valid" => { SesOption::Valid }
    "invalid" => { SesOption::Invalid }
    &_ => { SesOption::Valid }
  }
}

