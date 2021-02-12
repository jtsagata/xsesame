use freedesktop_entry_parser::Entry;
use std::path::Path;
use std::collections::HashMap;
use std::{io, fs};
use freedesktop_entry_parser::parse_entry;

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

  pub fn name(&self) -> &str {
    self.get_attribute("Name")
  }

  pub fn icon(&self) -> &str {
    self.get_attribute("Icon")
  }

  pub fn comment(&self) -> &str {
    // TODO: Localized
    self.get_attribute("Comment")
  }

  fn get_attribute(&self, attr: &str) -> &str {
    let comment = self.desktop.section("Desktop Entry").attr(attr);
    Option::or(comment, Some("")).unwrap()
  }


  pub fn active_str(&self) -> &str {
    let path = Path::new(self.path());
    let ext = path.extension().unwrap();
    if ext == "desktop" { "(active)" } else { "(inactive)" }
  }


  pub fn collect_sessions(sessions: &mut HashMap<String, DesktopInfo>, xsession_dir: &str) -> io::Result<()> {
    let mut file_paths = fs::read_dir(xsession_dir)?
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
      let map_key = base_path.replace(xsession_dir, "");

      // TODO: Demo run a parsing failure case
      let desktop_entry = parse_entry(path)?;
      let desktop = DesktopInfo::new(file_path, desktop_entry);

      println!("{}: [{}] '{}' {} -- {}", map_key, desktop.icon(), desktop.name(), desktop.active_str(), desktop.comment());

      sessions.entry(map_key).or_insert(desktop);
    }

    Ok(())
  }
}
