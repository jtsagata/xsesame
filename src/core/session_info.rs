use std::path::Path;

use freedesktop_entry_parser::Entry;

/// A path to a session file, and a parser to it
pub struct SessionInfo {
  path: String,
  desktop: freedesktop_entry_parser::Entry,
  error: Option<String>,
}

impl SessionInfo {
  /// Create a new DesktopInfo object and validate some basic properties
  pub fn new(path: String, desktop: Entry) -> Self {
    let path_copy = path.clone();

    let file = Path::new(&path_copy).file_name();
    let just_the_file = file.unwrap().to_string_lossy();

    let error: Option<String> = None;
    let mut entry = SessionInfo { path, desktop, error };

    // Test Type="Application"
    let d_type = entry.get_attribute("Type");
    match d_type {
      None => {
        entry.error = Some(format!("The file '{}' does not specify a {}.", just_the_file, "Type"));
      }
      // TODO: Also XSession
      Some(d_type) => if d_type != "Application" {
        entry.error = Some(format!("The file '{}' specify Type as '{}'", just_the_file, d_type));
      },
    }

    // Test if provides a Name
    if entry.get_attribute("Name").is_none() {
      entry.error = Some(format!("The file '{}' does not specify a Name.", just_the_file));
    }

    entry
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

  /// Get the session icon (if any)
  pub fn icon(&self) -> String {
    self.get_attribute_str("Icon")
  }

  /// Get if valid
  pub fn is_valid(&self) -> bool {
    self.error.is_none()
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

