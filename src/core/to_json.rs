use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::core::SessionInfo;

/// Serializer for DesktopInfo
impl Serialize for SessionInfo {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
  {
    let mut state = serializer.serialize_struct("Color", 7)?;
    state.serialize_field("key", self.get_path_key().as_str())?;
    state.serialize_field("path", self.path().as_str())?;
    state.serialize_field("name", self.get_name().as_str())?;
    state.serialize_field("comment", self.get_comment(false).as_str())?;
    state.serialize_field("comment-nls", self.get_comment(true).as_str())?;
    state.serialize_field("icon", self.icon().as_str())?;
    state.serialize_field("active", &self.is_active())?;
    state.end()
  }
}

