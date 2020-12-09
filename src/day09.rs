use std::collections::VecDeque;

use itertools::Itertools;

use crate::parse_lines;

pub fn star_one(input: &str, window_size: usize) -> usize {
    let numbers: Vec<usize> = parse_lines(input).collect();
    let mut window: VecDeque<_> = numbers.iter().cloned().take(window_size).collect();

    numbers
        .iter()
        .enumerate()
        .skip(window_size)
        .find(|&(i, n)| {
            if window.iter().combinations(2).any(|v| v[0] + v[1] == *n) {
                window.pop_front();
                window.push_back(numbers[i]);

                false
            } else {
                true
            }
        })
        .map(|(_, n)| *n)
        .unwrap_or(0)
}

pub fn star_two(input: &str, target: usize) -> usize {
    let numbers: Vec<usize> = parse_lines(input).collect();

    let result = (2..).find_map(|window_size| {
        numbers
            .windows(window_size)
            .find(|win| win.iter().sum::<usize>() == target)
    });

    result
        .map(|v| v.iter().min().unwrap() + v.iter().max().unwrap())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT, 5), 127);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT, 127), 62);
    }
}
