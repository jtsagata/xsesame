use std::process;

use clap::ArgMatches;
use colored::*;
use nix::unistd::Uid;

use crate::core;

/// This enables or disable a session given a key
pub fn cmd_enable_disable(xsession_dir: &str, matches: &ArgMatches, ext: &str) {
  let key = matches.value_of("session_key").unwrap();

  crate::core::get_default_session_dir();

  let file = core::get_filename_from_key(&xsession_dir, &key);
  let use_journal = !matches.is_present("no-journald");

  match file {
    Ok(_) => {}
    Err(err) => {
      eprintln!("{}: {}", "Error".red(), err);
      process::exit(-1);
    }
  }

  // File rename
  let file = file.unwrap();
  let orig_name = file.clone();
  let new_name = file.with_extension(ext);

  if orig_name == new_name {
    let state = if ext == "desktop" { "enabled" } else { "disabled" };
    eprintln!("Nothing to be done, '{}' is {}.", key.green(), state.green());
    return;
  }

  use systemd::journal;
  let done = std::fs::rename(&orig_name, &new_name);
  if done.is_ok() {
    println!("{} '{}' -> '{}'", "Done:".green(), orig_name.display().to_string().green(), new_name.display().to_string().green());
    if use_journal {
      journal::print(1, &format!("Rename file: '{}' ->  '{}'", orig_name.display(), new_name.display()));
    }
  } else {
    if use_journal {
      journal::print(1, &format!("Can't rename  file: '{}'", orig_name.display()));
    }
    eprintln!("{}  Can't rename '{}'", "Error:".red(), orig_name.display().to_string().green());
    if !Uid::effective().is_root() {
      eprintln!("You must run this executable with root permissions.");
    }
  }
}
