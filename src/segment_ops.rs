use crate::um_state::{State};
use std::mem;
    
    /// Loads an integer into rA from segmented memory
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'ra' - register offset from Field RA
    /// * 'rb' - register offset from Field RB
    /// * 'rc' - register offset from Field RC
    pub fn seg_load(current_state: &mut State, ra: u32, rb: u32, rc: u32 ) {
        current_state.registers[ra as usize] = current_state.segment_memory
                                                            [current_state.registers[rb as usize] as usize]
                                                            [current_state.registers[rc as usize] as usize];
    }

    /// Stores a value from rC into segmented memory
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'ra' - register offset from Field RA
    /// * 'rb' - register offset from Field RB
    /// * 'rc' - register offset from Field RC
    pub fn seg_store(current_state: &mut State, ra: u32, rb: u32, rc: u32 ) {
        current_state.segment_memory
                                [current_state.registers[ra as usize] as usize]
                                [current_state.registers[rb as usize] as usize] = 
                                    current_state.registers[rc as usize];
    }

    /// Loads program from segmented memory and reassigns program counter
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'rb' - register offset from Field RB
    /// * 'rc' - register offset from Field RC
    pub fn load_program (current_state: &mut State, rb: u32, rc: u32) {
        current_state.segment_memory[0] =  mem::take(&mut current_state.segment_memory
                                            [current_state.registers[rb as usize] as usize]);
        current_state.program_counter = current_state.registers[rc as usize];
    }
    
    /// Load a value into a register rA
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'ra' - register offset from Field RA
    /// * 'val' - value to load into reg
    pub fn load_value (current_state: &mut State, ra: u32, val: u32) {
        current_state.registers[ra as usize] = val;
    }

    /// Allocate a specified amount of memory to segment
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'rb' - register offset from Field RA
    /// * 'rc' - value to load into reg
    pub fn map_segment (current_state: &mut State, rb: u32, rc: u32) {
        match current_state.unmapped_segments.pop() {
            Some(idx) => {
                //if mapping to a unmapped segment
                current_state.segment_memory[ idx as usize ] = 
                                vec![0_u32; current_state.registers[rc as usize] as usize];
                current_state.registers[rb as usize] = idx;
            }
            None => {
                //if mapping to a new segment
                current_state.segment_memory.push(vec![0_u32; current_state.registers[rc as usize] as usize]);
                current_state.registers[rb as usize] = (current_state.segment_memory.len() - 1) as u32;
            }
        }
    }

    /// Frees a memory segment for future use
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'rc' - value to load into reg
    pub fn unmap_segment (current_state: &mut State, rc: u32) {
        current_state.segment_memory[current_state.registers[rc as usize] as usize].clear();
        current_state.unmapped_segments.push(current_state.registers[rc as usize]);
    }
