use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

type Mass = i32;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // for part 1, when the added fuel is not factored into the grand total
    let naive: bool;
    let filename: &String;

    if &args[1] == "--naive" {
        naive = true;
        filename = &args[2];
    } else {
        naive = false;
        filename = &args[1];
    }

    let file = File::open(filename)?;
    let file = BufReader::new(file);

    let lines_iter = file.lines().map(|l| l.unwrap());

    let mut total_fuel: Mass = 0;

    for line in lines_iter {
        let module_mass: Mass = line.parse().unwrap();
        if naive {
            total_fuel += naive_fuel_required(module_mass);
        } else {
            total_fuel += correct_fuel_required(module_mass);
        }
    }

    println!("Total fuel required: {:?}", total_fuel);

    Ok(())
}

fn naive_fuel_required(mass: Mass) -> Mass {
    (mass / 3) - 2
}

fn correct_fuel_required(mass: Mass) -> Mass {
    let mut total_fuel: Mass = 0;

    let mut fuel_required = (mass / 3) - 2;

    while fuel_required > 0 {
        total_fuel += fuel_required;
        fuel_required = (fuel_required / 3) - 2;
    }

    total_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_12_naive() {
        assert_eq!(naive_fuel_required(12), 2);
    }

    #[test]
    fn mass_14_naive() {
        assert_eq!(naive_fuel_required(14), 2);
    }

    #[test]
    fn mass_1969_naive() {
        assert_eq!(naive_fuel_required(1969), 654);
    }

    #[test]
    fn mass_100756_naive() {
        assert_eq!(naive_fuel_required(100756), 33583);
    }

    #[test]
    fn mass_12_correct() {
        assert_eq!(correct_fuel_required(12), 2);
    }

    #[test]
    fn mass_14_correct() {
        assert_eq!(correct_fuel_required(14), 2);
    }

    #[test]
    fn mass_1969_correct() {
        assert_eq!(correct_fuel_required(1969), 966);
    }

    #[test]
    fn mass_100756_correct() {
        assert_eq!(correct_fuel_required(100756), 50346);
    }
}
