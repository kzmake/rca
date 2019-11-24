use clap::{App, AppSettings};

pub fn build_new() -> App<'static, 'static> {
    let new_command = App::new("new")
        .about("Create a content")
        .setting(AppSettings::SubcommandRequiredElseHelp);

    new_command
}
