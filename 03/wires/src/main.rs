fn main() {
    println!("Hello, world!");
}

type Distance = i32;

struct Point {
    x: i32,
    y: i32,
}

struct Segment(Point, Point);

enum WireDirection {
    U(i32), // Up
    R(i32), // Right
    D(i32), // Down
    L(i32), // Left
}

type WD = WireDirection;

type WireInstructions = Vec<WireDirection>;

fn closest_intersection_distance(wire1: &WireInstructions, wire2: &WireInstructions) -> Distance {
    0
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn minimal_program() {
        let wire_instructions1 = vec![WD::R(10), WD::U(10)];
        let wire_instructions2 = vec![WD::U(5), WD::R(15)];

        assert_eq!(closest_intersection_distance(&wire_instructions1, &wire_instructions2), 15);
    }
}
