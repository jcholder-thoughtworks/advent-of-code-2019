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

type Wire = Vec<Segment>;

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

fn closest_intersection_distance(
    wire_directions1: &WireDirections,
    wire_directions2: &WireDirections
    ) -> Distance {

    let wire1 = wire_from_directions(wire_directions1);
    let wire2 = wire_from_directions(wire_directions2);

    println!("Wire 1: {:?}", wire1);
    println!("Wire 2: {:?}", wire2);

    let intersection_points = intersection_points_between(&wire1, &wire2);

    println!("Intersection points: {:?}", intersection_points);

    let distances_per_point: Vec<DistancePerPoint> = intersection_points.iter().map(|point| {
        let distance = point.x.abs() + point.y.abs();
        DistancePerPoint { distance, point: *point }
    }).collect();

    println!("Distances per point: {:?}", distances_per_point);

    let closest_distance_per_point = distances_per_point.iter().min_by(|a, b| {
        a.distance.cmp(&b.distance)
    }).unwrap();

    println!("Closest distance per point: {:?}", closest_distance_per_point);

    closest_distance_per_point.distance
}

fn intersection_points_between(wire1: &Wire, wire2: &Wire) -> Vec<Point> {
    let mut intersection_points = vec![];

    // Skipping origin point intersection
    for segment1 in wire1.iter().skip(1) {
        for segment2 in wire2 {
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
