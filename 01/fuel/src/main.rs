use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

type Mass = u32;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let file = BufReader::new(file);

    let lines_iter = file.lines().map(|l| l.unwrap());

    for line in lines_iter {
        if line.len() == 0 { // skip blank lines, e.g. trailing returns
            continue;
        }

        let parsed: Mass = line.parse().unwrap();
        println!("{:?}", fuel_required(parsed));
    }

    Ok(())
}

fn fuel_required(mass: Mass) -> Mass {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_12() {
        assert_eq!(fuel_required(12), 2);
    }

    #[test]
    fn mass_14() {
        assert_eq!(fuel_required(14), 2);
    }

    #[test]
    fn mass_1969() {
        assert_eq!(fuel_required(1969), 654);
    }

    #[test]
    fn mass_100756() {
        assert_eq!(fuel_required(100756), 33583);
    }
}
