use enums::{EffectType, ConditionType};

pub mod agent;
pub mod tasks;
pub mod enums;
pub mod planner;


pub struct CurrentWorldState {
    pub state: Vec<u8>,        
}

pub enum WorldState {

}

pub struct Condition {
    pub state:isize,
    pub value:u8,
    pub con_type:ConditionType,
}

pub struct Effect {
    pub state: isize,
    pub effect: u8,
    pub effect_type: EffectType,
}

pub trait Operator {
    fn execute(&self);
}

