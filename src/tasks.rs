use crate::enums::{ConditionStrictness};

pub struct PrimitiveTask {
    pub task_id: usize,
    pub operator_id: usize,
    pub effect_ids: Vec<usize>,
    pub condition_ids: Vec<usize>,
}

pub struct CompTask {
    pub task_id: usize,
    pub method_ids: Vec<usize>,
}

pub struct Method {
    pub method_id: usize,

    pub strictness: ConditionStrictness,
    pub condition_ids: Vec<usize>,    
    pub subtask_ids: Vec<usize>,
}