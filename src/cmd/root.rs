use clap::{App, AppSettings};

pub fn build_root() -> App<'static, 'static> {
    let root_command = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::HidePossibleValuesInHelp)
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .max_term_width(100)
        .about(
            "A cat(1) clone with wings.\n\n\
             Use '--help' instead of '-h' to see a more detailed version of the help text.",
        )
        .long_about("A cat(1) clone with syntax highlighting and Git integration.")
        .help_message("Print this help message.")
        .version_message("Show version information.");

    root_command
}
