use enums::{EffectType, ConditionType};

pub mod agent;
pub mod tasks;
pub mod enums;
pub mod planner;
pub mod condition;


pub struct CurrentWorldState {
    pub state: Vec<u8>,        
}

pub enum WorldState {
    FoodInRange,
    EnemyInRange,
}

pub struct Effect {
    pub state: isize,
    pub effect: u8,
    pub effect_type: EffectType,
}

pub trait Operator {
    fn execute(&self);
}

pub enum Tasks {
    Idle,
    NavigateToFood,
    ConsumeFood,
    Feed__,
}