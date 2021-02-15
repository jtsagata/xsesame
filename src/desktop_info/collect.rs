use std::collections::BTreeMap;
use std::fs::{DirEntry, read_dir};
use std::io;
use std::path::Path;

use colored::*;
use freedesktop_entry_parser::parse_entry;

use crate::desktop_info::DesktopInfo;

/// Collect and return all sessions
pub fn collect_sessions(sessions: &mut BTreeMap<String, DesktopInfo>, xsession_dir: &str) -> io::Result<()> {
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
        let desktop = DesktopInfo::new(file_path, desktop_entry);
        match desktop {
          Ok(desktop) => {
            // Everything is ok add it to session list
            sessions.entry(desktop.path_key()).or_insert(desktop);
          }
          Err(error) => {
            // The parsing was ok but some key elements is missing
            println!("{} {}", "Warning:".yellow(), error)
          }
        }
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
