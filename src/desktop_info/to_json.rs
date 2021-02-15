use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::desktop_info::DesktopInfo;
use crate::get_sessions;

/// Serializer for DesktopInfo
impl Serialize for DesktopInfo {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
  {
    let mut state = serializer.serialize_struct("Color", 7)?;
    state.serialize_field("key", self.path_key().as_str())?;
    state.serialize_field("path", self.path.as_str())?;
    state.serialize_field("name", self.name().as_str())?;
    state.serialize_field("comment", self.comment().as_str())?;
    state.serialize_field("comment-nls", self.comment_with_nls().as_str())?;
    state.serialize_field("icon", self.icon().as_str())?;
    state.serialize_field("active", &self.is_active())?;
    state.end()
  }
}

/// Export sessions as json
pub fn export(xsession_dir: &str) -> String {
  let sessions = get_sessions(&xsession_dir);
  serde_json::to_string(&sessions).unwrap()
}
