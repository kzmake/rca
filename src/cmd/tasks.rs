use clap::{App, Arg};

pub fn build_new_task() -> App<'static, 'static> {
    let new_task_command = App::new("tasks")
        .about("Create a task")
        .aliases(&["task"])
        .arg(
            Arg::with_name("data")
                .required(true)
                .help("a new task data"),
        );

    new_task_command
}
