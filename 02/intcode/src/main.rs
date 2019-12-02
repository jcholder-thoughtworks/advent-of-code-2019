type IntcodeProgram = Vec<i32>;
type Cursor = usize;

fn main() {
}

fn apply_addition(program: IntcodeProgram, cursor: Cursor) -> IntcodeProgram {
    let left_index = program[cursor + 1] as usize;
    let right_index = program[cursor + 2] as usize;
    let destination = program[cursor + 3] as usize;

    let sum = program[left_index] + program[right_index];

    let mut new_program = program.to_vec();

    new_program[destination] = sum;

    new_program
}

fn apply_multiplication(program: IntcodeProgram, cursor: Cursor) -> IntcodeProgram {
    let left_index = program[cursor + 1] as usize;
    let right_index = program[cursor + 2] as usize;
    let destination = program[cursor + 3] as usize;

    let product = program[left_index] * program[right_index];

    let mut new_program = program.to_vec();

    new_program[destination] = product;

    new_program
}

fn execute_intcode(program: IntcodeProgram, cursor: Cursor) -> IntcodeProgram {
    let command = program[cursor];

    if command == 99 {
        return program;
    }

    let program = match command {
        1 => apply_addition(program, cursor),
        2 => apply_multiplication(program, cursor),
        _ => panic!("Unrecognized command: {:?}", command),
    };

    execute_intcode(program, cursor + 4)
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
}
