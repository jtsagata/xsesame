use clap::ArgMatches;

use crate::core::{get_sessions, SesOption};

pub fn export(xsessions_dir: &str, options: Option<&ArgMatches>) {
  // clap will provide default values so it is ok
  let options = options.unwrap();
  let what = match options.value_of("what").unwrap() {
    "all" => { SesOption::All }
    "valid" => { SesOption::Valid }
    "invalid" => { SesOption::Invalid }
    &_ => { SesOption::Valid }
  };

  let sessions = get_sessions(&xsessions_dir, what);
  let json = serde_json::to_string(&sessions).unwrap();
  println!("{}", json);
}
