use crate::{CurrentWorldState, Condition, Effect, tasks::{Method, CompTask}, enums::{Task, ConditionStrictness}};

pub struct JayAgent {
    pub world_state: CurrentWorldState,
    pub conditions: Vec<Condition>,
    pub results: Vec<Effect>,
    pub methods: Vec<Method>,
    pub tasks: Vec<Task>,

    // Builder variables
    parent_id: usize,
}

impl Default for JayAgent {
    fn default() -> Self {
        Self { world_state: CurrentWorldState { state: Vec::new() }, 
            conditions: Vec::new(), 
            results: Vec::new(), 
            methods: Vec::new(), 
            tasks: Vec::new(), 
            parent_id: 0,
        }
    }
}

pub fn test(){
    let mut agent = JayAgent::default();
}

impl JayAgent {
    fn init() -> JayAgent {


        agent
    }
}

pub struct JayAgentBuilder {
    agent:JayAgent,
}

impl JayAgentBuilder {
    fn init_agent() -> JayAgent {
        let mut agent = JayAgent::default();
        agent.tasks.push(Task::Compound(CompTask {
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

    fn 
}

impl Default for JayAgentBuilder {
    fn default() -> Self {
        let agent = JayAgentBuilder::init_agent();
        Self { agent }
    }
}
