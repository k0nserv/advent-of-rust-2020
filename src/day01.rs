use crate::parse_lines;

pub fn star_one(input: &str) -> usize {
    let sorted_numbers = {
        let mut numbers: Vec<_> = parse_lines::<usize>(input).collect();
        numbers.sort();
        numbers
    };

    for x in &sorted_numbers {
        for y in &sorted_numbers {
            match x + y {
                2020 => return x * y,
                n @ _ if n > 2020 => break,
                _ => (),
            }
        }
    }

    0
}

pub fn star_two(input: &str) -> usize {
    let sorted_numbers = {
        let mut numbers: Vec<_> = parse_lines::<usize>(input).collect();
        numbers.sort();
        numbers
    };

    for x in &sorted_numbers {
        for y in &sorted_numbers {
            if x + y > 2020 {
                break;
            }

            for z in &sorted_numbers {
                match x + y + z {
                    2020 => return x * y * z,
                    n @ _ if n > 2020 => break,
                    _ => (),
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const TEST_INPUT: &'static str = "1721
979
366
299
675
1456";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(TEST_INPUT), 514579);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(TEST_INPUT), 241861950);
    }
}
