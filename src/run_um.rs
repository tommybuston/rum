use crate::um_state::{State};

pub fn run_um(input: Option<String>){

    //let instructions = rumload::load(input.as_deref());

    let mut current_state = State::initialize_state(input); 
    
    for instruction in current_state.get_instructions() {

    } 

}
