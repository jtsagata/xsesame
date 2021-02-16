mod collect;
mod to_json;
mod desktop_info;

pub use desktop_info::DesktopInfo;
pub use collect::get_sessions;

/// The default place of session data
#[cfg(debug_assertions)]
pub const XSESSION_DIR: &str = "../test/samples";
#[cfg(not(debug_assertions))]
pub const XSESSION_DIR: &str = "/usr/share/xsessions";

/// Get the default session dir
pub fn get_default_session_dir() -> &'static str {
  XSESSION_DIR
}
