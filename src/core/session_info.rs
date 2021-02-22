use std::io;
use std::io::Error;
use std::path::Path;

use freedesktop_entry_parser::{Entry, parse_entry};

use crate::core::utils::generate_path_key;
use crate::core::VALID_TYPES;

pub struct SessionInfo {
  key: String,
  filename: String,
  pub data: Result<freedesktop_entry_parser::Entry, io::Error>,
}

impl Clone for SessionInfo {
  fn clone(&self) -> Self {
    let f = self.filename();
    SessionInfo::new(generate_path_key(&f), f.to_string(), parse_entry(f))
  }
}

#[derive(Debug)]
pub struct SessionError {
  message: String,
}

impl SessionError {
  pub fn new(message: String) -> Self {
    SessionError { message }
  }

  pub fn message(&self) -> &str {
    &self.message
  }
}


impl SessionInfo {
  pub fn new(key: String, filename: String, data: Result<Entry, Error>) -> Self {
    SessionInfo { key, filename, data }
  }

  pub fn key(&self) -> &str {
    &self.key
  }

  pub fn filename(&self) -> &str {
    &self.filename
  }

  pub fn name(&self) -> Option<String> {
    self.attr_with_nls("Name")
  }

  pub fn comment(&self) -> Option<String> {
    self.attr_with_nls("Comment")
  }


  pub fn is_valid(&self) -> Result<(), SessionError> {
    if !self.have_desktop_entry() {
      return Err(SessionError::new("Can't parse".to_string()));
    }
    let section = self.data.as_ref().unwrap().section("Desktop Entry");

    if section.has_attr("Type") {
      let type_attr = section.attr("Type").unwrap();
      if type_attr.is_empty() {
        return Err(SessionError::new("Empty Type Attribute".to_string()));
      }
      if !VALID_TYPES.contains(&type_attr) {
        return Err(SessionError::new("Invalid Type Attribute".to_string()));
      }
    } else {
      return Err(SessionError::new("No Type Attribute".to_string()));
    }

    if section.has_attr("Exec") {
      let exec = section.attr("Exec").unwrap();
      if exec.is_empty() {
        return Err(SessionError::new("Empty Exec Attribute".to_string()));
      }
    } else {
      return Err(SessionError::new("No Exec Attribute".to_string()));
    }

    Ok(())
  }

  /// Get the session state (active/inactive) from filename
  pub fn is_active(&self) -> bool {
    let path = Path::new(&self.filename);
    let ext = path.extension().unwrap();
    let is_hidden = self.attr("Hidden");
    let is_hidden = match is_hidden {
      None => { false }
      Some(attr) => { attr != "true" }
    };
    ext == "desktop" && is_hidden
  }


  pub fn have_desktop_entry(&self) -> bool {
    self.data.is_ok() && self.data.as_ref().unwrap().has_section("Desktop Entry")
  }

  pub fn attr(&self, name: &str) -> Option<String> {
    if !self.have_desktop_entry() {
      return None;
    }
    let it = self.data.as_ref().unwrap().section("Desktop Entry");
    if it.has_attr(name) {
      Some(it.attr(name).unwrap().to_string())
    } else {
      None
    }
  }

  pub fn attr_with_nls(&self, name: &str) -> Option<String> {
    if !self.have_desktop_entry() {
      return None;
    }
    let sel = self.data.as_ref().unwrap().section("Desktop Entry");
    for locale in get_locales() {
      if sel.has_attr_with_param(name, &locale) {
        return Some(String::from(sel.attr_with_param(name, locale).unwrap()));
      }
    }
    self.attr(name)
  }
}


/// Helper function to get user locales. Example:en-US,en
fn get_locales() -> Vec<String> {
  use itertools::Itertools;

  let locales: Vec<String> = locale_config::Locale::current()
    .tags_for("messages")
    .map(|l| l.to_string()).collect();
  let mut ret: Vec<String> = Vec::new();

  // https://github.com/rust-locale/locale_config/issues/7
  for l in locales {
    let split: Vec<&str> = l.split('-').collect();
    if split.len() > 1 {
      let plain = split[0].to_string();
      ret.push(l);
      ret.push(plain);
    } else {
      ret.push(l);
    }
  }
  // Remove duplicates
  ret.into_iter().unique().collect()
}
