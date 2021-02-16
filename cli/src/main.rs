//! xsessame
//! Some desktop environments install a lot of different types that have to real use for
//! the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
//! This small utility helps you to list and disable some of them. Of course you can also re-enable them.
//!
//! The propose of this little tool is to minimize the clutter in the display manager.
//!

use std::{io, process};
use std::path::Path;

use clap::{ArgMatches, Shell};
use colored::*;

use xsesame_core::get_sessions;

mod opts;
mod list_sessions;
mod enable_disable;
mod tools;

#[cfg(target_os = "linux")]
fn main() {
  let matches = opts::build_cli().get_matches();

  // Sessions directory
  let xsession_dir = matches.value_of("session-dir").unwrap();
  if !Path::new(xsession_dir).is_dir() {
    eprintln!("{} '{}' is not a directory", "Error:".red(), xsession_dir.green());
    process::exit(-1);
  }

  let mut sub_command: bool = false;

  if let Some(matches) = matches.subcommand_matches("list") {
    list_sessions::cmd_list_sessions(xsession_dir, matches);
    sub_command = true;
  }

  if let Some(matches) = matches.subcommand_matches("enable") {
    enable_disable::cmd_enable_disable(xsession_dir, matches, &"desktop");
    sub_command = true;
  }

  if let Some(matches) = matches.subcommand_matches("disable") {

    // check if there is at least 2 enabled sessions
    let active_sessions = get_sessions(&xsession_dir)
      .values()
      .filter(|el| el.is_active())
      .count();

    if active_sessions < 2 {
      eprintln!("{}", "There is only one active session! Nothing to be done!".yellow());
    } else {
      enable_disable::cmd_enable_disable(xsession_dir, matches, &"desktop-disable");
    }
    sub_command = true;
  }

  if let Some(matches) = matches.subcommand_matches("completion") {
    cmd_completion(matches);
    sub_command = true;
  }

  if let Some(_matches) = matches.subcommand_matches("export") {
    let json = export_json(xsession_dir);
    println!("{}", json);
    sub_command = true;
  }

  // If no subcommand is given rerun with list option
  if !sub_command {
    cmd_rerun_with_list_cmd(xsession_dir)
  }
}


/// Command line completion generation
fn cmd_completion(matches: &ArgMatches) {
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
fn cmd_rerun_with_list_cmd(xsession_dir: &str) {
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
pub fn export_json(xsession_dir: &str) -> String {
  let sessions = get_sessions(&xsession_dir);
  serde_json::to_string(&sessions).unwrap()
}


#[cfg(not(target_os = "linux"))]
pub fn main() {
  println!("Not supported on this system");
}
