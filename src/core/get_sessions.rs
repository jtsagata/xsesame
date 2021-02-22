use std::fs::read_dir;
use std::path::PathBuf;

use freedesktop_entry_parser::parse_entry;

use crate::core::{SessionInfo, SessionList};
use crate::core::utils::generate_path_key;

pub enum SesOption {
  All,
  Valid,
  Invalid,
}

pub fn get_session_info<'a>(sessions: &'a [SessionInfo], key: &str) -> Option<&'a SessionInfo> {
  for s in sessions {
    if s.key() == key {
      return Some(s);
    }
  }
  None
}

pub fn get_sessions(xsession_dir: &str, what: SesOption) -> SessionList {
  // Client must assure that xsession_dir exists, or panic!
  use regex::Regex;

  let re = Regex::new(r"^.*\.desktop.*$").unwrap();
  let mut sessions: SessionList = read_dir(xsession_dir)
    .expect(&*format!("Can't read directory {}", xsession_dir))
    .map(|it| it.map(|e| e.path()))
    .map(|it| make_string_form_rpb(&it))
    .filter(|path| re.is_match(path))
    .map(|f| {
      SessionInfo::new(generate_path_key(&f), f.clone(), parse_entry(f))
    }).collect();
  sessions.sort_by(|a: &SessionInfo, b: &SessionInfo| (a.key()).cmp(&b.key()));
  match what {
    SesOption::All => {
      sessions
    }
    SesOption::Valid => {
      sessions.iter().filter(|s| s.is_valid().is_ok()).cloned().collect()
    }
    SesOption::Invalid => {
      sessions.iter().filter(|s| s.is_valid().is_err()).cloned().collect()
    }
  }
}

fn make_string_form_rpb(it: &Result<PathBuf, std::io::Error>) -> String {
  let s = it.as_ref().unwrap();
  String::from(s.to_str().unwrap())
}
