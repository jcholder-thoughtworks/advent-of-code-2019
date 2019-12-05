pub type Intcode = Vec<i32>;
pub type Pointer = usize;
pub type Input = i32;
pub type Output = i32;

pub const OUTPUT: Pointer = 0;
pub const NOUN: Pointer = 1;
pub const VERB: Pointer = 2;

pub fn parse_code(code: String) -> Intcode {
    let intcode = code.trim().split(',');
    intcode.map(|c| c.parse().unwrap()).collect()
}

pub fn execute_intcode(intcode: Intcode, input: Input) -> (Intcode, Output) {
    execute_intcode_at_pointer(intcode, 0, input)
}

fn execute_intcode_at_pointer(mut intcode: Intcode, pointer: Pointer, input: Input) -> (Intcode, Output) {
    let command = intcode[pointer];

    if command == 99 {
        return (intcode, 0);
    }

    match command {
        1 => perform_add(&mut intcode, pointer),
        2 => perform_multiply(&mut intcode, pointer),
        _ => panic!("Unrecognized command: {:?}", command),
    };

    execute_intcode_at_pointer(intcode, pointer + 4, input)
}

fn perform_add(intcode: &mut Intcode, pointer: Pointer) {
    let left_index = intcode[pointer + 1] as usize;
    let right_index = intcode[pointer + 2] as usize;
    let destination = intcode[pointer + 3] as usize;

    let left_value = intcode[left_index];
    let right_value = intcode[right_index];

    let new_value = left_value + right_value;

    intcode[destination] = new_value;
}

fn perform_multiply(intcode: &mut Intcode, pointer: Pointer) {
    let left_index = intcode[pointer + 1] as usize;
    let right_index = intcode[pointer + 2] as usize;
    let destination = intcode[pointer + 3] as usize;

    let left_value = intcode[left_index];
    let right_value = intcode[right_index];

    let new_value = left_value * right_value;

    intcode[destination] = new_value;
}


#[cfg(test)]
pub mod tests {
    use super::*;

    const DEFAULT_INPUT: Input = 0;

    #[test]
    fn minimal_intcode() {
        let intcode = vec![1, 0, 0, 0, 99];

        let expected = vec![2, 0, 0, 0, 99];

        let (executed_intcode, _output) = execute_intcode(intcode, DEFAULT_INPUT);

        assert_eq!(expected, executed_intcode);
    }

    #[test]
    fn example_intcode() {
        let intcode = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        let (executed_intcode, _output) = execute_intcode(intcode, DEFAULT_INPUT);

        assert_eq!(expected, executed_intcode);
    }

    #[test]
    fn small_intcode_1() {
        let intcode = vec![1,0,0,0,99];

        let expected = vec![2,0,0,0,99];

        let (executed_intcode, _output) = execute_intcode(intcode, DEFAULT_INPUT);

        assert_eq!(expected, executed_intcode);
    }

    #[test]
    fn small_intcode_2() {
        let intcode = vec![2,3,0,3,99];

        let expected = vec![2,3,0,6,99];

        let (executed_intcode, _output) = execute_intcode(intcode, DEFAULT_INPUT);

        assert_eq!(expected, executed_intcode);
    }

    #[test]
    fn small_intcode_3() {
        let intcode = vec![2,4,4,5,99,0];

        let expected = vec![2,4,4,5,99,9801];

        let (executed_intcode, _output) = execute_intcode(intcode, DEFAULT_INPUT);

        assert_eq!(expected, executed_intcode);
    }

    #[test]
    fn small_intcode_4() {
        let intcode = vec![1,1,1,4,99,5,6,0,99];

        let expected = vec![30,1,1,4,2,5,6,0,99];

        let (executed_intcode, _output) = execute_intcode(intcode, DEFAULT_INPUT);

        assert_eq!(expected, executed_intcode);
    }

    #[test]
    #[ignore]
    fn basic_input_output() {
        let intcode = vec![3,0,4,0,99];

        let input = 5;

        let (_, output) = execute_intcode(intcode, input);

        assert_eq!(input, output);
    }
}
