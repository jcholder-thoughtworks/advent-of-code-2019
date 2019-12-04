fn digit_at_place(guess: u32, place: u32) -> u32 {
    let power = u32::pow(10, place);
    ((guess - (guess % power)) / power) % 10
}

pub fn total_possible_passwords(range_start: u32, range_end: u32) -> u32 {
    let mut total = 0;

    for guess in range_start..=range_end {
        let mut pair_lengths = vec![];
        let mut low_digit = false;
        let mut pair_count = 0;

        for place in 0..5 {
            let right = digit_at_place(guess, place);
            let left = digit_at_place(guess, place + 1);

            if right < left {
                low_digit = true;
                break;
            }

            if right == left {
                pair_count += 1;
            } else {
                if pair_count > 0 {
                    pair_lengths.push(pair_count + 1);
                }
                pair_count = 0;
            }
        }

        if pair_count > 0 {
            pair_lengths.push(pair_count + 1);
        }

        if low_digit {
            continue;
        }

        if pair_lengths.len() == 0 {
            continue;
        }

        let any_single_pair = pair_lengths.iter().any(|pl| *pl == 2);

        if !any_single_pair {
            continue;
        }

        total += 1;
    }

    total
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const GUESS: u32 = 163499;

    #[test]
    fn digit_at_place_0() {
        assert_eq!(digit_at_place(GUESS, 0), 9);
    }

    #[test]
    fn digit_at_place_1() {
        assert_eq!(digit_at_place(GUESS, 1), 9);
    }

    #[test]
    fn digit_at_place_2() {
        assert_eq!(digit_at_place(GUESS, 2), 4);
    }

    #[test]
    fn digit_at_place_3() {
        assert_eq!(digit_at_place(GUESS, 3), 3);
    }

    #[test]
    fn digit_at_place_4() {
        assert_eq!(digit_at_place(GUESS, 4), 6);
    }

    #[test]
    fn digit_at_place_5() {
        assert_eq!(digit_at_place(GUESS, 5), 1);
    }

    #[test]
    fn total_possible_passwords_0() {
        assert_eq!(total_possible_passwords(112233, 112233), 1);
    }

    #[test]
    fn total_possible_passwords_1() {
        assert_eq!(total_possible_passwords(123444, 123444), 0);
    }

    #[test]
    fn total_possible_passwords_2() {
        assert_eq!(total_possible_passwords(111122, 111122), 1);
    }
}
