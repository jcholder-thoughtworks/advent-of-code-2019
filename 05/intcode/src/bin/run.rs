use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;
    let original_intcode = parse_code(code);

    let input = 0; // example input

    println!("Input: {}", input);

    let (_executed_intcode, output) = execute_intcode(original_intcode, input);
    println!("Output: {}", output);

    Ok(())
}
