use std::{env, process};

use exec::Error;

enum SessionKind {
  Wayland,
  X11,
  Terminal,
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let mut executable = String::from(&args[0]);

  let mut prefix = match session_type() {
    SessionKind::Wayland => { "-gtk" }
    SessionKind::X11 => { "-gtk" }
    SessionKind::Terminal => { "-cli" }
  };

  // check if 2nd argument is 'help' or '--help'
  if args.len() > 1 {
    let arg2 = &args[1];
    if arg2 == "help" || arg2 == "--help" {
      prefix = "-cli";
    }
  }

  executable += prefix;

  let mut cmd = exec::Command::new(&executable);

  let mut it = args.iter();
  it.next();

  for arg in it {
    cmd.arg(arg);
  }

  let err = cmd.exec();
  match err {
    Error::BadArgument(_) => {}
    Error::Errno(errno) => {
      eprintln!("Error {}", err);
      process::exit(errno.0);
    }
  }
}

fn session_type() -> SessionKind {
  return match env::var("XDG_SESSION_TYPE") {
    Ok(ok) => match ok.to_lowercase().as_ref() {
      "wayland" => SessionKind::Wayland,
      _ => SessionKind::X11,
    },
    Err(_) => {
      SessionKind::Terminal
    }
  };
}
