use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::parse_lines;

#[derive(Debug, Clone)]
struct Policy {
    required_char: char,
    range: RangeInclusive<usize>,
}

impl Policy {
    fn is_valid_sled(&self, password: &str) -> bool {
        let count = password
            .chars()
            .filter(|&c| self.required_char == c)
            .count();

        self.range.contains(&count)
    }

    fn is_valid_toboggan(&self, password: &str) -> bool {
        let count = password
            .chars()
            .enumerate()
            .filter(|&(idx, c)| {
                match (
                    self.required_char == c,
                    *self.range.start() == idx + 1,
                    *self.range.end() == idx + 1,
                ) {
                    (true, true, false) => true,
                    (true, false, true) => true,
                    _ => false,
                }
            })
            .count();

        count == 1
    }
}

impl FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();
        let range_part = parts.next();
        let require_part = parts.next();

        if range_part.is_none() || require_part.is_none() {
            return Err(format!("Invalid policy definition `{}`", s));
        }

        let range_parts: Vec<_> = range_part
            .unwrap()
            .split('-')
            .map(str::trim)
            .map(|part| part.parse::<usize>().unwrap())
            .collect();

        if range_parts.len() != 2 {
            return Err(format!(
                "Invalid policy definition `{}`, expected exactly two parts for the range",
                s
            ));
        }
        let required_char = require_part.and_then(|p| p.trim().chars().take(1).next());

        if required_char.is_none() {
            return Err(format!(
                "Invalid policy definition `{}`, expected password",
                s
            ));
        }

        Ok(Self {
            required_char: required_char.unwrap(),
            range: RangeInclusive::new(range_parts[0], range_parts[1]),
        })
    }
}

#[derive(Debug, Clone)]
struct Entry {
    policy: Policy,
    password: String,
}

impl Entry {
    fn is_valid_sled(&self) -> bool {
        self.policy.is_valid_sled(&self.password)
    }

    fn is_valid_toboggan(&self) -> bool {
        self.policy.is_valid_toboggan(&self.password)
    }
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid Entry `{}`", s));
        }

        let policy = parts[0].parse::<Policy>()?;
        let password = parts[1].trim().to_owned();

        Ok(Self { policy, password })
    }
}

pub fn star_one(input: &str) -> usize {
    parse_lines::<Entry>(input)
        .filter(Entry::is_valid_sled)
        .count()
}

pub fn star_two(input: &str) -> usize {
    parse_lines::<Entry>(input)
        .filter(Entry::is_valid_toboggan)
        .count()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 2);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 1)
    }
}
