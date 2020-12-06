use std::collections::HashSet;

pub fn star_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|l| l.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn star_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let all_chars: HashSet<_> = group.lines().flat_map(|l| l.chars()).collect();

            group
                .lines()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .fold(all_chars, |acc, set| {
                    acc.intersection(&set).cloned().collect()
                })
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 11);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 6);
    }
}
