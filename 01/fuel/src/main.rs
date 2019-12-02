type Mass = i32;

fn main() {
    println!("Hello, world!");
}

fn fuel_required(mass: Mass) -> Mass {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_12() {
        assert_eq!(fuel_required(12), 2);
    }
}
