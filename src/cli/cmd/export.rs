use clap::ArgMatches;

use crate::cli::cmd::parse_what_opt;
use crate::core::get_sessions;

pub fn export(xsession_dir: &str, options: Option<&ArgMatches>) {
  // clap will provide default values so it is ok
  let options = options.unwrap();
  let what = parse_what_opt(options);

  let sessions = get_sessions(&xsession_dir, what);
  let json = serde_json::to_string(&sessions).unwrap();
  println!("{}", json);
}


