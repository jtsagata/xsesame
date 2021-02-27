use clap::{App, AppSettings, Arg, crate_version, SubCommand};

use crate::core::get_default_session_dir;

const HELP_TOP: &str = "\nSome desktop environments install a lot of different types that have to real use for
the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
This small utility helps you to list and disable some of them. Of course you can also re-enable them.

The propose of this little tool is to minimize the clutter in the display manager.";

/// Generate the command line argument structure
pub fn build_cli() -> App<'static, 'static> {
  App::new("xsesame")
    .version(crate_version!())
    // .author(crate_authors!("\n"))
    .about("Manage display manager sessions. Allow to list, enable and disable them")
    .after_help("Use help <subcommand> for more info.")
    .long_about(HELP_TOP)

    .setting(AppSettings::UnifiedHelpMessage)
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::ColorAuto)
    .setting(AppSettings::GlobalVersion)
    .setting(AppSettings::InferSubcommands)

    .arg(session_dir_arg().global(true))

    .subcommand(
      SubCommand::with_name("export")
        .about("Export session list")
        .setting(AppSettings::ColoredHelp)
        .after_help("For use with integration with guis")
        .display_order(4)
        .arg(
          Arg::with_name("what")
            .long("what").short("w")
            .takes_value(true)
            .value_name("what")
            .default_value("all")
            .require_equals(true)
            .possible_values(&["all", "valid", "invalid"])
            .hide_default_value(true)
            .help("filter results")
            .next_line_help(true)
        )
    )

    .subcommand(
      SubCommand::with_name("completion")
        .about("Generate completions for various shells")
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("shell")
          .long("shell").short("s")
          .possible_values(&["bash", "zsh", "fish", "elvish"])
          .default_value("bash")
          .takes_value(true).value_name("shell")
          .require_equals(true)
          .hide_default_value(true)
          .help("shell to generate completions")
          .next_line_help(true)
        )
    )

    .subcommand(build_enable_disable_cli(
      "enable", "Enable a session")
      .display_order(3)
    )

    .subcommand(
      build_enable_disable_cli("disable", "Disable a session")
        .display_order(4)
    )

    .subcommand(
      build_enable_disable_cli("toggle", "Toggle session visibility")
        .help_short("Toggle session visibility")
        .display_order(2)
    )


    .subcommand(build_list_cli())
}

// Generate '--session-dir' argument
fn session_dir_arg() -> Arg<'static, 'static> {
  Arg::with_name("session-dir")
    .long("session-dir").short("d")
    .takes_value(true)
    .value_name("session-dir")
    .default_value(get_default_session_dir())
    .help("Session config directory")
    .next_line_help(true)
    .display_order(1)
}


/// Generate the command line argument structure for list
fn build_list_cli() -> App<'static, 'static> {
  SubCommand::with_name("list")
    .about("list display manager sessions")
    .setting(AppSettings::ColoredHelp)
    .long_about(HELP_TOP)
    .display_order(1)

    // .arg(session_dir_arg())

    .arg(Arg::with_name("comments")
      .short("c").long("comments")
      .help("Show comments")
      .takes_value(true).value_name("comments")
      .default_value("auto")
      .possible_values(&["auto", "show", "hide"])
      .hide_default_value(true)
      .require_equals(true)
      .next_line_help(true)
    )


    .arg(Arg::with_name("emoji")
      .long("no-emoji").short("e")
      .help("Nice active/inactive symbols")
      .display_order(2)
    )

    .arg(Arg::with_name("nls")
      .long("no-nls").short("l")
      .help("Show comment localized if possible")
      .display_order(2)
    )
}

/// Generate the command line argument structure for enable/disable
fn build_enable_disable_cli(name: &str, description: &'static str) -> App<'static, 'static> {
  SubCommand::with_name(name)
    .about(description)
    .setting(AppSettings::ColoredHelp)
    .arg(Arg::with_name("session_key")
      .required(true)
      .takes_value(true).value_name("session key")
      .hide_default_value(true)
      .help(description)
    )
    .arg(Arg::with_name("no-journald")
      .long("no-journal").short("J")
      .help("Disable logging to journal")
    )
}
