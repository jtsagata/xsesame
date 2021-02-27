use clap::ArgMatches;

pub use completion::completion as completion;
pub use disable::disable as disable;
pub use enable::enable as enable;
pub use export::export as export;
pub use list::list as list;
pub use toggle::toggle as toggle;

use crate::core::SesOption;

mod export;
mod completion;
mod disable;
mod enable;
mod list;
mod toggle;

fn parse_what_opt(options: &ArgMatches) -> SesOption {
  match options.value_of("what").unwrap() {
    "all" => { SesOption::All }
    "valid" => { SesOption::Valid }
    "invalid" => { SesOption::Invalid }
    &_ => { SesOption::Valid }
  }
}

const ASCII_STATES: [&str; 3] = ["+", "-", "x"];
const EMOJI_HEARTS: [&str; 3] = ["💚", "🤍", "💔"];
const EMOJI_STATES: [&str; 3] = [figures_rs::TICK, figures_rs::CROSS, figures_rs::WARNING];
