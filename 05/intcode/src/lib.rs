use std::convert::From;

type Instruction = i32;

#[derive(Debug)]
enum InstructionType {
    Add,
    Multiply,
    StoreInput,
    FetchOutput,
}

type IT = InstructionType;

#[derive(Debug)]
struct Command {
    instruction: Instruction,
    instruction_type: InstructionType,
    parameter_modes: Vec<ParameterMode>,
}

type Intcode = Vec<Instruction>;

type Pointer = usize;

pub type Input = i32;
pub type Output = i32;

pub const OUTPUT: Pointer = 0;
pub const NOUN: Pointer = 1;
pub const VERB: Pointer = 2;

const ADD: Instruction = 1;
const MULTIPLY: Instruction = 2;
const STORE_INPUT: Instruction = 3;
const FETCH_OUTPUT: Instruction = 4;
const HALT: Instruction = 99;

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

type PM = ParameterMode;

impl From<i32> for ParameterMode {
    fn from(opcode_digit: i32) -> Self {
        match opcode_digit {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unrecognized parameter mode code: {}", opcode_digit),
        }
    }
}

impl From<Instruction> for Command {
    fn from(instruction: Instruction) -> Self {
        let instruction_type = match digit_at_place(instruction, 0) {
            ADD => IT::Add,
            MULTIPLY => IT::Multiply,
            STORE_INPUT => IT::StoreInput,
            FETCH_OUTPUT => IT::FetchOutput,
            _ => panic!("Unrecognized instruction {:?}", instruction),
        };

        let parameter_modes = match instruction_type {
            IT::Add => { vec![
                ParameterMode::from(digit_at_place(instruction, 2)),
                ParameterMode::from(digit_at_place(instruction, 3)),
            ] },
            IT::Multiply => { vec![
                ParameterMode::from(digit_at_place(instruction, 2)),
                ParameterMode::from(digit_at_place(instruction, 3)),
            ] },
            IT::StoreInput => vec![ParameterMode::from(digit_at_place(instruction, 2))],
            IT::FetchOutput => vec![ParameterMode::from(digit_at_place(instruction, 2))],
        };

        Self {
            instruction_type,
            instruction,
            parameter_modes,
        }
    }
}

impl Command {
    fn execute_against(&self, program: &mut Program, pointer: Pointer, input: Input) -> Pointer {
        match self.instruction_type {
            IT::Add => self.addition(program, pointer),
            IT::Multiply => self.multiplication(program, pointer),
            IT::StoreInput => self.store_input(program, input, pointer),
            IT::FetchOutput => self.fetch_output(program, pointer),
        }
    }

    fn addition(&self, program: &mut Program, pointer: Pointer) -> Pointer {
        let destination = program.intcode[pointer + 3] as usize;

        let left_value = match self.parameter_modes[0] {
            PM::Position => {
                let left_index = program.intcode[pointer + 1] as usize;
                program.intcode[left_index]
            },
            PM::Immediate => program.intcode[pointer + 1],
        };

        let right_value = match self.parameter_modes[1] {
            PM::Position => {
                let right_index = program.intcode[pointer + 2] as usize;
                program.intcode[right_index]
            },
            PM::Immediate => program.intcode[pointer + 2],
        };

        let new_value = left_value + right_value;

        program.intcode[destination] = new_value;

        pointer + 4
    }

    fn multiplication(&self, program: &mut Program, pointer: Pointer) -> Pointer {
        let destination = program.intcode[pointer + 3] as usize;

        let left_value = match self.parameter_modes[0] {
            PM::Position => {
                let left_index = program.intcode[pointer + 1] as usize;
                program.intcode[left_index]
            },
            PM::Immediate => program.intcode[pointer + 1],
        };

        let right_value = match self.parameter_modes[1] {
            PM::Position => {
                let right_index = program.intcode[pointer + 2] as usize;
                program.intcode[right_index]
            },
            PM::Immediate => program.intcode[pointer + 2],
        };

        let new_value = left_value * right_value;

        program.intcode[destination] = new_value;

        pointer + 4
    }

    fn store_input(&self, program: &mut Program, input: Input, pointer: Pointer) -> Pointer {
        let store_at = program.intcode[pointer + 1] as usize;

        program.intcode[store_at] = input;

        pointer + 2
    }

    fn fetch_output(&self, program: &mut Program, pointer: Pointer) -> Pointer {
        let fetch_from = program.intcode[pointer + 1] as usize;

        program.output = program.intcode[fetch_from];

        pointer + 2
    }
}

#[derive(Debug)]
pub struct Program {
    intcode: Intcode,
    output: Output,
}

fn digit_at_place(source: i32, place: i32) -> i32 {
    let power = i32::pow(10, place as u32);
    ((source - (source % power)) / power) % 10
}

impl From<&str> for Program {
    fn from(code: &str) -> Self {
        let intcode = code.trim().split(',');
        let intcode = intcode.map(|c| c.parse().unwrap()).collect();
        Self::new(intcode)
    }
}

impl Program {
    pub fn new(intcode: Intcode) -> Self {
        Self {
            intcode,
            output: 0,
        }
    }

    pub fn execute(&mut self, input: Input) -> Output {
        self.execute_at_pointer(0, input);

        self.output
    }

    fn execute_at_pointer(&mut self, pointer: Pointer, input: Input) {
        let instruction = self.intcode[pointer];

        if instruction == HALT {
            return;
        }

        let command = Command::from(instruction);

        let new_pointer = command.execute_against(self, pointer, input);

        self.execute_at_pointer(new_pointer, input)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const DEFAULT_INPUT: Input = 0;

    #[test]
    fn example_intcode() {
        let mut program = Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        program.execute(DEFAULT_INPUT);

        assert_eq!(expected, program.intcode);
    }

    #[test]
    fn addition() {
        let mut program = Program::new(vec![1,0,0,0,99]);

        let expected = vec![2,0,0,0,99];

        program.execute(DEFAULT_INPUT);

        assert_eq!(expected, program.intcode);
    }

    #[test]
    fn multiplication() {
        let mut program = Program::new(vec![2,3,0,3,99]);

        let expected = vec![2,3,0,6,99];

        program.execute(DEFAULT_INPUT);

        assert_eq!(expected, program.intcode);
    }

    #[test]
    fn multiplication_and_storage() {
        let mut program = Program::new(vec![2,4,4,5,99,0]);

        let expected = vec![2,4,4,5,99,9801];

        program.execute(DEFAULT_INPUT);

        assert_eq!(expected, program.intcode);
    }

    #[test]
    fn multiple_instructions() {
        let mut program = Program::new(vec![1,1,1,4,99,5,6,0,99]);

        let expected = vec![30,1,1,4,2,5,6,0,99];

        program.execute(DEFAULT_INPUT);

        assert_eq!(expected, program.intcode);
    }

    #[test]
    fn basic_input_output() {
        let mut program = Program::new(vec![3,0,4,0,99]);

        let input = 5;

        let output = program.execute(input);

        assert_eq!(input, output);
    }

    #[test]
    fn parameter_types_addition() {
        let mut program = Program::new(vec![1101,3,4,5,99,0]);

        let expected = vec![1101,3,4,5,99,7];

        program.execute(DEFAULT_INPUT);

        assert_eq!(expected, program.intcode);
    }


    #[test]
    fn parameter_types_multiplication() {
        let mut program = Program::new(vec![1102,3,4,5,99,0]);

        let expected = vec![1102,3,4,5,99,12];

        program.execute(DEFAULT_INPUT);

        assert_eq!(expected, program.intcode);
    }

    #[test]
    fn digit_at_place_basic() {
        assert_eq!(digit_at_place(12345, 0), 5);
        assert_eq!(digit_at_place(12345, 1), 4);
        assert_eq!(digit_at_place(12345, 2), 3);
        assert_eq!(digit_at_place(12345, 3), 2);
        assert_eq!(digit_at_place(12345, 4), 1);
    }

    #[test]
    fn default_to_zero_beyond_last_place() {
        assert_eq!(digit_at_place(12345, 5), 0);
        assert_eq!(digit_at_place(12345, 9), 0);
    }
}
