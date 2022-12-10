use crate::um_state::{State};

    /// If rc is not zero ra gets rb
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'ra' - register offset from Field RA
    /// * 'rb' - register offset from Field RB
    /// * 'rc' - register offset from Field RC
    pub fn cmov(current_state: &mut State, ra: u32, rb: u32, rc: u32 ) {
        if current_state.registers[rc as usize] != 0 {
            current_state.registers[ra as usize] = current_state.registers[rb as usize];
        }
    }

    /// Add rb and rc and store in ra
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'ra' - register offset from Field RA
    /// * 'rb' - register offset from Field RB
    /// * 'rc' - register offset from Field RC
    pub fn add(current_state: &mut State, ra: u32, rb: u32, rc: u32) {
        current_state.registers[ra as usize] = (current_state.registers[rb as usize] as u64 + 
                                                current_state.registers[rc as usize] as u64
                                                % 2_u64.pow(32)) as u32;
    }

    /// multiply rb and rc and store in ra
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'ra' - register offset from Field RA
    /// * 'rb' - register offset from Field RB
    /// * 'rc' - register offset from Field RC
    pub fn mult(current_state: &mut State, ra: u32, rb: u32, rc: u32) {
        current_state.registers[ra as usize] = (current_state.registers[rb as usize] as u64 * 
                                                current_state.registers[rc as usize] as u64
                                                % 2_u64.pow(32)) as u32;
    }

    /// integer divide rb and rc and store in ra
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'ra' - register offset from Field RA
    /// * 'rb' - register offset from Field RB
    /// * 'rc' - register offset from Field RC
    pub fn div(current_state: &mut State, ra: u32, rb: u32, rc: u32) {
        current_state.registers[ra as usize] = current_state.registers[rb as usize] / 
                                                current_state.registers[rc as usize];
    }

    /// nand rb and rc and store in ra
    ///
    /// # Arguments:
    ///
    /// * 'current_state' - state of the UM
    /// * 'ra' - register offset from Field RA
    /// * 'rb' - register offset from Field RB
    /// * 'rc' - register offset from Field RC
    pub fn nand(current_state: &mut State, ra: u32, rb: u32, rc: u32) {
        current_state.registers[ra as usize] = !(current_state.registers[rb as usize] & 
                                                 current_state.registers[rc as usize]);
    }
