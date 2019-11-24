use domain::entities::task::Task;

#[derive(Debug, Clone)]
pub struct CreateInputData {
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct CreateOutputData {
    pub task: Task,
}
