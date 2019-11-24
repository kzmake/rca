pub struct CreateTaskInteractor {}

impl CreateTaskInteractor {
    pub fn new() -> CreateTaskInteractor {
        return CreateTaskInteractor {};
    }

    pub fn execute(&self, _data: String) {
        println!("CreateTaskInteractor.execute: {}", _data)
    }
}
