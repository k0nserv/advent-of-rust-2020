use std::collections::{HashMap, VecDeque};
use std::iter;
use std::str::FromStr;

use itertools::Itertools;

use crate::{parse_lines, Either};

#[derive(Debug)]
struct Mask {
    raw: String,
    mask: usize,
    base_value: usize,
}

impl Mask {
    fn new(mask: usize, base_value: usize, raw: String) -> Self {
        Self {
            mask,
            base_value,
            raw,
        }
    }

    fn apply_mask(&self, value: usize) -> usize {
        self.base_value + (value & self.mask)
    }

    fn apply_address_mask(&self, address: usize) -> impl Iterator<Item = usize> {
        let options = self
            .raw
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(b, c)| match c {
                '0' => {
                    if (address & (1 << b)) >> b == 1 {
                        Some(['1'].iter())
                    } else {
                        Some(['0'].iter())
                    }
                }
                '1' => Some(['1'].iter()),
                'X' => Some(['0', '1'].iter()),
                _ => panic!("Invalid mask value {}", c),
            })
            .multi_cartesian_product();

        options.map(|address| {
            address
                .into_iter()
                .enumerate()
                .fold(0, |acc, (b, c)| match c {
                    '1' => acc + 2usize.pow(b as u32),
                    '0' => acc,
                    _ => panic!("Invalid bit {}", c),
                })
        })
    }
}

impl FromStr for Mask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('=');

        match (parts.next().map(str::trim), parts.next().map(str::trim)) {
            (Some("mask"), Some(rest)) => {
                let mask = usize::from_str_radix(&rest.replace('1', "0").replace('X', "1"), 2)
                    .map_err(|e| format!("Invalid mask `{}`. Failed to parse mask: {}", s, e))?;
                let value = usize::from_str_radix(&rest.replace('X', "0"), 2)
                    .map_err(|e| format!("Invalid mask `{}`. Failed to parse mask: {}", s, e))?;

                return Ok(Self::new(mask, value, rest.into()));
            }
            _ => Err(format!("Invalid mask `{}`", s)),
        }
    }
}

#[derive(Debug)]
struct Assignment {
    address: usize,
    value: usize,
}

impl Assignment {
    fn new(address: usize, value: usize) -> Self {
        Self { address, value }
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('=');

        match (parts.next().map(str::trim), parts.next().map(str::trim)) {
            (Some(marker), Some(rest)) => {
                let index = match marker.strip_prefix("mem") {
                    Some(indexing) => indexing
                        .replace('[', "")
                        .replace(']', "")
                        .parse::<usize>()
                        .ok(),
                    _ => None,
                };

                index
                    .and_then(|i| match rest.parse::<usize>() {
                        Ok(v) => Some((i, v)),
                        _ => None,
                    })
                    .map(|(i, v)| Self::new(i, v))
                    .ok_or_else(|| format!("Invalid assignment `{}`", s))
            }
            _ => Err(format!("Invalid assignment `{}`", s)),
        }
    }
}

pub fn star_one(input: &str) -> usize {
    let mut iter = parse_lines::<Either<Assignment, Mask>>(input);
    let mut active_mask = iter.find_map(Either::right).unwrap();
    let mut instructions: VecDeque<_> = iter.collect();
    let mut memory = HashMap::<usize, usize>::default();

    while !instructions.is_empty() {
        let next_instruction = instructions.pop_front().unwrap();

        match next_instruction {
            Either::Left(assignment) => {
                let masked_value = active_mask.apply_mask(assignment.value);
                memory
                    .entry(assignment.address)
                    .and_modify(|e| *e = masked_value)
                    .or_insert(masked_value);
            }
            Either::Right(mask) => {
                active_mask = mask;
            }
        }
    }

    memory.values().sum()
}

pub fn star_two(input: &str) -> usize {
    let mut iter = parse_lines::<Either<Assignment, Mask>>(input);
    let mut active_mask = iter.find_map(Either::right).unwrap();
    let mut instructions: VecDeque<_> = iter.collect();
    let mut memory = HashMap::<usize, usize>::default();

    while !instructions.is_empty() {
        let next_instruction = instructions.pop_front().unwrap();

        match next_instruction {
            Either::Left(assignment) => {
                for masked_address in active_mask.apply_address_mask(assignment.address) {
                    memory
                        .entry(masked_address)
                        .and_modify(|e| *e = assignment.value)
                        .or_insert(assignment.value);
                }
            }
            Either::Right(mask) => {
                active_mask = mask;
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    const INPUT_TWO: &'static str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 165);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT_TWO), 208);
    }
}
