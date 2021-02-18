pub use collect::get_sessions as get_sessions;
pub use desktop_info::DesktopInfo as DesktopInfo;
pub use tools::get_filename_from_key as get_filename_from_key;
pub use tools::program_name as program_name;

mod collect;
mod to_json;
mod desktop_info;
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
