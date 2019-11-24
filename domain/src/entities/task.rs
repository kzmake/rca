use crate::value_objects::data::TaskData;
use crate::value_objects::ids::TaskId;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub data: TaskData,
}
