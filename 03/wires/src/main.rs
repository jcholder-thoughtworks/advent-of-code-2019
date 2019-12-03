fn main() {
    println!("Hello, world!");
}

type Distance = i32;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment(Point, Point);

type Wire = Vec<Segment>;

enum WireDirection {
    U(i32), // Up
    R(i32), // Right
    D(i32), // Down
    L(i32), // Left
}

type WD = WireDirection;

type WireInstructions = Vec<WireDirection>;

fn closest_intersection_distance(
    wire_instructions1: &WireInstructions,
    wire_instructions2: &WireInstructions
    ) -> Distance {

    println!("Test output: {:?}", instructions_to_wire(wire_instructions1));
    println!("Test output: {:?}", instructions_to_wire(wire_instructions2));

    0
}

fn instructions_to_wire(wire_instructions: &WireInstructions) -> Wire {
    let mut wire: Wire = vec![];

    let mut x = 0;
    let mut y = 0;

    for direction in wire_instructions {
        let start = Point { x, y };
        let end = match direction {
            WD::U(distance) => Point { x, y: y + distance },
            WD::R(distance) => Point { x: x + distance, y },
            WD::D(distance) => Point { x, y: y - distance },
            WD::L(distance) => Point { x: x - distance, y },
        };

        x = end.x;
        y = end.y;

        let new_segment = Segment(start, end);

        wire.push(new_segment);
    }

    wire
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
