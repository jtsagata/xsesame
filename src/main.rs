mod desktop_info;

use std::io;
use crate::desktop_info::{DesktopInfo};
use std::collections::HashMap;
use stybulate::{Table, Style, Headers, Cell};
use std::path::Path;
use std::ffi::OsStr;
use colored::*;

// TODO: From cargo? or from command line arguments :-) -- solved
// const XSESSION_DIR: &str = "/usr/share/xsessions/";
const XSESSION_DIR: &str = "/home/talos/CLionProjects/Rust/sesman/test/samples";

#[cfg(target_os = "linux")]
fn main() -> io::Result<()> {
  cmd_list_sessions();
  Ok(())
}

fn cmd_list_sessions() {
  let mut sessions = HashMap::<String, DesktopInfo>::new();

  if DesktopInfo::collect_sessions(&mut sessions, XSESSION_DIR).is_err() {
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
    Style::Fancy,
    elements,
    Some(Headers::from(vec!["Key", "Name", "Comment"])),
  ).tabulate();

  println!("List of active and inactive sessions:");
  println!("{}", table);
  println!("To enable/disable a session run: {} {}", program_name().unwrap().green(), "enable|disable <key>".green() );
  println!();
}

fn program_name() -> Option<String>  {
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
