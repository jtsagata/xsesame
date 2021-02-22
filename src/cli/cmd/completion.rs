use std::io;

use clap::{ArgMatches, Shell};

use crate::opts;

pub fn completion(_: &str, options: Option<&ArgMatches>) {
  // clap will provide default vaues so it is ok
  let options = options.unwrap();
  let shell = match options.value_of("shell").unwrap() {
    "zsh" => { Shell::Zsh }
    "fish" => { Shell::Fish }
    "elvish" => { Shell::Elvish }
    &_ => { Shell::Bash }
  };

  opts::build_cli().gen_completions_to(clap::crate_name!(), shell, &mut io::stdout());
}
