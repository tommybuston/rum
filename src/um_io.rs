use crate::um_state::{State};

    /// Prints a char from a register
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'rc' - register offset from Field RC
    pub fn print_rc (current_state: &State, rc: u32) {

        match char::from_u32(current_state.registers[rc as usize]) {
            Some(x)=>{
                print!("{}",x);
            }
            None=>{panic!()}
        }

    }

    /// Loads an integer into rA from segmented memory
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'rc' - register offset from Field RC
    /// * 'input' - user provided input byte
    pub fn user_in(current_state: &mut State, rc: u32 ,input: u8) {
        current_state.registers[rc as usize] = input as u32;
    }

    /// Loads a u32 MAX into a register if user input is empty
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'rc' - register offset from Field RC
    pub fn user_in_empty (current_state: &mut State, rc: u32 ) {
        current_state.registers[rc as usize] = u32::MAX;
    }
