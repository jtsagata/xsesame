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
  pub fn new(path: String, desktop: Entry) -> Result<Self,String> {
    let path_copy = path.clone();
    let entry = DesktopInfo{ path, desktop };

    // Test Type="Application"
    let d_type = entry.get_attribute("Type");
    match d_type {
      None => {
        return Err(format!("The file {} does not specify a type", path_copy));
      }
      Some(d_type) => if d_type != "Application" {
        return Err(format!("The file {} specify type as '{}'", path_copy, d_type));
      },
    }

    // Test if provides a Name
    if entry.get_attribute("Name").is_none() {
      return Err(format!("The file {} does not specify a name", path_copy));
    }

    Ok(entry)
  }

  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn name(&self) -> &str {
    self.get_attribute_str("Name")
  }

  pub fn icon(&self) -> &str {
    self.get_attribute_str("Icon")
  }

  pub fn comment(&self) -> &str {
    // TODO: Localized
    self.get_attribute_str("Comment")
  }

  pub fn get_attribute(&self, attr: &str) -> Option<&str>{
    return self.desktop.section("Desktop Entry").attr(attr);
  }

  pub fn get_attribute_str(&self, attr: &str) -> &str {
    Option::or(self.get_attribute(&attr), Some("")).unwrap()
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

      let desktop_entry = parse_entry(path);
      match desktop_entry {
        Ok(desktop_entry) => {
          let desktop = DesktopInfo::new(file_path, desktop_entry);
          match desktop {
            Ok(desktop) => {
              println!("{}: [{}] '{}' {} -- {}", map_key, desktop.icon(), desktop.name(), desktop.active_str(), desktop.comment());
              sessions.entry(map_key).or_insert(desktop);
            }
            Err(error) => {
              println!("*** ERROR {}", error)
            }
          }
        }
        Err(_) => {
          // TODO: To error stream
          println!("*** Error: Unable to parse {}", file_path);
        }
      }
    }

    Ok(())
  }
}
