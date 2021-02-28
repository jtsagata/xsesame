pub use get_sessions::get_sessions as get_sessions;
pub use get_sessions::SesOption as SesOption;
pub use session_info::SessionInfo as SessionInfo;
pub use session_info::SessionType as SessionType;
pub use toggle::get_desktop_text as get_desktop_text;
pub use utils::get_default_session_dir as get_default_session_dir;

pub type SessionList = Vec<SessionInfo>;

mod get_sessions;
mod session_info;
mod utils;
mod toggle;


const VALID_TYPES: [&str; 2] = ["XSession", "Application"];
