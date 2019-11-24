#[macro_use]
extern crate clap;

use std::process;

use infrastructure::handlers::tasks::TaskHandler;
use interface::controllers::tasks::TaskController;
use usecase::interactors::tasks::CreateTaskInteractor;

mod app;
mod cmd;

use crate::app::App;

fn run() -> Result<bool, String> {
    let app = App::new()?;
    println!("Debug: {:?}", app.matches);

    let i = CreateTaskInteractor::new();
    let c = TaskController::new(i);
    let h = TaskHandler::new(c);

    match app.matches.subcommand() {
        ("new", Some(new_matches)) => match new_matches.subcommand() {
            ("tasks", Some(task_matches)) => {
                h.create(task_matches);
                Ok(true)
            }
            ("", None) => Err(String::from("hoge")),
            _ => Ok(true),
        },
        ("", None) => {
            println!("No subcommand was used");
            Ok(true)
        }
        _ => Ok(true),
    }
}

fn main() {
    let result = run();

    match result {
        Err(_error) => {
            // handle_error(&error);
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}
