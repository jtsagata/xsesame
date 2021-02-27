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

mod opts;
mod core;
mod cli;


#[cfg(target_os = "linux")]
fn main() {
  #[cfg(not(debug_assertions))]
  setup_panic!();
  #[cfg(debug_assertions)]
    better_panic::install();

  let matches = opts::build_cli().get_matches();

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
      cli::cmd::list(xsession_dir, opts);
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
    (_, _) => {
      todo!("TODO: no sub command do list or gui");
    }
  }


// if let Some(matches) = matches.subcommand_matches("list") {
//   cmd_list_sessions(xsession_dir, matches);
//   sub_command = true;
// }
//
// if let Some(matches) = matches.subcommand_matches("enable") {
//   cmd_enable_disable(xsession_dir, matches, &"desktop");
//   sub_command = true;
// }

// TODO: Check must be on method
// if let Some(matches) = matches.subcommand_matches("disable") {
//   // check if there is at least 2 enabled sessions
//   let active_sessions = get_sessions(&xsession_dir,SesOption::All)
//     .values()
//     .filter(|el| el.is_active())
//     .count();
//
//   if active_sessions < 2 {
//     eprintln!("{}", "There is only one active session! Nothing to be done!".yellow());
//   } else {
//     cmd_enable_disable(xsession_dir, matches, &"desktop-disable");
//   }
//   sub_command = true;
// }

//   if let Some(matches) = matches.subcommand_matches("toggle") {
//     let key = matches.value_of("session_key").unwrap();
//     cmd_toggle(&key, &xsession_dir);
//     sub_command = true;
//   }
//
//
//
// // If no subcommand is given rerun with list option
//   if !sub_command {
//     match core::run_with_gui() {
//       true => { todo!() }
//       false => { cmd_rerun_with_list_cmd(xsession_dir) }
//     }
//   }
}


#[cfg(not(target_os = "linux"))]
pub fn main() {
  println!("Not supported on this system");
}
