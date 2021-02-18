//! xsessame
//! Some desktop environments install a lot of different types that have to real use for
//! the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
//! This small utility helps you to list and disable some of them. Of course you can also re-enable them.
//!
//! The propose of this little tool is to minimize the clutter in the display manager.
//!

use std::path::Path;
use std::process;

use colored::*;

use crate::cli::{cmd_completion, cmd_enable_disable, cmd_export_json, cmd_list_sessions, cmd_rerun_with_list_cmd};
use crate::core::get_sessions;

mod opts;
mod core;
mod cli;


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
    cmd_list_sessions(xsession_dir, matches);
    sub_command = true;
  }

  if let Some(matches) = matches.subcommand_matches("enable") {
    cmd_enable_disable(xsession_dir, matches, &"desktop");
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
      cmd_enable_disable(xsession_dir, matches, &"desktop-disable");
    }
    sub_command = true;
  }

  if let Some(matches) = matches.subcommand_matches("completion") {
    cmd_completion(matches);
    sub_command = true;
  }

  if let Some(_matches) = matches.subcommand_matches("export") {
    let json = cmd_export_json(xsession_dir);
    println!("{}", json);
    sub_command = true;
  }

  // If no subcommand is given rerun with list option
  if !sub_command {
    cmd_rerun_with_list_cmd(xsession_dir)
  }
}

#[cfg(not(target_os = "linux"))]
pub fn main() {
  println!("Not supported on this system");
}