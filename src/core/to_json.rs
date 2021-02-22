use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::core::SessionInfo;

/// Serializer for DesktopInfo
impl Serialize for SessionInfo {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
  {
    let mut state = serializer.serialize_struct("Color", 7)?;
    state.serialize_field("key", self.key())?;
    state.serialize_field("path", self.filename())?;
    state.serialize_field("name", &self.name().unwrap_or_default())?;
    state.serialize_field("comment", &self.comment().unwrap_or_default())?;
    state.serialize_field("icon", &self.attr("Icon").unwrap_or_default())?;
    state.serialize_field("active", &self.is_active())?;
    state.serialize_field("valid", &self.is_valid().is_ok())?;
    state.end()
  }
}

