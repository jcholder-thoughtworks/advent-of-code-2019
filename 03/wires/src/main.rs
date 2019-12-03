fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn minimal_program() {
        let wire1 = ["R10", "U10"];
        let wire2 = ["U5", "R15"];

        assert_eq!(closest_intersection_distance(wire1, wire2), 15);
    }
}
