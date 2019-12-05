use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;

    diagnose_air_conditioner(code);

    Ok(())
}

fn diagnose_air_conditioner(code: String) {
    let mut program = Program::from(code.as_str());

    println!("Diagnosing air conditioner ...");

    let output = program.execute(1);

    println!("Output: {}", output);
}
