use std::fs;
use std::io::{self};

type IntcodeProgram = Vec<i32>;
type Pointer = usize;

fn main() -> io::Result<()> {
    let program = fs::read_to_string("input.txt")?;
    let program = program.trim().split(',');
    let program: IntcodeProgram = program.map(|c| c.parse().unwrap()).collect();

    let original_program = program.to_vec();
    let fixed_program = apply_fix(program.to_vec());

    println!("Input: {:?}", program);
    println!("Original output: {:?}", execute_intcode(original_program, 0));
    println!("Fixed output: {:?}", execute_intcode(fixed_program, 0));

    Ok(())
}

fn apply_fix(mut program: IntcodeProgram) -> IntcodeProgram {
    program[1] = 12;
    program[2] = 2;

    program
}

fn execute_intcode(mut program: IntcodeProgram, pointer: Pointer) -> IntcodeProgram {
    let command = program[pointer];

    if command == 99 {
        return program;
    }

    let left_index = program[pointer + 1] as usize;
    let right_index = program[pointer + 2] as usize;
    let destination = program[pointer + 3] as usize;

    let left_value = program[left_index];
    let right_value = program[right_index];

    let new_value = match command {
        1 => left_value + right_value,
        2 => left_value * right_value,
        _ => panic!("Unrecognized command: {:?}", command),
    };

    program[destination] = new_value;

    execute_intcode(program, pointer + 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_program() {
        let program = vec![1, 0, 0, 0, 99];

        let expected = vec![2, 0, 0, 0, 99];

        assert_eq!(expected, execute_intcode(program, 0));
    }

    #[test]
    fn example_program() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(expected, execute_intcode(program, 0));
    }

    #[test]
    fn small_program_1() {
        let program = vec![1,0,0,0,99];

        let expected = vec![2,0,0,0,99];

        assert_eq!(expected, execute_intcode(program, 0));
    }

    #[test]
    fn small_program_2() {
        let program = vec![2,3,0,3,99];

        let expected = vec![2,3,0,6,99];

        assert_eq!(expected, execute_intcode(program, 0));
    }

    #[test]
    fn small_program_3() {
        let program = vec![2,4,4,5,99,0];

        let expected = vec![2,4,4,5,99,9801];

        assert_eq!(expected, execute_intcode(program, 0));
    }

    #[test]
    fn small_program_4() {
        let program = vec![1,1,1,4,99,5,6,0,99];

        let expected = vec![30,1,1,4,2,5,6,0,99];

        assert_eq!(expected, execute_intcode(program, 0));
    }
}
