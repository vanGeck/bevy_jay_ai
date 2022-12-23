use crate::enums::ConditionType;

pub struct Condition {
    pub state:isize,
    pub value:u8,
    pub con_type:ConditionType,
}

impl Clone for Condition {
    fn clone(&self) -> Condition {
        Condition {
            state: self.state.clone(),
            value: self.value.clone(),
            con_type: self.con_type.clone(),
        }
    }
}

impl Condition {
    pub fn equal(state:isize, value:u8) -> Condition {
        Condition{
            state,
            value,
            con_type: ConditionType::Equal,
        }
    }
    
    pub fn not_equal(state: isize, value:u8) -> Condition {
        Condition{
            state,
            value,
            con_type: ConditionType::NotEqual,
        }
    }
    
    pub fn greater_than(state: isize, value: u8) -> Condition {
        Condition{
            state,
            value,
            con_type: ConditionType::GreaterThan,
        }
    }
    
    pub fn greater_or_equal(state: isize, value: u8) -> Condition {
        Condition {
            state,
            value,
            con_type: ConditionType::GreaterOrEqual,
        }
    }
    
    pub fn less_than(state: isize, value: u8) -> Condition {
        Condition {
            state,
            value,
            con_type: ConditionType::LessThan,
        }
    }
    
    pub fn less_or_equal(state: isize, value: u8) -> Condition {
        Condition {
            state,
            value,
            con_type: ConditionType::LessOrEqual,
        }
    }
}
