fn main() {
    println!("Hello, world!");
}

type Distance = i32;

enum WireDirection {
    U(i32), // Up
    R(i32), // Right
    D(i32), // Down
    L(i32), // Left
}

type WD = WireDirection;

type Wire = Vec<WireDirection>;

fn closest_intersection_distance(wire1: &Wire, wire2: &Wire) -> Distance {
    0
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn minimal_program() {
        let wire1 = vec![WD::R(10), WD::U(10)];
        let wire2 = vec![WD::U(5), WD::R(15)];

        assert_eq!(closest_intersection_distance(&wire1, &wire2), 15);
    }
}
