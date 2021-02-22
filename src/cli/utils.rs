use std::env;

enum SessionKind {
  Wayland,
  X11,
  Terminal,
}


pub fn run_with_gui() -> bool {
  let session_type = session_type();
  match session_type {
    SessionKind::Wayland => { true }
    SessionKind::X11 => { true }
    SessionKind::Terminal => { false }
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
