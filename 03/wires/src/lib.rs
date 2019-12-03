use std::convert::From;

type Distance = i32;

#[derive(Clone,Copy,Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
    length: Distance,
}

#[derive(Debug)]
pub struct Wire {
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

    pub fn shortest_circuit_distance_with(&self, wire2: &Wire) -> Distance {
        0
    }
}

impl From<&str> for Wire {
    fn from(source: &str) -> Self {
        let directions: WireDirections = source.trim().split(',').map(|d| {
            let direction = d.chars().next();
            let length: i32 = d[1..].parse().unwrap();

            match direction {
                Some('U') => WD::U(length),
                Some('R') => WD::R(length),
                Some('D') => WD::D(length),
                Some('L') => WD::L(length),
                _ => panic!("Invalid wire length: {}", d),
            }
        }).collect();

        let mut wire = Wire::new();

        let mut x = 0;
        let mut y = 0;

        for direction in directions {
            let start = Point { x, y };
            let end = match direction {
                WD::U(distance) => Point { x, y: y + distance },
                WD::R(distance) => Point { x: x + distance, y },
                WD::D(distance) => Point { x, y: y - distance },
                WD::L(distance) => Point { x: x - distance, y },
            };
            let length = match direction {
                WD::U(distance) => distance,
                WD::R(distance) => distance,
                WD::D(distance) => distance,
                WD::L(distance) => distance,
            };

            x = end.x;
            y = end.y;

            let new_segment = Segment {
                start,
                end,
                length,
            };

            wire.segments.push(new_segment);
        }

        wire
    }
}

#[derive(Debug)]
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
            let too_high_0 = segment1.start.x > segment2.start.x && segment1.start.x > segment2.end.x;
            let too_low_0 = segment1.start.x < segment2.start.x && segment1.start.x < segment2.end.x;
            let too_left_0 = segment1.start.y > segment2.start.y && segment1.start.y > segment2.end.y;
            let too_right_0 = segment1.start.y < segment2.start.y && segment1.start.y < segment2.end.y;

            let too_high_1 = segment1.end.x > segment2.start.x && segment1.end.x > segment2.end.x;
            let too_low_1 = segment1.end.x < segment2.start.x && segment1.end.x < segment2.end.x;
            let too_left_1 = segment1.end.y > segment2.start.y && segment1.end.y > segment2.end.y;
            let too_right_1 = segment1.end.y < segment2.start.y && segment1.end.y < segment2.end.y;

            let too_high = too_high_0 && too_high_1;
            let too_low = too_low_0 && too_low_1;
            let too_left = too_left_0 && too_left_1;
            let too_right = too_right_0 && too_right_1;

            if too_high || too_low || too_left || too_right {
                continue;
            }

            let x;
            let y;

            if segment1.start.x == segment1.end.x { // assume vertical line
                x = segment1.start.x;
                y = segment2.start.y;
            } else { // assume horizontal line
                x = segment2.start.x;
                y = segment1.start.y;
            }

            intersection_points.push(Point { x, y });
        }
    }

    intersection_points
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub mod closest_intersection {
        use super::*;

        #[test]
        fn minimal_program() {
            let wire1 = Wire::from("R10,U10");
            let wire2 = Wire::from("U5,R15");

            assert_eq!(wire1.closest_intersection_distance_with(&wire2), 15);
        }

        #[test]
        fn example1() {
            let wire1 = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
            let wire2 = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83");

            assert_eq!(wire1.closest_intersection_distance_with(&wire2), 159);
        }

        #[test]
        fn example2() {
            let wire1 = Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
            let wire2 = Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

            assert_eq!(wire1.closest_intersection_distance_with(&wire2), 135);
        }
    }

    pub mod shortest_intersection {
        use super::*;

        #[test]
        fn minimal_program() {
            let wire1 = Wire::from("R10,U10");
            let wire2 = Wire::from("U5,R15");

            assert_eq!(wire1.shortest_circuit_distance_with(&wire2), 40);
        }

        #[test]
        fn example1() {
            let wire1 = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
            let wire2 = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83");

            assert_eq!(wire1.shortest_circuit_distance_with(&wire2), 610);
        }

        #[test]
        fn example2() {
            let wire1 = Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
            let wire2 = Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

            assert_eq!(wire1.shortest_circuit_distance_with(&wire2), 410);
        }
    }
}
