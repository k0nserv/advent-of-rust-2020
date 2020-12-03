use std::convert::TryFrom;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Location {
    Empty,
    Tree,
}

impl TryFrom<char> for Location {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Self::Tree),
            '.' => Ok(Self::Empty),
            _ => Err(format!("Invalid location `{}`", c)),
        }
    }
}

#[derive(Debug, Clone)]
struct World {
    locations: Vec<Vec<Location>>,
}

impl World {
    fn is_past_end(&self, y: usize) -> bool {
        y >= self.locations.len()
    }
}

impl FromStr for World {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locations_result: Result<Vec<Vec<_>>, _> = s
            .lines()
            .map(str::trim)
            .filter(|s| s.len() > 0)
            .map(|l| l.chars().map(|c| Location::try_from(c)).collect())
            .collect();

        match locations_result {
            Ok(locations) => Ok(Self { locations }),
            Err(e) => Err(format!("Failed to parse world with error: {}", e)),
        }
    }
}

impl Index<(usize, usize)> for World {
    type Output = Location;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (mut x, y) = index;
        x %= self.locations[0].len();

        &self.locations[y][x]
    }
}

fn check_slope(world: &World, xdelta: usize, ydelta: usize) -> usize {
    (0..)
        .take_while(|&y| !world.is_past_end(y * ydelta))
        .map(|y| world[(y * xdelta, y * ydelta)])
        .filter(|&l| l == Location::Tree)
        .count()
}

pub fn star_one(input: &str) -> usize {
    let world = input.parse::<World>().expect("World should be parsable");

    check_slope(&world, 3, 1)
}

pub fn star_two(input: &str) -> usize {
    let world = input.parse::<World>().expect("World should be parsable");

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(xdelta, ydelta)| check_slope(&world, xdelta, ydelta))
        .product()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 7);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 336);
    }
}
