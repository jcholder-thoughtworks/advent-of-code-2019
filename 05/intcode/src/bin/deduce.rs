use std::fs;
use std::io::{self};

use intcode::*;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("input.txt")?;
    let original_program = parse_code(code);

    let (noun, verb) = deduce_required_inputs(original_program.to_vec());
    let combination = 100 * noun + verb;

    println!("Solution found: {:1}, {:2}", noun, verb);
    println!("Combination: {}", combination);

    Ok(())
}

fn deduce_required_inputs(original_program: IntcodeProgram) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut trial_program = original_program.to_vec();
            trial_program[NOUN] = noun;
            trial_program[VERB] = verb;

            let executed = execute_intcode(trial_program);

            if executed[OUTPUT] == 19690720 {
                return (executed[NOUN], executed[VERB])
            }
        }
    }

    panic!("Solution not found");
}
