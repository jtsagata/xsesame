use std::io;
use std::io::Write;

use clap::ArgMatches;
use colored::*;
use terminal_size::{terminal_size, Width};

use crate::core::{get_sessions, SessionInfo, SessionType};

pub fn list(xsession_dir: &str, options: Option<&ArgMatches>) {
  use crate::cli::cmd::parse_what_opt;

  let options = options.unwrap();
  let what = parse_what_opt(&options);

  let sessions = get_sessions(&xsession_dir, what);
  if sessions.is_empty() {
    println!("The is no valid sessions on '{}' directory", xsession_dir);
    return;
  }

  let key_len = get_max_len(&sessions, |e| { String::from(e.key()) });
  let name_len = get_max_len(&sessions, |e| { e.name().unwrap_or_default() });
  let mut comment_len = get_max_len(&sessions, |e| { e.name().unwrap_or_default() });

  let mut show_comments = true;
  let size = terminal_size();
  if let Some((Width(w), _)) = size {
    let width = w as usize;
    comment_len = width - key_len - name_len;
    comment_len = if comment_len >= 8 {
      comment_len - 8
    } else {
      0
    };
    if comment_len < 12 {
      show_comments = false;
    }
  }
  show_comments = match options.value_of("comments").unwrap() {
    "auto" => { show_comments }
    "show" => { true }
    "hide" => { false }
    _ => { true }
  };

  let comment_hdr = if show_comments {
    "Comment".bold()
  } else {
    "".white()
  };

  // Don't panic on pipe, so take the long way
  let _ = io::stdout().write("List of active and inactive sessions:\n".as_bytes());
  let _ = io::stdout().write(
    format!("   {:a$}  {:b$} {}\n", "Key".bold(), "Name".bold(), comment_hdr,
            a = key_len, b = name_len).as_bytes());

  for session in sessions {
    let valid = session.is_valid();
    if valid.is_ok() {
      let _ = io::stdout().write(
        &format!("{}  {:a$}  {:b$} {}\n",
                 fmt_bullet(&session, &options),
                 fmt_key(&session, &options),
                 fmt_name(&session, &options),
                 fmt_comment(&session, &options, comment_len, show_comments),
                 a = key_len, b = name_len
        ).as_bytes());
    } else {
      let err = valid.err().unwrap();
      let err = format!("<{}>", err.message());
      let _ = io::stdout().write(
        format!("{}  {:a$}  {}\n", fmt_bullet(&session, &options),
                fmt_key(&session, &options), err.red(),
                a = key_len
        ).as_bytes());
    }
  }
}

fn get_max_len<F>(sessions: &[SessionInfo], f: F) -> usize
  where F: Fn(&SessionInfo) -> String
{
  let max_item = sessions.iter().max_by_key(|ses| {
    f(ses).len()
  }).unwrap();
  f(max_item).len()
}


fn fmt_bullet(session: &SessionInfo, options: &ArgMatches) -> ColoredString {
  use crate::cli::cmd::{EMOJI_STATES, ASCII_STATES, EMOJI_HEARTS};

  let emoji_table = match options.value_of("emoji").unwrap() {
    "hearts" => EMOJI_HEARTS,
    "check" => EMOJI_STATES,
    "plain" => ASCII_STATES,
    &_ => EMOJI_HEARTS
  };


  let text = match session.state() {
    SessionType::Active => { emoji_table[0] }
    SessionType::Inactive => { emoji_table[1] }
    SessionType::Invalid => { emoji_table[2] }
  };

  color_str(text, &session)
}

fn color_str(str: &str, session: &SessionInfo) -> ColoredString {
  match session.state() {
    SessionType::Active => { str.green() }
    SessionType::Inactive => { str.yellow() }
    SessionType::Invalid => { str.red() }
  }
}

fn fmt_key(session: &SessionInfo, _options: &ArgMatches) -> ColoredString {
  color_str(session.key(), &session)
}

fn fmt_name(session: &SessionInfo, _options: &ArgMatches) -> ColoredString {
  let name = session.name().unwrap_or_default();
  color_str(&name, &session)
}


fn fmt_comment(session: &SessionInfo, options: &ArgMatches, max: usize, show: bool) -> ColoredString {
  use atty::Stream;
  if !show {
    return "".white();
  }

  let nls = !options.is_present("nls");
  let comment = match nls {
    true => { session.attr_with_nls("Comment") }
    false => { session.comment() }
  };
  let comment = comment.unwrap_or_default();
  if atty::is(Stream::Stdout) {
    let comment = truncate_ellipse(&comment, max);
    color_str(&comment, &session)
  } else {
    color_str(&comment, &session)
  }
}


fn truncate_ellipse(text: &str, len: usize) -> String {
  use unicode_segmentation::UnicodeSegmentation;

  if text.graphemes(true).count() <= len {
    return String::from(text);
  } else if len == 0 {
    return String::from("");
  }

  text.graphemes(true)
    .take(len)
    .chain("…".graphemes(true))
    .collect()
}
