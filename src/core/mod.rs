use std::env;

pub use collect::get_sessions as get_sessions;
pub use session_info::SessionInfo as SessionInfo;
pub use tools::get_filename_from_key as get_filename_from_key;
pub use tools::program_name as program_name;

mod collect;
mod to_json;
mod session_info;
mod tools;

/// The default place of session data
#[cfg(debug_assertions)]
pub const XSESSION_DIR: &str = "test/samples";
#[cfg(not(debug_assertions))]
pub const XSESSION_DIR: &str = "/usr/share/xsessions";

/// Get the default session dir
pub fn get_default_session_dir() -> &'static str {
  XSESSION_DIR
}


enum SessionKind {
  Wayland,
  X11,
  Terminal,
}


pub fn run_with_gui() -> bool {
  let session_type = session_type();
  match session_type {
    SessionKind::Wayland => { true }
    SessionKind::X11 => { true }
    SessionKind::Terminal => { false }
  }
}

fn session_type() -> SessionKind {
  return match env::var("XDG_SESSION_TYPE") {
    Ok(ok) => match ok.to_lowercase().as_ref() {
      "wayland" => SessionKind::Wayland,
      _ => SessionKind::X11,
    },
    Err(_) => {
      SessionKind::Terminal
    }
  };
}
