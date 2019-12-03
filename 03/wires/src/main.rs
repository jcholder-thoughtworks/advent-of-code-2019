use std::convert::From;

fn main() {
    println!("Hello, world!");
}

type Distance = i32;

#[derive(Clone,Copy,Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment(Point, Point);

#[derive(Debug)]
struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    pub fn new() -> Self {
        Self { segments: vec![] }
    }

    pub fn closest_intersection_distance_with(&self, wire2: &Wire) -> Distance {
        let wire1 = self;

        let intersection_points = intersection_points_between(&wire1, &wire2);

        let distances_per_point: Vec<DistancePerPoint> = intersection_points.iter().map(|point| {
            let distance = point.x.abs() + point.y.abs();
            DistancePerPoint { distance, point: *point }
        }).collect();

        let closest_distance_per_point = distances_per_point.iter().min_by(|a, b| {
            a.distance.cmp(&b.distance)
        }).unwrap();

        closest_distance_per_point.distance
    }
}

impl From<String> for Wire {
    fn from(source: String) -> Self {
        Wire::new()
    }
}

enum WireDirection {
    U(i32), // Up
    R(i32), // Right
    D(i32), // Down
    L(i32), // Left
}

type WD = WireDirection;

type WireDirections = Vec<WireDirection>;

#[derive(Debug)]
struct DistancePerPoint {
    distance: i32,
    point: Point,
}

fn intersection_points_between(wire1: &Wire, wire2: &Wire) -> Vec<Point> {
    let mut intersection_points = vec![];

    // Skipping origin point intersection
    for segment1 in wire1.segments.iter().skip(1) {
        for segment2 in wire2.segments.iter() {
            let too_high_0 = segment1.0.x > segment2.0.x && segment1.0.x > segment2.1.x;
            let too_low_0 = segment1.0.x < segment2.0.x && segment1.0.x < segment2.1.x;
            let too_left_0 = segment1.0.y > segment2.0.y && segment1.0.y > segment2.1.y;
            let too_right_0 = segment1.0.y < segment2.0.y && segment1.0.y < segment2.1.y;

            let too_high_1 = segment1.1.x > segment2.0.x && segment1.1.x > segment2.1.x;
            let too_low_1 = segment1.1.x < segment2.0.x && segment1.1.x < segment2.1.x;
            let too_left_1 = segment1.1.y > segment2.0.y && segment1.1.y > segment2.1.y;
            let too_right_1 = segment1.1.y < segment2.0.y && segment1.1.y < segment2.1.y;

            let too_high = too_high_0 && too_high_1;
            let too_low = too_low_0 && too_low_1;
            let too_left = too_left_0 && too_left_1;
            let too_right = too_right_0 && too_right_1;

            if too_high || too_low || too_left || too_right {
                continue;
            }

            let x;
            let y;

            if segment1.0.x == segment1.1.x { // assume vertical line
                x = segment1.0.x;
                y = segment2.0.y;
            } else { // assume horizontal line
                x = segment2.0.x;
                y = segment1.0.y;
            }

            intersection_points.push(Point { x, y });
        }
    }

    intersection_points
}

fn wire_from_directions(wire_directions: &WireDirections) -> Wire {
    let mut wire = Wire::new();

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

        wire.segments.push(new_segment);
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

        let wire1 = wire_from_directions(&wire_directions1);
        let wire2 = wire_from_directions(&wire_directions2);

        assert_eq!(wire1.closest_intersection_distance_with(&wire2), 15);
    }

    /*
    #[test]
    fn example1() {
        R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83
    }
    */
}
