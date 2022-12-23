use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::utils::HashMap;
use crate::{CurrentWorldState, Effect, tasks::{Method, CompTask}, enums::{Task, ConditionStrictness}, Tasks, WorldState};
use crate::condition::Condition;

pub struct JayAgent {
    pub world_state: CurrentWorldState,
    pub conditions: Vec<Condition>,
    pub results: Vec<Effect>,
    pub methods: Vec<Method>,
    pub tasks: HashMap<isize, Task>,

    // Builder variables
    parent_id: usize,
}

impl Default for JayAgent {
    fn default() -> Self {
        Self { world_state: CurrentWorldState { state: Vec::new() }, 
            conditions: Vec::new(), 
            results: Vec::new(), 
            methods: Vec::new(), 
            tasks: HashMap::new(), 
            parent_id: 0,
        }
    }
}

pub fn test(){
    let mut agent = JayAgent::default();
}

pub struct JayAgentBuilder {
    agent:JayAgent,
    current_parent_id:usize,
    last_parent_id:usize,
}

impl JayAgentBuilder {
    fn init_agent() -> JayAgent {
        let mut agent = JayAgent::default();
        agent.tasks.insert(0, Task::Compound(CompTask {
            task_id: 0 as usize,
            method_ids: vec![0 as usize],
        }));

        agent.methods.push(Method {
            method_id: 0 as usize,
            strictness: ConditionStrictness::AlwaysPass,
            condition_ids: Vec::new(),
            subtask_ids: Vec::new(),
        });
    
        agent
    }

    pub fn method(&mut self, mut conditions: Vec<Condition>, strictness: ConditionStrictness, subtasks: Vec<isize>) -> &JayAgentBuilder {
        let new_method_id = self.agent.methods.len();
        let condition_ids = self.process_conditions(&mut conditions);
        let mut method = Method {
            method_id: new_method_id,
            strictness,
            condition_ids,
            subtask_ids: vec![],
        };
        
        self
    }
    
    pub fn compound_task(&self, task_id: isize, methods: Vec<Method>) -> &JayAgentBuilder {
        
        self
    }
    
    fn escape(&mut self) -> &JayAgentBuilder {
        self.method(vec![Condition::equal(WorldState::FoodInRange as isize, 1),
                         Condition::less_than(WorldState::EnemyInRange as isize, 1)],
                    ConditionStrictness::All,
                    vec![Tasks::Idle as isize, Tasks::NavigateToFood as isize])
    }
    
    fn process_conditions(&mut self, conditions: &mut Vec<Condition>) -> Vec<usize> {
        let mut result:Vec<usize> = Vec::new();
        // replace conditions with already existing ones if identical.
        // pull out their ids to return.
        let length = self.agent.conditions.len();
        let cond_len = conditions.len().clone();
        let mut added:usize = 0;
        for c in 0..cond_len {
            let candidate = &conditions[c];
            for i in 0..length {
                let existing = &self.agent.conditions[i];
                if candidate.state == existing.state && candidate.con_type == existing.con_type && candidate.value == existing.value {
                    result.push(i);
                } else {
                    added += 1;
                    self.agent.conditions.push(candidate.clone());
                    result.push(cond_len + added);
                }
            }
        }

        
        result
    }
}


impl Default for JayAgentBuilder {
    fn default() -> Self {
        let agent = JayAgentBuilder::init_agent();
        Self { agent, current_parent_id: 0, last_parent_id: 0 }
    }
}
