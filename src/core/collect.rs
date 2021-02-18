use std::{io, process};
use std::collections::BTreeMap;
use std::fs::{DirEntry, read_dir};
use std::path::Path;

use colored::*;
use freedesktop_entry_parser::parse_entry;

use crate::core::SessionInfo;

/// Get all sessions in directory
pub fn get_sessions(xsession_dir: &str) -> BTreeMap<String, SessionInfo> {
  let mut sessions = BTreeMap::<String, SessionInfo>::new();

  if collect_sessions(&mut sessions, &xsession_dir).is_err() {
    eprintln!("{} Unable to parse sessions", "Error:".red());
    process::exit(-1);
  }

  sessions
}

/// Collect and return all sessions
fn collect_sessions(sessions: &mut BTreeMap<String, SessionInfo>, xsession_dir: &str) -> io::Result<()> {
  let mut file_paths = read_dir(xsession_dir)?
    .filter(|entry| is_desktop_file(entry))
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  file_paths.sort();

  for path in file_paths {
    let file_path = path.display().to_string();
    let desktop_entry = parse_entry(path);

    match desktop_entry {
      Ok(desktop_entry) => {
        let desktop = SessionInfo::new(file_path, desktop_entry);
        sessions.entry(desktop.path_key()).or_insert(desktop);
      }
      // The parser generates an error
      Err(_) => {
        let just_the_file = Path::new(&file_path).file_name().unwrap().to_string_lossy();
        eprintln!("{} Unable to parse '{}'.", "Warning:".yellow(), just_the_file.green());
      }
    }
  }

  Ok(())
}

/// Check if something looks like a desktop file
fn is_desktop_file(entry: &Result<DirEntry, std::io::Error>) -> bool {
  use regex::Regex;
  let re = Regex::new(r"^.*\.desktop.*$").unwrap();

  match entry {
    Ok(entry) => {
      let file_name = entry.file_name();
      let s = file_name.to_str().unwrap_or("");
      re.is_match(s)
    }
    Err(_) => { false }
  }
}
