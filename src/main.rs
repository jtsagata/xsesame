//! xsessame
//! Some desktop environments install a lot of different types that have to real use for
//! the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
//! This small utility helps you to list and disable some of them. Of course you can also re-enable them.
//!
//! The propose of this little tool is to minimize the clutter in the display manager.
//!
//!
//
use std::path::Path;
use std::process;

use colored::*;

use crate::cli::run_with_gui;

mod core;
mod cli;
mod cmdline_opts;


#[cfg(target_os = "linux")]
fn main() {
  #[cfg(not(debug_assertions))]
  setup_panic!();
  #[cfg(debug_assertions)]
    better_panic::install();

  let matches = cmdline_opts::build_cli().get_matches();

  // Sessions directory
  let xsession_dir = matches.value_of("session-dir").unwrap();
  if !Path::new(xsession_dir).is_dir() {
    eprintln!("{} '{}' is not a directory", "Error:".red(), xsession_dir.green());
    process::exit(-1);
  }

  match matches.subcommand() {
    ("list", opts) => {
      cli::cmd::list(xsession_dir, opts);
    }
    ("enable", opts) => {
      cli::cmd::enable(xsession_dir, opts);
    }
    ("disable", opts) => {
      cli::cmd::disable(xsession_dir, opts);
    }
    ("toggle", opts) => {
      cli::cmd::toggle(xsession_dir, opts);
    }
    ("export", opts) => {
      cli::cmd::export(xsession_dir, opts);
    }
    ("completion", opts) => {
      cli::cmd::completion(xsession_dir, opts);
    }
    (_, opts) => {
      match run_with_gui() {
        true => { todo!("TODO: gui"); }
        false => {
          cli::cmd::list(xsession_dir, opts);
        }
      }
    }
  }
}


#[cfg(not(target_os = "linux"))]
pub fn main() {
  println!("Not supported on this system");
}
