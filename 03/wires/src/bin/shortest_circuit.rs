use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use wires::*;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(file);

    let lines: Vec<String> = file.lines().map(|l| l.unwrap()).collect();

    let wire1 = Wire::from(lines[0].as_str());
    let wire2 = Wire::from(lines[1].as_str());

    //println!("Distance: {}", wire1.closest_intersection_distance_with(&wire2));

    Ok(())
}
