type IntcodeProgram = Vec<i32>;

fn main() {
    println!("Hello, world!");
}

fn execute_intcode(ints: IntcodeProgram) -> IntcodeProgram {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_program() {
        let program = vec![1, 0, 0, 0, 99];

        let expected = vec![2, 0, 0, 0, 99];
        let actual = execute_intcode(program);

        assert_eq!(expected, actual);
    }
}
