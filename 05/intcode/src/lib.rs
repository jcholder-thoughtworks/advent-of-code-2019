use std::convert::From;

pub type Intcode = Vec<i32>;
pub type Pointer = usize;
pub type Input = i32;
pub type Output = i32;

pub const OUTPUT: Pointer = 0;
pub const NOUN: Pointer = 1;
pub const VERB: Pointer = 2;

pub struct Program {
    intcode: Intcode
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
        Self { intcode }
    }

    pub fn execute(&mut self, input: Input) -> Output {
        self.execute_at_pointer(0, input)
    }

    fn execute_at_pointer(&mut self, pointer: Pointer, input: Input) -> Output {
        let command = self.intcode[pointer];

        if command == 99 {
            return 0;
        }

        match command {
            1 => self.perform_add_at(pointer),
            2 => self.perform_multiply_at(pointer),
            _ => panic!("Unrecognized command: {:?}", command),
        };

        self.execute_at_pointer(pointer + 4, input)
    }

    fn perform_add_at(&mut self, pointer: Pointer) {
        let left_index = self.intcode[pointer + 1] as usize;
        let right_index = self.intcode[pointer + 2] as usize;
        let destination = self.intcode[pointer + 3] as usize;

        let left_value = self.intcode[left_index];
        let right_value = self.intcode[right_index];

        let new_value = left_value + right_value;

        self.intcode[destination] = new_value;
    }

    fn perform_multiply_at(&mut self, pointer: Pointer) {
        let left_index = self.intcode[pointer + 1] as usize;
        let right_index = self.intcode[pointer + 2] as usize;
        let destination = self.intcode[pointer + 3] as usize;

        let left_value = self.intcode[left_index];
        let right_value = self.intcode[right_index];

        let new_value = left_value * right_value;

        self.intcode[destination] = new_value;
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
    #[ignore]
    fn basic_input_output() {
        let mut program = Program::new(vec![3,0,4,0,99]);

        let input = 5;

        let output = program.execute(input);

        assert_eq!(input, output);
    }
}
