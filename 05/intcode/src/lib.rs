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
    parameters: Vec<Parameter>,
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

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
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

        Self {
            instruction_type,
            instruction,
            parameters: vec![],
        }
    }
}

impl Command {
    fn execute_against(&self, program: &mut Program, pointer: Pointer, input: Input) -> Pointer {
        match self.instruction_type {
            IT::Add => program.perform_add_at(pointer),
            IT::Multiply => program.perform_multiply_at(pointer),
            IT::StoreInput => program.store_input(input, pointer),
            IT::FetchOutput => program.fetch_output(pointer),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    intcode: Intcode,
    output: Output,
}

/*
fn parameters_for(instruction: Instruction, raw_value: i32) -> Vec<Parameter> {
    let mut params = vec![];

    let param_count = match instruction {
        1 => 3,
        2 => 3,
        3 => 1,
        4 => 1,
        _ => panic!("Unrecognized instruction {:?}", instruction),
    };

    for index in 1..=param_count {
        let mode_code = digit_at_place(raw_value, index);
        let mode = match mode_code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unrecognized parameter mode code {} in instruction {:?}", mode_code, instruction),
        };
        let param = Parameter {
            mode,
            value: 0, // todo: re-assign later
        };
        params.push(param);
    }

    params
}
*/

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

    fn store_input(&mut self, input: Input, pointer: Pointer) -> Pointer {
        let store_at = self.intcode[pointer + 1] as usize;

        self.intcode[store_at] = input;

        pointer + 2
    }

    fn fetch_output(&mut self, pointer: Pointer) -> Pointer {
        let fetch_from = self.intcode[pointer + 1] as usize;

        self.output = self.intcode[fetch_from];

        pointer + 2
    }

    fn perform_add_at(&mut self, pointer: Pointer) -> Pointer {
        let left_index = self.intcode[pointer + 1] as usize;
        let right_index = self.intcode[pointer + 2] as usize;
        let destination = self.intcode[pointer + 3] as usize;

        let left_value = self.intcode[left_index];
        let right_value = self.intcode[right_index];

        let new_value = left_value + right_value;

        self.intcode[destination] = new_value;

        pointer + 4
    }

    fn perform_multiply_at(&mut self, pointer: Pointer) -> Pointer {
        let left_index = self.intcode[pointer + 1] as usize;
        let right_index = self.intcode[pointer + 2] as usize;
        let destination = self.intcode[pointer + 3] as usize;

        let left_value = self.intcode[left_index];
        let right_value = self.intcode[right_index];

        let new_value = left_value * right_value;

        self.intcode[destination] = new_value;

        pointer + 4
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
