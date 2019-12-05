use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;
    let mut program = Program::from(code.as_str());

    let input = 1; // run air conditioning diagnostics

    println!("Input: {}", input);

    let output = program.execute(input);

    println!("Output: {}", output);

    Ok(())
}
