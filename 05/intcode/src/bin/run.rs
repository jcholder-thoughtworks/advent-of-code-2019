use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;
    let original_program = parse_code(code);

    let input = 0; // example input

    println!("Input: {}", input);

    let (_executed_program, output) = execute_intcode(original_program, input);
    println!("Output: {}", output);

    Ok(())
}
