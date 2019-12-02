type IntcodeProgram = Vec<i32>;

fn main() {
    println!("Hello, world!");
}

fn execute_intcode(program: IntcodeProgram) -> IntcodeProgram {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_program() {
        let program = vec![1, 0, 0, 0, 99];

        let expected = vec![2, 0, 0, 0, 99];

        assert_eq!(expected, execute_intcode(program));
    }

    #[test]
    fn example_program() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(expected, execute_intcode(program));
    }
}
