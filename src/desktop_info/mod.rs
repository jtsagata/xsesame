use std::path::Path;

use colored::*;
use freedesktop_entry_parser::Entry;

pub mod collect;

/// A path to a session file, and a parser to it
pub struct DesktopInfo {
  path: String,
  desktop: freedesktop_entry_parser::Entry,
}

impl DesktopInfo {
  /// Create a new DesktopInfo object and validate some basic properties
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

  /// Export the path
  pub fn path(&self) -> String {
    self.path.to_string()
  }

  /// Get the path key from filename
  pub fn path_key(&self) -> String {
    let path = Path::new(&self.path).with_extension("");
    let res = path.file_name().unwrap();
    res.to_str().unwrap().to_string().to_lowercase()
  }

  /// Get the session name
  pub fn name(&self) -> String {
    self.get_attribute_str("Name")
  }

  /// Get the session comment in native language if available, fallbacks to English
  pub fn comment_with_nls(&self) -> String {
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

  /// Get the session comment
  pub fn comment(&self) -> String {
        self.get_attribute_str("Comment")
  }



  /// Helper to get an attribute
  fn get_attribute(&self, attr: &str) -> Option<&str> {
    return self.desktop.section("Desktop Entry").attr(attr);
  }

  /// Helper to get an attribute localized
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

  /// Helper to get an attribute as string
  fn get_attribute_str(&self, attr: &str) -> String {
    Option::or(self.get_attribute(&attr), Some("")).unwrap().to_string()
  }

  /// Get the session state (active/inactive) from filename
  pub fn is_active(&self) -> bool {
    let path = Path::new(&self.path);
    let ext = path.extension().unwrap();
    ext == "desktop"
  }
}
