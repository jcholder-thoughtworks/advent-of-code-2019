use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;
    let original_program = parse_code(code);

    println!("Program post execution:");
    println!("{:?}", execute_intcode(original_program));

    Ok(())
}
