use clap::ArgMatches;
use colored::*;
use stybulate::{Cell, Headers, Style, Table};
use terminal_size::{Height, terminal_size, Width};

use xsesame_core::get_sessions;

use crate::tools;

/// List sessions action
pub fn cmd_list_sessions(xsession_dir: &str, matches: &ArgMatches) {
  let style = matches.value_of("style").unwrap();
  let comments = matches.value_of("comments").unwrap();
  let use_nls = matches.is_present("nls");
  let use_emoji = matches.is_present("emoji");

  let style = match style {
    "Fancy" => { Style::Fancy }
    "Grid" => { Style::Grid }
    "Simple" => { Style::Simple }
    &_ => { Style::Plain }
  };

  // get Terminal Width default to true wif columns > 110
  let mut show_columns = if let Some((Width(w), Height(_h))) = terminal_size() {
    w > 110
  } else {
    true
  };

  show_columns = match comments {
    "auto" => { show_columns }
    "show" => { true }
    "hide" => { false }
    _ => { true }
  };

  let sessions = get_sessions(&xsession_dir);

  let mut elements: Vec<Vec<Cell>> = Vec::new();
  for (_, el) in sessions {
    let active_str = if use_emoji {
      if el.is_active() { " ✓ " } else { " ✗ " }
    } else if el.is_active() { "+" } else { "-" };
    let key = format!("{} {}", active_str, el.path_key());

    let comment = if use_nls {
      el.comment_with_nls()
    } else {
      el.comment()
    };

    if show_columns {
      elements.push(vec![Cell::from(key.as_str()), Cell::from(el.name().as_str()), Cell::from(comment.as_str())]);
    } else {
      elements.push(vec![Cell::from(key.as_str()), Cell::from(el.name().as_str())]);
    }
  }

  let table = Table::new(
    style, elements,
    if show_columns {
      Some(Headers::from(vec!["Key", "Name", "Comment"]))
    } else {
      Some(Headers::from(vec!["Key", "Name"]))
    },
  ).tabulate();

  println!("List of active and inactive sessions:");
  println!();
  println!("{}", table);
  println!();
  println!("To enable/disable a session run: {} {}", tools::program_name().unwrap().green(), "enable|disable <key>".green());
  println!();
}


