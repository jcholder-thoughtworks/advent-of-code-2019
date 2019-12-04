use password::*;

const RANGE_START: u32 = 134564;
const RANGE_END: u32 = 585159;

fn main() {
    println!("Total possible passwords: {}", total_possible_passwords(RANGE_START, RANGE_END));
}
