use std::collections::BTreeMap;

use clap::ArgMatches;
use colored::*;
use pad::PadStr;
use terminal_size::{Height, terminal_size, Width};

use crate::core::{get_sessions, program_name, SessionInfo};

type SessionMap = BTreeMap<String, SessionInfo>;

fn get_max_len<F>(sessions: &SessionMap, f: F) -> usize
  where F: Fn(&SessionInfo) -> String
{
  let max_item = sessions.iter().max_by_key(|(_, ses)| {
    f(ses).len()
  }).unwrap().1;
  f(max_item).len()
}

fn color_str(str: &str, acive: bool) -> String {
  if acive {
    str.white().to_string()
  } else {
    str.bright_red().to_string()
  }
}

/// List sessions action
pub fn cmd_list_sessions(xsession_dir: &str, options: &ArgMatches) {
  let use_nls = options.is_present("nls");
  let use_emoji = options.is_present("emoji");
  let show_comments_opt = options.value_of("comments").unwrap();
  let show_comments = check_if_show_comments(show_comments_opt);

  let sessions = get_sessions(&xsession_dir);
  let mut filter_sessions = SessionMap::new();
  sessions.into_iter()
    .filter(|(_, ses)| ses.is_valid())
    .for_each(|(key, ses)| {
      filter_sessions.insert(key, ses);
    });

  let max_key_len = get_max_len(&filter_sessions, |e| { e.get_path_key() });
  let max_name_len = get_max_len(&filter_sessions, |e| { e.get_name() });

  let mut display_lines: Vec<Vec<String>> = Vec::new();
  for (_, ses) in filter_sessions {
    let active_str_color = color_str(&get_active_str(use_emoji, &ses), ses.is_active());
    let path_key = ses.get_path_key().pad_to_width(max_key_len);
    let path_key_color = color_str(&path_key, ses.is_active());
    let name = ses.get_name().pad_to_width(max_name_len);
    let name_color = color_str(&name, ses.is_active());
    let comment_color = color_str(&ses.get_comment(use_nls), ses.is_active());

    let mut line = vec![active_str_color, path_key_color, name_color];
    if show_comments {
      line.push(comment_color)
    }
    display_lines.push(line);
  }

  println!("List of active and inactive sessions:");
  if show_comments {
    println!("   {:a$}  {:b$} {}", "Key".bold(), "Name".bold(), "Comment".bold(),
             a = max_key_len, b = max_name_len);
    for line in display_lines {
      println!("{}  {}  {} {}", line[0].bold(), line[1], line[2], line[3]);
    }
  } else {
    println!("   {:a$}  {:b$}", "Key".bold(), "Name".bold(), a = max_key_len, b = max_name_len);
    for line in display_lines {
      println!("{}  {}  {}", line[0].bold(), line[1], line[2]);
    }
  }
  println!();
  println!("To enable/disable a session run: {} {}", program_name().unwrap().green(), "enable|disable <key>".green());
  println!();
}


fn check_if_show_comments(comments: &str) -> bool {
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
  show_columns
}


/// Get active state as a displayable str
fn get_active_str(use_emoji: bool, el: &SessionInfo) -> String {
  if use_emoji {
    if el.is_active() { "✓" } else { "✗" }
  } else if el.is_active() { "+" } else { "-" }.to_string()
}


