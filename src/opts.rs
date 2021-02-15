use clap::{App, AppSettings, Arg, crate_version, SubCommand};

/// The default place of session data
pub const XSESSION_DIR: &str = "/usr/share/xsessions";

/// Generate the command line argument structure
pub fn build_cli() -> App<'static, 'static> {
  let help_start = "\nSome desktop environments install a lot of different types that have to real use for
the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
This small utility helps you to list and disable some of them. Of course you can also re-enable them.

The propose of this little tool is to minimize the clutter in the display manager.";

  App::new("xsesame")
    .version(crate_version!())
    // .author(crate_authors!("\n"))
    .about("Manage display manager sessions. Allow to list, enable and disable them")
    .after_help("Use help <subcommand> for more info.")
    .long_about(help_start)

    .setting(AppSettings::UnifiedHelpMessage)
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::ColorAuto)
    .setting(AppSettings::GlobalVersion)
    .setting(AppSettings::InferSubcommands)

    .arg(Arg::with_name("session-dir")
      .long("session-dir").short("d")
      .value_name("XSESSION_DIR").takes_value(true)
      .default_value(XSESSION_DIR)
      .global(true)
      .help("Session config directory")
      .next_line_help(true)
      .display_order(1)
    )

    .subcommand(
      SubCommand::with_name("export")
        .about("Export session list")
        .setting(AppSettings::ColoredHelp)
        .after_help("For use with integration with guis")
        .display_order(4)
    )

    .subcommand(
      SubCommand::with_name("completion")
        .about("Generate completions for various shells")
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("shell")
          .require_equals(true)
          .possible_values(&["bash", "zsh", "fish", "elvish"])
          .default_value("bash")
          .takes_value(true).value_name("shell")
          .hide_default_value(true)
          .help("shell to generate completions")
        )
    )

    .subcommand(build_enable_disable_cli(
      "enable",
      "Enable a session using a key from list (for example cinnamon2d)")
      .display_order(2)
    )

    .subcommand(build_enable_disable_cli(
      "disable",
      "Disable a session using a key from list (for example cinnamon2d)")
      .display_order(3)
    )

    .subcommand(
      SubCommand::with_name("list")
        .about("list display manager sessions")
        .setting(AppSettings::ColoredHelp)
        .long_about(help_start)
        .display_order(1)

        .arg(Arg::with_name("style")
          .possible_values(&["Plain", "Fancy", "Grid", "Simple"])
          .takes_value(true).value_name("Style")
          .default_value("Fancy")
          .hide_default_value(true)
          .help("Display table with style")
        )

        .arg(Arg::with_name("emoji")
          .long("with-emoji").short("e")
          .require_equals(true)
          .value_name("emoji").takes_value(true)
          .default_value("yes")
          .possible_values(&["yes", "no"])
          .global(true)
          .help("Nice active/inactive symbols")
          .next_line_help(true)
          .display_order(2)
        )

        .arg(Arg::with_name("nls")
          .long("with-nls").short("l")
          .require_equals(true)
          .value_name("nls").takes_value(true)
          .default_value("yes")
          .possible_values(&["yes", "no"])
          .global(true)
          .help("Show comment localized if possible")
          .next_line_help(true)
          .display_order(2)
        )
    )
}

pub fn build_enable_disable_cli(name: &str, description: &'static str) -> App<'static, 'static> {
  SubCommand::with_name(name)
    .about("Disable a session name")
    .setting(AppSettings::ColoredHelp)
    .arg(Arg::with_name("session_key")
      .required(true)
      .takes_value(true).value_name("session key")
      .hide_default_value(true)
      .help(description)
    )
}
