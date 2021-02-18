use clap::{App, AppSettings, Arg, crate_version};
use xsesame_core::get_default_session_dir;

const HELP_TOP: &str = "\nSome desktop environments install a lot of different types that have to real use for
the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
This small utility helps you to list and disable some of them. Of course you can also re-enable them.

The propose of this little tool is to minimize the clutter in the display manager.";

/// Generate the command line argument structure for the gtk version
pub fn build_cli_gui() -> App<'static, 'static> {
  App::new("xsesame-gtk")
    .version(crate_version!())
    // .author(crate_authors!("\n"))
    .about("Manage display manager sessions. Allow to list, enable and disable them")
    .after_help("Use help <subcommand> for more info.")
    .long_about(HELP_TOP)

    .setting(AppSettings::ColoredHelp)

    .arg(Arg::with_name("session-dir")
      .long("session-dir").short("d")
      .value_name("XSESSION_DIR").takes_value(true)
      .default_value(get_default_session_dir())
      .help("Session config directory")
      .next_line_help(true)
    )
}

