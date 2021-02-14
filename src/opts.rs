use clap::{SubCommand, Arg, crate_authors, crate_version, AppSettings, App};

pub const XSESSION_DIR: &str = "/usr/share/xsessions";


pub fn build_cli() -> App<'static, 'static> {
  let help_start = "\nSome desktop environments install a lot of different types that have to real use for
the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
This small utility helps you to list and disable some of them. Of course you can also re-enable them.

The propose of this little tool is to minimize the clutter in the display manager.";

  App::new("xsesame")
    .version(crate_version!())
    .author(crate_authors!("\n"))
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
      .require_equals(true)
      .hide_default_value(true)
      .global(true)
      .help("Session config directory")
    )

    .subcommand(
      SubCommand::with_name("export")
        .about("Export session list")
        .setting(AppSettings::Hidden)
    )

    .subcommand(
      SubCommand::with_name("completion")
        .about("Generate completions for various shells")
        .arg(Arg::with_name("shell")
          .require_equals(true)
          .possible_values(&["bash", "zsh", "fish", "elvish"])
          .default_value("bash")
          .takes_value(true).value_name("shell")
          .hide_default_value(true)
          .help("shell to generate completions")
        )
    )

    .subcommand(
      SubCommand::with_name("list")
        .about("list display manager sessions")
        .long_about(help_start)
        .display_order(1)
        .arg(Arg::with_name("style")
          .possible_values(&["Plain", "Fancy", "Grid", "Simple"])
          .takes_value(true).value_name("Style")
          .default_value("Fancy")
          .hide_default_value(true)
          .help("Display table with style")
        )
    )
}
