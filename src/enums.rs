use crate::tasks::{PrimitiveTask, CompTask};


pub enum ConditionType {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual
}

pub enum OperatorState {
    InProgress,
    Failed,
    Successful
}

pub enum ConditionStrictness {
    Any,
    All,
    AlwaysPass,
\}

pub enum EffectType {
    Add,
    Subtract,
    Set
}

pub enum Task {
    Primitive (PrimitiveTask),
    Compound (CompTask)
}