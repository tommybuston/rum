use crate::rumload;

pub struct State {
    pub program_counter: u32,
    pub segment_memory: Vec<Vec<u32>>,
    pub registers: Vec<u32>,
}

impl State {
    pub fn initialize_state(input: Option<String>) -> State {
        let current_state = State { program_counter: 0,
                                    segment_memory: vec![rumload::load(input.as_deref()); 1],
                                    registers: vec![0; 8],
        };
        //current_state.segment_memory.push( rumload::load(input.as_deref()) );
        current_state    
    }

    pub fn get_instructions(&self) -> &Vec<u32> {
        &self.segment_memory[0]
    }

    pub fn get_pc(&mut self) -> &mut u32 {
        &mut self.program_counter
    }
    pub fn get_regs(&mut self) -> &mut Vec<u32> {
        &mut self.registers
    }

}
