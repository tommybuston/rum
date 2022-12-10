use crate::rumload;

/// A representation of UM's state
pub struct State {
    pub program_counter: u32,
    pub segment_memory: Vec<Vec<u32>>,
    pub registers: Vec<u32>,
    pub unmapped_segments: Vec<u32>,
}


impl State {
    /// Constructor for State and sets UM up to run
    ///
    /// # Arguments:
    ///
    /// * 'input' - the binary UM filename
    pub fn initialize_state(input: Option<String>) -> State {
        let current_state = State { program_counter: 0,
                                    segment_memory: vec![rumload::load(input.as_deref()); 1],
                                    registers: vec![0; 8],
                                    unmapped_segments: Vec::new(),
        };
        current_state    
    }
    
    /// Returns an instruction word at the offset specified by program_counter
    pub fn get_instruction(&self) -> u32 {
        self.segment_memory[0][self.program_counter as usize]
    }

}
