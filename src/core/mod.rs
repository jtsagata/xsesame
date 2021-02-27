pub use get_sessions::get_session_info as get_session_info;
pub use get_sessions::get_sessions as get_sessions;
pub use get_sessions::SesOption as SesOption;
pub use session_info::SessionInfo as SessionInfo;
pub use tools::program_name as program_name;
pub use utils::get_default_session_dir as get_default_session_dir;

pub type SessionList = Vec<SessionInfo>;

mod get_sessions;
mod session_info;
mod tools;
mod utils;
mod actions;


const VALID_TYPES: [&str; 2] = ["XSession", "Application"];
