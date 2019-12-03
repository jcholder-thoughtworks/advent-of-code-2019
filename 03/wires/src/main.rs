fn main() {
    println!("Hello, world!");
}

enum WireDirection {
    U(i32), // Up
    R(i32), // Right
    D(i32), // Down
    L(i32), // Left
}

type WD = WireDirection;

type Wire = Vec<WireDirection>;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn minimal_program() {
        let wire1 = [WD::R(10), WD::U(10)];
        let wire2 = [WD::U(5), WD::R(15)];

        assert_eq!(closest_intersection_distance(wire1, wire2), 15);
    }
}
