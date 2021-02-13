mod desktop_info;
mod opts;

// use crate::desktop_info::DesktopInfo;
use stybulate::{Table, Style, Headers, Cell};
use desktop_info::DesktopInfo;
use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;
use clap::{Shell};
use colored::*;
use std::io;

#[cfg(target_os = "linux")]
fn main() {
  let matches = opts::build_cli().get_matches();

  // Sessions directory
  let xsession_dir = matches.value_of("session-dir").unwrap_or(opts::XSESSION_DIR);
  if !Path::new(xsession_dir).is_dir() {
    //TODO: Add better error handling and error codes
    panic!(format!("{}: is not a directory", xsession_dir));
  }

  if let Some(matches) = matches.subcommand_matches("list") {
    let style = matches.value_of("style").unwrap_or("Fancy");
    let style = match style {
      "Fancy" => { Style::Fancy }
      "Grid" => { Style::Grid }
      "Simple" => { Style::Simple }
      &_ => { Style::Plain }
    };
    cmd_list_sessions(xsession_dir, style);
  }

  if let Some(matches) = matches.subcommand_matches("completion") {
    let shell = matches.value_of("shell").unwrap_or("bash");
    let shell = match shell
    {
      "zsh" => { Shell::Zsh }
      "fish" => { Shell::Fish }
      "elvish" => { Shell::Elvish }
      &_ => { Shell::Bash }
    };
    opts::build_cli().gen_completions_to("xsesame", shell, &mut io::stdout());
  }
}

fn cmd_list_sessions(xsession_dir: &str, style: stybulate::Style) {
  let mut sessions = HashMap::<String, DesktopInfo>::new();

  if DesktopInfo::collect_sessions(&mut sessions, xsession_dir).is_err() {
    panic!("Unable to parse sessions");
  }

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

fn program_name() -> Option<String> {
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
