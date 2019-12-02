type Mass = i32;

fn main() {
    println!("Hello, world!");
}

fn fuel_required(_mass: Mass) -> Mass {
    0
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
