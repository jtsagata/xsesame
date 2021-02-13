use freedesktop_entry_parser::Entry;
use std::path::Path;
use std::collections::HashMap;
use std::{io, fs};
use freedesktop_entry_parser::parse_entry;
use colored::*;

pub struct DesktopInfo {
  path: String,
  desktop: freedesktop_entry_parser::Entry,
}

impl DesktopInfo {
  pub fn new(path: String, desktop: Entry) -> Result<Self, String> {
    let path_copy = path.clone();
    let just_the_file = Path::new(&path_copy).file_name().unwrap().to_string_lossy();
    let entry = DesktopInfo { path, desktop };

    // Test Type="Application"
    let d_type = entry.get_attribute("Type");
    match d_type {
      None => {
        return Err(format!("The file '{}' does not specify a {}.", just_the_file.green(), "Type".bold()));
      }
      Some(d_type) => if d_type != "Application" {
        return Err(format!("The file '{}' specify {} as '{}'.", just_the_file.green(), "Type".bold(), d_type.green()));
      },
    }

    // Test if provides a Name
    if entry.get_attribute("Name").is_none() {
      return Err(format!("The file '{}' does not specify a {}.", just_the_file.green(), "Name".bold()));
    }

    Ok(entry)
  }

  pub fn path(&self) -> String {
    self.path.to_string()
  }

  pub fn path_key(&self) -> String {
    let path = Path::new(&self.path).with_extension("");
    let res = path.file_name().unwrap();
    res.to_str().unwrap().to_string()
  }

  pub fn name(&self) -> String {
    self.get_attribute_str("Name")
  }

  pub fn comment(&self) -> String {
    let mut lang_env = std::env::var("LC_ALL");
    if lang_env.is_err() {
      lang_env = std::env::var("LANG");
    }
    return match lang_env {
      Ok(lang_env) => {
        let lang = env_lang::to_struct(&lang_env).unwrap().language.unwrap();
        let localized = self.get_attribute_with_locale("Comment", lang);
        match localized {
          None => { self.get_attribute_str("Comment") }
          Some(text) => { text }
        }
      }
      Err(_) => {
        self.get_attribute_str("Comment")
      }
    };
  }

  fn get_attribute(&self, attr: &str) -> Option<&str> {
    return self.desktop.section("Desktop Entry").attr(attr);
  }

  fn get_attribute_with_locale(&self, attr: &str, locale: &str) -> Option<String> {
    let section = self.desktop.section("Desktop Entry");
    let localized = section.attr_with_param(attr, locale);
    match localized {
      None => { None }
      Some(txt) => {
        Some(txt.to_string())
      }
    }
  }

  fn get_attribute_str(&self, attr: &str) -> String {
    Option::or(self.get_attribute(&attr), Some("")).unwrap().to_string()
  }

  pub fn is_active(&self) -> bool {
    let path = Path::new(&self.path);
    let ext = path.extension().unwrap();
    ext == "desktop"
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
              println!("{} {}", "Error:".red(), error)
            }
          }
        }
        Err(_) => {
          let just_the_file = Path::new(&file_path).file_name().unwrap().to_string_lossy();
          eprintln!("{} Unable to parse '{}'.", "Error:".red(), just_the_file.green());
        }
      }
    }

    Ok(())
  }
}
