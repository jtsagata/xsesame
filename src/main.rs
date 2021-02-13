mod desktop_info;

use std::io;
use crate::desktop_info::{DesktopInfo};
use std::collections::HashMap;

// TODO: From cargo?
// const XSESSION_DIR: &str = "/usr/share/xsessions/";
const XSESSION_DIR: &str = "/home/talos/CLionProjects/Rust/sesman/test/samples";

fn main() -> io::Result<()> {
  let mut sessions = HashMap::<String, DesktopInfo>::new();

  if DesktopInfo::collect_sessions(&mut sessions, XSESSION_DIR).is_err() {
    panic!("Unable to parse sessions");
  }

  // TODO: Print sessions as table

  Ok(())
}
