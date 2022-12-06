use std::env;
use rum::run_um::{run_um};

fn main() {
    let input = env::args().nth(1);
    run_um(input); 
}
