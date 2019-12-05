use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;
    let program = parse_code(code);

    let original_output = execute_intcode(program.to_vec());
    let fixed_output = execute_intcode(apply_fix(program.to_vec()));

    println!("Input: {:?}", program);
    println!("Original output: {:?}", original_output);
    println!("Fixed output: {:?}", fixed_output);

    Ok(())
}

fn apply_fix(mut program: IntcodeProgram) -> IntcodeProgram {
    program[NOUN] = 12;
    program[VERB] = 2;

    program
}

