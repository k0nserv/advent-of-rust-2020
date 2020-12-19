use std::collections::VecDeque;

use itertools::Itertools;

fn parse(input: &str, filter: bool) -> (usize, Vec<Option<usize>>) {
    let lines: Vec<_> = input.lines().map(str::trim).collect();
    let desired_departure = lines[0].parse::<usize>().expect(&format!(
        "Expected to be able to parse line `{}` as a number",
        lines[0]
    ));
    let buses: Vec<_> = lines[1]
        .split(',')
        .map(str::trim)
        .filter_map(|l| {
            if filter && l == "x" {
                None
            } else if !filter && l == "x" {
                Some(None)
            } else {
                Some(Some(l))
            }
        })
        .map(|o| o.map(|n| n.parse::<_>().expect("Panik")))
        .collect::<Vec<Option<usize>>>();

    (desired_departure, buses)
}

fn inverse(y: usize, modulou: usize) -> usize {
    (1..)
        .filter_map(|x| {
            if ((x * y) % modulou) == 1 {
                Some(x)
            } else {
                None
            }
        })
        .nth(0)
        .unwrap()
}

pub fn star_one(input: &str) -> usize {
    let (desired_departure, buses) = parse(input, true);

    buses
        .into_iter()
        .map(Option::unwrap)
        .map(|bus_departure| {
            (
                bus_departure,
                bus_departure - (desired_departure % bus_departure),
            )
        })
        .min_by_key(|&(_, t)| t)
        .map(|(b, t)| b * t)
        .unwrap_or(0)
}

pub fn star_two(input: &str) -> usize {
    let (_, buses) = parse(input, false);

    let a = buses
        .iter()
        .enumerate()
        .filter_map(|(offset, id)| match id {
            Some(id) => Some(id - (offset % id)),
            None => None,
        });
    let n: usize = buses.iter().filter_map(|&s| s).product();
    let y = buses.iter().filter_map(|id| match id {
        Some(id) => Some(n / id),
        None => None,
    });
    let z = y
        .clone()
        .zip(buses.iter().filter_map(|&s| s).clone())
        .map(|(y, n)| inverse(y, n));

    izip!(a.clone(), y, z)
        .map(|(a, y, z)| a * y * z)
        .sum::<usize>()
        % n
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "939
7,13,x,x,59,x,31,19";
    const INPUT_ALT_1: &'static str = "1232131
67,x,7,59,61";
    const INPUT_ALT_2: &'static str = "1232131
1789,37,47,1889";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 295);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 1068781);
        assert_eq!(star_two(INPUT_ALT_1), 779210);
        assert_eq!(star_two(INPUT_ALT_2), 1202161486);
    }
}
