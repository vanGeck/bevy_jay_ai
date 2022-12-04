use crate::{tasks::PrimitiveTask, CurrentWorldState};

pub fn create_plan(){
    let mut tasks_to_process:Vec<PrimitiveTask> = Vec::new();
    let mut plan:Vec<PrimitiveTask> = Vec::new();
    let mut working_state = CurrentWorldState::new();
    // let mut domain: Domain;

    // Push toplevel compound task from the domain onto the processing stack.

    // Iterate:
    // Pop a task off the stack.
    // IF it's a COMPOUND task:
    //  - Iterate through the task methods and find the first one that fulfills conditions
    //  - If a method is found, add it's subtasks onto the stack.
    //  - If a method is not found, roll planner's state back to the last compound task successfully processed.
    // IF it's a PRIMITIVE task:
    //  - If it's conditions are met, then add it to the plan.
    //  - If they are not, roll the state back to last compound task like before.

}