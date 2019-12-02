use std::env;

type Mass = u32;

fn main() {
    for arg in env::args().skip(1) {
        let parsed: Mass = arg.parse().unwrap();
        println!("Fuel required: {:?}", fuel_required(parsed));
    }
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
