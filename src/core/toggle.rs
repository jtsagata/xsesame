use std::fmt::Write;

use crate::core::SessionInfo;

pub fn get_desktop_text(session: &SessionInfo, enable: bool) -> String {
  let mut text = String::new();
  let desktop = &session.data;
  let mut seen_hidden = false;

  let value = match enable {
    true => { "false" }
    false => { "true" }
  };


  if desktop.is_ok() {
    let desktop = desktop.as_ref().unwrap();
    for section in desktop.sections() {
      let on_desktop_entry = section.name() == "Desktop Entry";
      let _ = writeln!(text, "[{}]", section.name());
      for attr in section.attrs() {
        if on_desktop_entry && attr.name == "Hidden" {
          let _ = writeln!(text, "{}={}", attr.name, value);
          seen_hidden = true;
        } else {
          let _ = writeln!(text, "{}={}", attr.name, attr.value.unwrap_or_default());
        }
        for p in attr.params() {
          let _ = writeln!(text, "{}[{}]={}", p.attr_name, p.param_val, section.attr_with_param(p.attr_name, p.param_val).unwrap());
        }
      }
      if !seen_hidden {
        let _ = writeln!(text, "Hidden={}", value);
      }
    }
  }
  text
}
