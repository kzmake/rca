extern crate clap;

use clap::ArgMatches;

use interface::controllers::tasks::TaskController;

pub struct TaskHandler {
    controller: TaskController,
}

impl TaskHandler {
    pub fn new(controller: TaskController) -> TaskHandler {
        return TaskHandler { controller };
    }

    pub fn create(&self, args: &ArgMatches) {
        if let Some(data) = args.value_of("data") {
            self.controller.create(data.to_owned());
        }
    }
}
