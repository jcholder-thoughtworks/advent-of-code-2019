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

type WireDirections = Vec<WireDirection>;

fn closest_intersection_distance(
    wire_directions1: &WireDirections,
    wire_directions2: &WireDirections
    ) -> Distance {

    let wire1 = wire_from_directions(wire_directions1);
    let wire2 = wire_from_directions(wire_directions2);

    println!("Test output: {:?}", wire1);
    println!("Test output: {:?}", wire2);

    //let intersection_points = intersection_points_between(wire1, wire2);

    0
}

fn wire_from_directions(wire_directions: &WireDirections) -> Wire {
    let mut wire: Wire = vec![];

    let mut x = 0;
    let mut y = 0;

    for direction in wire_directions {
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
        let wire_directions1 = vec![WD::R(10), WD::U(10)];
        let wire_directions2 = vec![WD::U(5), WD::R(15)];

        assert_eq!(closest_intersection_distance(&wire_directions1, &wire_directions2), 15);
    }
}
