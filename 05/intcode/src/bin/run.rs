use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;

    diagnose_air_conditioner(code.as_str());

    diagnose_thermal_radiators(code.as_str());

    Ok(())
}

fn diagnose_air_conditioner(code: &str) {
    let mut program = Program::from(code);

    println!("Diagnosing air conditioner ...");

    let output = program.execute(1);

    println!("Output: {}", output);
}

fn diagnose_thermal_radiators(code: &str) {
    let mut program = Program::from(code);

    println!("Diagnosing thermal radiators ...");

    let output = program.execute(5);

    println!("Output: {}", output);
}
