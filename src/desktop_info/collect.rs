use std::collections::BTreeMap;
use std::fs::read_dir;
use std::io;
use std::path::Path;

use colored::*;
use freedesktop_entry_parser::parse_entry;

use crate::desktop_info::DesktopInfo;

/// Collect and return all sessions
pub fn collect_sessions(sessions: &mut BTreeMap<String, DesktopInfo>, xsession_dir: &str) -> io::Result<()> {
  let mut file_paths = read_dir(xsession_dir)?
    .filter(|entry| {
      match entry {
        Ok(e) => {
          let local = e.path();
          let ext = local.extension().unwrap_or_default();
          ext == "desktop" || ext == "desktop-disable"
        }
        Err(_) => { false }
      }
    })
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  file_paths.sort();

  for path in file_paths {
    let file_path = path.display().to_string();
    let base_path = path.with_extension("").display().to_string();
    let map_key = base_path.replace(xsession_dir, "").replace("/", "");

    let desktop_entry = parse_entry(path);
    match desktop_entry {
      Ok(desktop_entry) => {
        let desktop = DesktopInfo::new(file_path, desktop_entry);
        match desktop {
          Ok(desktop) => {
            sessions.entry(map_key).or_insert(desktop);
          }
          Err(error) => {
            println!("{} {}", "Warning:".yellow(), error)
          }
        }
      }
      Err(_) => {
        let just_the_file = Path::new(&file_path).file_name().unwrap().to_string_lossy();
        eprintln!("{} Unable to parse '{}'.", "Warning:".yellow(), just_the_file.green());
      }
    }
  }

  Ok(())
}
