use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;
    let program = parse_code(code);

    let original_program = execute_intcode(program.to_vec());

    println!("Input: {:?}", program);
    println!("Original output: {:?}", original_program);

    Ok(())
}
