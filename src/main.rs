use std::{io, process};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use clap::{ArgMatches, Shell};
use colored::*;
use nix::unistd::Uid;
use stybulate::{Cell, Headers, Style, Table};

use desktop_info::DesktopInfo;

mod desktop_info;
mod opts;

#[cfg(target_os = "linux")]
fn main() {
  let matches = opts::build_cli().get_matches();

  // Sessions directory
  let xsession_dir = matches.value_of("session-dir").unwrap_or(opts::XSESSION_DIR);
  if !Path::new(xsession_dir).is_dir() {
    println!("{} '{}' is not a directory", "Error:".red(), xsession_dir.green());
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
    cmd_enable_disable(xsession_dir, matches, &"desktop");
    sub_command = true;
  }


  if let Some(matches) = matches.subcommand_matches("completion") {
    cmd_completion(matches);
    sub_command = true;
  }

  // If no subcommand is given rerun with list option
  if !sub_command {
    cmd_rerun_with_list_cmd()
  }
}

fn cmd_enable_disable(xsession_dir: &str, matches: &ArgMatches, ext: &str) {
  let key = matches.value_of("session_key").unwrap();
  let file = get_filename_from_key(&xsession_dir, &key);

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
  }

  let done = std::fs::rename(&orig_name, &new_name);
  if done.is_ok() {
    println!("{} '{}' -> '{}'", "Done:".green(), orig_name.display().to_string().green(), new_name.display().to_string().green());
  } else {
    eprintln!("{}  Can't rename '{}'", "Error:".red(), orig_name.display().to_string().green());
    if !Uid::effective().is_root() {
      eprintln!("You must run this executable with root permissions.");
    }
  }
}

fn cmd_list_sessions(xsession_dir: &str, matches: &ArgMatches) {
  let style = matches.value_of("style").unwrap_or("Fancy");
  let style = match style {
    "Fancy" => { Style::Fancy }
    "Grid" => { Style::Grid }
    "Simple" => { Style::Simple }
    &_ => { Style::Plain }
  };
  print_sessions(xsession_dir, style);
}

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

fn cmd_rerun_with_list_cmd() {
  use exec::Error;
  use std::ffi::OsStr;

  let exe = std::env::current_exe().unwrap().display().to_string();
  let err = exec::Command::new(OsStr::new(&exe)).arg("list").exec();
  match err {
    Error::BadArgument(_) => {}
    Error::Errno(errno) => {
      eprintln!("{} {}", "Error:".red(), err);
      process::exit(errno.0);
    }
  }
}

fn print_sessions(xsession_dir: &str, style: stybulate::Style) {
  let sessions = get_sessions(&xsession_dir);

  let mut elements: Vec<Vec<Cell>> = Vec::new();
  for (_, el) in sessions {
    // let active_str = if el.is_active() { " ✓ " } else { " ✗ "};
    let active_str = if el.is_active() { "+" } else { "-" };
    let key = format!("{} {}", active_str, el.path_key());
    elements.push(vec![Cell::from(key.as_str()), Cell::from(el.name().as_str()), Cell::from(el.comment().as_str())]);
  }

  let table = Table::new(
    style, elements,
    Some(Headers::from(vec!["Key", "Name", "Comment"])),
  ).tabulate();

  println!("List of active and inactive sessions:");
  println!();
  println!("{}", table);
  println!();
  println!("To enable/disable a session run: {} {}", program_name().unwrap().green(), "enable|disable <key>".green());
  println!();
}

fn get_sessions(xsession_dir: &str) -> HashMap<String, DesktopInfo> {
  let mut sessions = HashMap::<String, DesktopInfo>::new();

  if DesktopInfo::collect_sessions(&mut sessions, &xsession_dir).is_err() {
    println!("{} Unable to parse sessions", "Error:".red());
    process::exit(-1);
  }

  sessions
}

fn get_filename_from_key(xsession_dir: &str, key: &str) -> Result<PathBuf, String> {
  let sessions = get_sessions(xsession_dir);

  // Session key must exist
  if !sessions.contains_key(key) {
    return Err(format!("'{}' is not a valid session key", key.green()));
  }

  // File must exist
  let file_name = sessions.get(key).unwrap().path();
  let orig = Path::new(&file_name).to_owned();
  if !orig.exists() {
    return Err(format!("File name {}' does not exist", file_name.green()));
  }

  Ok(orig)
}

fn program_name() -> Option<String> {
  use std::ffi::OsStr;

  std::env::current_exe().ok()
    .as_ref()
    .map(Path::new)
    .and_then(Path::file_name)
    .and_then(OsStr::to_str)
    .map(String::from)
}


#[cfg(not(target_os = "linux"))]
pub fn main() {
  println!("Not supported on this system");
}
