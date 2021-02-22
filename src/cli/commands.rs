use std::{io, process};

use clap::{ArgMatches, Shell};
use colored::Colorize;

use crate::core::{get_sessions, SesOption};
use crate::opts;

/// Command line completion generation
pub fn cmd_completion(matches: &ArgMatches) {
  let shell = matches.value_of("shell").unwrap_or("bash");
  let shell = match shell {
    "zsh" => { Shell::Zsh }
    "fish" => { Shell::Fish }
    "elvish" => { Shell::Elvish }
    &_ => { Shell::Bash }
  };
  opts::build_cli().gen_completions_to("xsesame", shell, &mut io::stdout());
}

/// Rerun current executable using list subcommand
///
/// As clap crate can't support a default subcommand yet, we hack it with execve()
pub fn cmd_rerun_with_list_cmd(xsession_dir: &str) {
  use exec::Error;
  use std::ffi::OsStr;

  let exe = std::env::current_exe().unwrap().display().to_string();
  let err = exec::Command::new(OsStr::new(&exe))
    .arg("--session-dir")
    .arg(xsession_dir)
    .arg("list").exec();
  match err {
    Error::BadArgument(_) => {}
    Error::Errno(errno) => {
      eprintln!("{} {}", "Error:".red(), err);
      process::exit(errno.0);
    }
  }
}


/// Export sessions as json
pub fn cmd_export_json(xsession_dir: &str) -> String {
  let sessions = get_sessions(&xsession_dir, SesOption::All);
  serde_json::to_string(&sessions).unwrap()
}
