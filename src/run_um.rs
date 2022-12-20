use crate::um_state::{State};
use std::io;
use std::io::Read;
use crate::um_io::{print_rc, user_in, user_in_empty};
//use crate::reg_math::{cmov, add, mult, div, nand};
use crate::segment_ops::{seg_load,seg_store,load_program,load_value,map_segment,unmap_segment};

pub struct Field { 
    width: u32,
    lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6}; 
static RB: Field = Field {width: 3, lsb: 3}; 
static RC: Field = Field {width: 3, lsb: 0}; 
static RL: Field = Field {width: 3, lsb: 25}; 
static VL: Field = Field {width: 25, lsb: 0}; 
static OP: Field = Field {width: 4, lsb: 28};
static MOD_NUM: u64 = 2_u64.pow(32);

/// Protects the desired word from being erased in get
///
/// # Arguments:
///
/// * 'bits' - width of the desired field
fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

///Retrieves a word from a given integer at a specified bit
///
/// # Arguments:
///
/// * 'field' - specifies width and lsb
/// * 'instruction' - integer to be extracted from
pub fn get(field: &Field, instruction: u32) -> u32 { 
    (instruction >> field.lsb) & mask(field.width)
}

/// Provides the runtime for the UM and processes instruction words
///
/// # Arguments:
///
/// * 'input' - input file to load the program into state 
pub fn run_um(input: Option<String>){

    let mut current_state = State::initialize_state(input);
    
    loop {
        let instruction = current_state.get_instruction();
        let current_op = get(&OP, instruction);
        match current_op {
            //cmov
            0 => {
                //cmov(&mut current_state, get(&RA, instruction), get(&RB, instruction), get(&RC, instruction))
                if current_state.registers[get(&RC, instruction) as usize] != 0 {
                        current_state.registers[get(&RA, instruction) as usize] = 
                                        current_state.registers[get(&RB, instruction) as usize];
                }
            }
            //seg load
            1 => {
                seg_load(&mut current_state, get(&RA, instruction), get(&RB, instruction), get(&RC, instruction) );
            }
            2 => {
                seg_store(&mut current_state, get(&RA, instruction), get(&RB, instruction), 
                                        get(&RC, instruction) );
            }
            3 => {
                //add(&mut current_state, get(&RA, instruction), get(&RB, instruction), get(&RC, instruction) );
                current_state.registers[get(&RA, instruction) as usize] = 
                                                (current_state.registers[get(&RB, instruction) as usize] as u64 + 
                                                current_state.registers[get(&RC, instruction) as usize] as u64
                                                % MOD_NUM) as u32;
            }
            4 => {
                //mult(&mut current_state, get(&RA, instruction), get(&RB, instruction), get(&RC, instruction) );
                current_state.registers[get(&RA, instruction) as usize] = 
                                                (current_state.registers[get(&RB, instruction) as usize] as u64 * 
                                                current_state.registers[get(&RC, instruction) as usize] as u64
                                                % MOD_NUM) as u32;
            }
            5 => {
                //div(&mut current_state, get(&RA, instruction), get(&RB, instruction), get(&RC, instruction) );
                current_state.registers[get(&RA, instruction) as usize] = 
                                                current_state.registers[get(&RB, instruction) as usize] / 
                                                current_state.registers[get(&RC, instruction) as usize];
            }
            6 => {
                //nand(&mut current_state, get(&RA, instruction), get(&RB, instruction), get(&RC, instruction) );
                current_state.registers[get(&RA, instruction) as usize] = 
                                                !(current_state.registers[get(&RB, instruction) as usize] & 
                                                 current_state.registers[get(&RC, instruction) as usize]);
            }
            7 => {
                break;
            }
            8 => {
                map_segment(&mut current_state, get(&RB, instruction), get(&RC, instruction) );
            }
            9 => {
                unmap_segment(&mut current_state, get(&RC, instruction) );
            }
            10 => {
                print_rc( &current_state, get(&RC, instruction) );
            }
            11 => {
                let mut buffer: [u8; 1] = [0; 1];
                let var = io::stdin().read(&mut buffer);

                match var {
                    Ok(0) => {
                        user_in_empty(&mut current_state, get(&RC, instruction) );
                    }
                    Ok(_) => {
                        user_in(&mut current_state, get(&RC, instruction), buffer[0] );
                    }
                    Err(_) => {
                        panic!();
                    }
                }

            }
            12 => {
                load_program(&mut current_state, get(&RB, instruction), get(&RC, instruction));
            }
            13 => {
                load_value(&mut current_state, get(&RL, instruction), get(&VL, instruction));
            }
            _ => { panic!() }            

        }
        if current_op != 12 {
            current_state.program_counter += 1;
        }

    } 

}
