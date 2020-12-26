use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use itertools::Itertools;

use crate::math::Vector2;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Location {
    Floor,
    FilledSeat,
    EmptySeat,
}

impl TryFrom<char> for Location {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Self::FilledSeat),
            'L' => Ok(Self::EmptySeat),
            '.' => Ok(Self::Floor),
            _ => Err(format!("Invalid location `{}`", c)),
        }
    }
}

impl Location {
    fn as_char(&self) -> char {
        match self {
            Location::Floor => '.',
            Location::FilledSeat => '#',
            Location::EmptySeat => 'L',
        }
    }
}

const ALL_DIRECTIONS: &[Vector2<isize>] = &[
    Vector2::new(-1, 0),  // Left
    Vector2::new(-1, 1),  // Left-Up
    Vector2::new(0, 1),   // Up
    Vector2::new(1, 1),   // Right-Up
    Vector2::new(1, 0),   // Right
    Vector2::new(1, -1),  // Right-Down
    Vector2::new(0, -1),  // Down
    Vector2::new(-1, -1), // Left-Down
];

struct SeatMap {
    seats: Vec<Vec<Location>>,
}

impl fmt::Display for SeatMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.seats
                .iter()
                .map(|row| row.iter().map(Location::as_char).collect::<String>())
                .intersperse("\n".into())
                .collect::<String>()
        )
    }
}

impl SeatMap {
    fn is_out_of_bounds(&self, location: Vector2<isize>) -> bool {
        location.y() < 0
            || (location.y() as usize) >= self.seats.len()
            || location.x() < 0
            || (location.x() as usize) >= self.seats[location.y() as usize].len()
    }

    fn adjacent_seats<'a>(&'a self, to: Vector2<isize>) -> impl Iterator<Item = Location> + 'a {
        ALL_DIRECTIONS.iter().filter_map(move |&dir| {
            let location = to + dir;

            if !self.is_out_of_bounds(location) {
                Some(self.seats[location.y() as usize][location.x() as usize])
            } else {
                None
            }
        })
    }

    fn visible_seats<'a>(&'a self, to: Vector2<isize>) -> impl Iterator<Item = Location> + 'a {
        ALL_DIRECTIONS.iter().filter_map(move |&dir| {
            (1..)
                .map(|offset| {
                    let location = to + dir * offset;
                    if self.is_out_of_bounds(location) {
                        None
                    } else {
                        Some(location)
                    }
                })
                .take_while(Option::is_some)
                .map(Option::unwrap)
                .find_map(|location| {
                    match self.seats[location.y() as usize][location.x() as usize] {
                        l @ Location::FilledSeat => Some(l),
                        l @ Location::EmptySeat => Some(l),
                        _ => None,
                    }
                })
        })
    }

    fn all_seats(&self) -> impl Iterator<Item = &Location> {
        self.seats.iter().flat_map(|row| row.iter())
    }

    fn apply_adjacent_rule(&self, to: (Vector2<isize>, Location)) -> Location {
        let (coord, l) = to;

        if l == Location::Floor {
            return Location::Floor;
        }

        match l {
            Location::EmptySeat
                if self
                    .adjacent_seats(coord)
                    .all(|l| l == Location::EmptySeat || l == Location::Floor) =>
            {
                Location::FilledSeat
            }
            Location::FilledSeat
                if self
                    .adjacent_seats(coord)
                    .filter(|&l| l == Location::FilledSeat)
                    .count()
                    >= 4 =>
            {
                Location::EmptySeat
            }
            s => s,
        }
    }

    fn apply_visible_rule(&self, to: (Vector2<isize>, Location)) -> Location {
        let (coord, l) = to;

        if l == Location::Floor {
            return Location::Floor;
        }

        match l {
            Location::EmptySeat
                if self
                    .visible_seats(coord)
                    .all(|l| l == Location::EmptySeat || l == Location::Floor) =>
            {
                Location::FilledSeat
            }
            Location::FilledSeat
                if self
                    .visible_seats(coord)
                    .filter(|&l| l == Location::FilledSeat)
                    .count()
                    >= 5 =>
            {
                Location::EmptySeat
            }
            s => s,
        }
    }

    fn tick(self, use_adjacent_rule: bool) -> Self {
        Self {
            seats: self
                .seats
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, &l)| {
                            if use_adjacent_rule {
                                self.apply_adjacent_rule((Vector2::new(x as isize, y as isize), l))
                            } else {
                                self.apply_visible_rule((Vector2::new(x as isize, y as isize), l))
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn tick_until_stable(self, use_adjacent_rule: bool) -> Self {
        let mut observed_states: HashSet<_> = HashSet::new();
        let mut map = self;
        observed_states.insert(map.seats.clone());

        loop {
            map = map.tick(use_adjacent_rule);

            if !observed_states.insert(map.seats.clone()) {
                break map;
            }
        }
    }
}

impl FromStr for SeatMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seats_or_error: Result<_, Self::Err> = s
            .lines()
            .filter_map(|l| {
                let trimmed = l.trim();

                if trimmed.len() > 0 {
                    Some(trimmed)
                } else {
                    None
                }
            })
            .map(|l| l.chars().map(Location::try_from).collect())
            .collect();

        seats_or_error.map(|seats| Self { seats })
    }
}

pub fn star_one(input: &str) -> usize {
    let seat_map = input.parse::<SeatMap>().expect("Invalid seat map");
    let stable_map = seat_map.tick_until_stable(true);

    stable_map
        .all_seats()
        .filter(|&l| l == &Location::FilledSeat)
        .count()
}

pub fn star_two(input: &str) -> usize {
    let seat_map = input.parse::<SeatMap>().expect("Invalid seat map");
    let stable_map = seat_map.tick_until_stable(false);

    stable_map
        .all_seats()
        .filter(|&l| l == &Location::FilledSeat)
        .count()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 37);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 26);
    }
}
