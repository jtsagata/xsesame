use freedesktop_entry_parser::Entry;
use std::path::Path;
use std::collections::HashMap;
use std::{io, fs};
use freedesktop_entry_parser::parse_entry;
use std::ffi::OsStr;

pub struct DesktopInfo {
  path: String,
  desktop: freedesktop_entry_parser::Entry,
}

impl DesktopInfo {
  pub fn new(path: String, desktop: Entry) -> Self {
    DesktopInfo { path, desktop }
  }

  pub fn path(&self) -> &str {
    &self.path
  }
  pub fn desktop(&self) -> &Entry {
    &self.desktop
  }

  pub fn name(&self) -> &str {
    let name = self.desktop.section("Desktop Entry").attr("Name");
    Option::or(name, Some("<None>")).unwrap()
  }

  pub fn active_str(&self) -> &str {
    let path = Path::new(self.path());
    let ext = path.extension().unwrap();
    if ext == "desktop" { "(active)" } else { "(inactive)" }
  }


  pub fn collect_sessions(sessions: &mut HashMap<String, DesktopInfo>, xsession_dir: &str) -> io::Result<()> {
    let mut entries = fs::read_dir(xsession_dir)?
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

    entries.sort();

    for entry in entries {
      let file_path = entry.to_str().unwrap().to_string();
      let base_file = entry.with_extension("").to_str().unwrap_or_default().to_owned();
      let key = base_file.replace(xsession_dir, "");

      let desktop_file = parse_entry(entry)?;
      let desktop = DesktopInfo::new(file_path, desktop_file);
      println!("{} K=> {}", key, desktop.name());

      sessions.entry(key).or_insert(desktop);
    }

    Ok(())
  }
}
