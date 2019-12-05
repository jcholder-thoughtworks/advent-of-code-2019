pub type IntcodeProgram = Vec<i32>;
pub type Pointer = usize;
pub type Input = i32;
pub type Output = i32;

pub const OUTPUT: Pointer = 0;
pub const NOUN: Pointer = 1;
pub const VERB: Pointer = 2;

pub fn parse_code(code: String) -> IntcodeProgram {
    let program = code.trim().split(',');
    program.map(|c| c.parse().unwrap()).collect()
}

pub fn execute_intcode(program: IntcodeProgram, input: Input) -> (IntcodeProgram, Output) {
    execute_intcode_at_pointer(program, 0, input)
}

fn execute_intcode_at_pointer(mut program: IntcodeProgram, pointer: Pointer, input: Input) -> (IntcodeProgram, Output) {
    let command = program[pointer];

    if command == 99 {
        return (program, 0);
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

    execute_intcode_at_pointer(program, pointer + 4, input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const DEFAULT_INPUT: Input = 0;

    #[test]
    fn minimal_program() {
        let program = vec![1, 0, 0, 0, 99];

        let expected = vec![2, 0, 0, 0, 99];

        let (executed_program, _output) = execute_intcode(program, DEFAULT_INPUT);

        assert_eq!(expected, executed_program);
    }

    #[test]
    fn example_program() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        let (executed_program, _output) = execute_intcode(program, DEFAULT_INPUT);

        assert_eq!(expected, executed_program);
    }

    #[test]
    fn small_program_1() {
        let program = vec![1,0,0,0,99];

        let expected = vec![2,0,0,0,99];

        let (executed_program, _output) = execute_intcode(program, DEFAULT_INPUT);

        assert_eq!(expected, executed_program);
    }

    #[test]
    fn small_program_2() {
        let program = vec![2,3,0,3,99];

        let expected = vec![2,3,0,6,99];

        let (executed_program, _output) = execute_intcode(program, DEFAULT_INPUT);

        assert_eq!(expected, executed_program);
    }

    #[test]
    fn small_program_3() {
        let program = vec![2,4,4,5,99,0];

        let expected = vec![2,4,4,5,99,9801];

        let (executed_program, _output) = execute_intcode(program, DEFAULT_INPUT);

        assert_eq!(expected, executed_program);
    }

    #[test]
    fn small_program_4() {
        let program = vec![1,1,1,4,99,5,6,0,99];

        let expected = vec![30,1,1,4,2,5,6,0,99];

        let (executed_program, _output) = execute_intcode(program, DEFAULT_INPUT);

        assert_eq!(expected, executed_program);
    }

    #[test]
    fn basic_input_output() {
        let program = vec![3,0,4,0,99];

        let input = 5;

        let (_, output) = execute_intcode(program, input);

        assert_eq!(input, output);
    }
}
