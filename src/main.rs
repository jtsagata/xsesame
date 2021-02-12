mod desktop_info;

use std::io;
use crate::desktop_info::{DesktopInfo};
use std::collections::HashMap;


fn main() -> io::Result<()> {
  let mut sessions = HashMap::<String, DesktopInfo>::new();
  let xsession_dir = "/usr/share/xsessions/";

  if DesktopInfo::collect_sessions(&mut sessions, xsession_dir).is_err() {
    panic!("Unable to parse sessions");
  }

  Ok(())
}
