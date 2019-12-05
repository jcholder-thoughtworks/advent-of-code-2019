use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;
    let original_intcode = parse_code(code);
    let mut program = Program::new(original_intcode);

    let input = 0; // example input

    println!("Input: {}", input);

    let output = program.execute(input);

    println!("Output: {}", output);

    Ok(())
}
