use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

use crate::math::{Vector3, Vector4};

// TODO: Should really use lazy static for this
const NEIGHBOURS_3: &'static [Vector3<isize>] = &[
    Vector3::new(0, 0, -1),
    Vector3::new(0, 0, 1),
    Vector3::new(0, -1, 0),
    Vector3::new(0, -1, -1),
    Vector3::new(0, -1, 1),
    Vector3::new(0, 1, 0),
    Vector3::new(0, 1, -1),
    Vector3::new(0, 1, 1),
    Vector3::new(-1, 0, 0),
    Vector3::new(-1, 0, -1),
    Vector3::new(-1, 0, 1),
    Vector3::new(-1, -1, 0),
    Vector3::new(-1, -1, -1),
    Vector3::new(-1, -1, 1),
    Vector3::new(-1, 1, 0),
    Vector3::new(-1, 1, -1),
    Vector3::new(-1, 1, 1),
    Vector3::new(1, 0, 0),
    Vector3::new(1, 0, -1),
    Vector3::new(1, 0, 1),
    Vector3::new(1, -1, 0),
    Vector3::new(1, -1, -1),
    Vector3::new(1, -1, 1),
    Vector3::new(1, 1, 0),
    Vector3::new(1, 1, -1),
    Vector3::new(1, 1, 1),
];

// TODO: Should really use lazy static for this
const NEIGHBOURS_4: &'static [Vector4<isize>] = &[
    Vector4::new(0, 0, 0, 1),
    Vector4::new(0, 0, 0, -1),
    Vector4::new(0, 0, 1, 0),
    Vector4::new(0, 0, 1, 1),
    Vector4::new(0, 0, 1, -1),
    Vector4::new(0, 0, -1, 0),
    Vector4::new(0, 0, -1, 1),
    Vector4::new(0, 0, -1, -1),
    Vector4::new(0, 1, 0, 0),
    Vector4::new(0, 1, 0, 1),
    Vector4::new(0, 1, 0, -1),
    Vector4::new(0, 1, 1, 0),
    Vector4::new(0, 1, 1, 1),
    Vector4::new(0, 1, 1, -1),
    Vector4::new(0, 1, -1, 0),
    Vector4::new(0, 1, -1, 1),
    Vector4::new(0, 1, -1, -1),
    Vector4::new(0, -1, 0, 0),
    Vector4::new(0, -1, 0, 1),
    Vector4::new(0, -1, 0, -1),
    Vector4::new(0, -1, 1, 0),
    Vector4::new(0, -1, 1, 1),
    Vector4::new(0, -1, 1, -1),
    Vector4::new(0, -1, -1, 0),
    Vector4::new(0, -1, -1, 1),
    Vector4::new(0, -1, -1, -1),
    Vector4::new(1, 0, 0, 0),
    Vector4::new(1, 0, 0, 1),
    Vector4::new(1, 0, 0, -1),
    Vector4::new(1, 0, 1, 0),
    Vector4::new(1, 0, 1, 1),
    Vector4::new(1, 0, 1, -1),
    Vector4::new(1, 0, -1, 0),
    Vector4::new(1, 0, -1, 1),
    Vector4::new(1, 0, -1, -1),
    Vector4::new(1, 1, 0, 0),
    Vector4::new(1, 1, 0, 1),
    Vector4::new(1, 1, 0, -1),
    Vector4::new(1, 1, 1, 0),
    Vector4::new(1, 1, 1, 1),
    Vector4::new(1, 1, 1, -1),
    Vector4::new(1, 1, -1, 0),
    Vector4::new(1, 1, -1, 1),
    Vector4::new(1, 1, -1, -1),
    Vector4::new(1, -1, 0, 0),
    Vector4::new(1, -1, 0, 1),
    Vector4::new(1, -1, 0, -1),
    Vector4::new(1, -1, 1, 0),
    Vector4::new(1, -1, 1, 1),
    Vector4::new(1, -1, 1, -1),
    Vector4::new(1, -1, -1, 0),
    Vector4::new(1, -1, -1, 1),
    Vector4::new(1, -1, -1, -1),
    Vector4::new(-1, 0, 0, 0),
    Vector4::new(-1, 0, 0, 1),
    Vector4::new(-1, 0, 0, -1),
    Vector4::new(-1, 0, 1, 0),
    Vector4::new(-1, 0, 1, 1),
    Vector4::new(-1, 0, 1, -1),
    Vector4::new(-1, 0, -1, 0),
    Vector4::new(-1, 0, -1, 1),
    Vector4::new(-1, 0, -1, -1),
    Vector4::new(-1, 1, 0, 0),
    Vector4::new(-1, 1, 0, 1),
    Vector4::new(-1, 1, 0, -1),
    Vector4::new(-1, 1, 1, 0),
    Vector4::new(-1, 1, 1, 1),
    Vector4::new(-1, 1, 1, -1),
    Vector4::new(-1, 1, -1, 0),
    Vector4::new(-1, 1, -1, 1),
    Vector4::new(-1, 1, -1, -1),
    Vector4::new(-1, -1, 0, 0),
    Vector4::new(-1, -1, 0, 1),
    Vector4::new(-1, -1, 0, -1),
    Vector4::new(-1, -1, 1, 0),
    Vector4::new(-1, -1, 1, 1),
    Vector4::new(-1, -1, 1, -1),
    Vector4::new(-1, -1, -1, 0),
    Vector4::new(-1, -1, -1, 1),
    Vector4::new(-1, -1, -1, -1),
];

trait NDimVector: Hash + Eq + PartialEq + Copy + Clone {
    fn new_ndim(x: isize, y: isize) -> Self;
    fn neighbours(self) -> Box<dyn Iterator<Item = Self>>;
}

impl NDimVector for Vector3<isize> {
    fn new_ndim(x: isize, y: isize) -> Self {
        Self::new(x, y, 0)
    }

    fn neighbours(self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(NEIGHBOURS_3.iter().map(move |&dir| self + dir))
    }
}

impl NDimVector for Vector4<isize> {
    fn new_ndim(x: isize, y: isize) -> Self {
        Self::new(x, y, 0, 0)
    }

    fn neighbours(self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(NEIGHBOURS_4.iter().map(move |&dir| self + dir))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum State {
    Active,
    Inactive,
}
struct World<Vector> {
    locations: HashMap<Vector, State>,
}

impl<Vector: NDimVector> World<Vector> {
    fn active_cubes(&self) -> usize {
        self.locations
            .values()
            .filter(|&s| s == &State::Active)
            .count()
    }

    fn neighbours<'a>(
        locations: &'a HashMap<Vector, State>,
        location: Vector,
    ) -> Box<dyn Iterator<Item = State> + 'a> {
        Box::new(
            location
                .neighbours()
                .filter_map(move |neighbour_location| locations.get(&neighbour_location).cloned()),
        )
    }

    fn tick(self) -> Self {
        let locations = self.locations;

        let new_locations: HashSet<Vector> = locations
            .iter()
            .flat_map(|(&l, _)| l.neighbours())
            .collect();

        let new_state = new_locations
            .into_iter()
            .map(|l| {
                let neighbours = World::neighbours(&locations, l);

                match locations.get(&l) {
                    Some(State::Active) => {
                        let active_neighbours = neighbours.filter(|s| *s == State::Active).count();

                        if active_neighbours == 2 || active_neighbours == 3 {
                            (l, State::Active)
                        } else {
                            (l, State::Inactive)
                        }
                    }
                    _ => {
                        let active_neighbours = neighbours.filter(|s| *s == State::Active).count();

                        if active_neighbours == 3 {
                            (l, State::Active)
                        } else {
                            (l, State::Inactive)
                        }
                    }
                }
            })
            .collect();

        Self {
            locations: new_state,
        }
    }
}

impl<Vector: NDimVector> FromStr for World<Vector> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locations = s
            .lines()
            .filter_map(|l| {
                let trimmed = l.trim();

                if trimmed.len() > 0 {
                    Some(trimmed)
                } else {
                    None
                }
            })
            .enumerate()
            .flat_map(move |(y, l)| {
                l.chars().enumerate().map(move |(x, c)| match c {
                    '#' => Ok((Vector::new_ndim(x as isize, y as isize), State::Active)),
                    '.' => Ok((Vector::new_ndim(x as isize, y as isize), State::Inactive)),
                    _ => Err(format!("Invalid location `{}`", c)),
                })
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self { locations })
    }
}

pub fn star_one(input: &str) -> usize {
    let mut world = input
        .parse::<World<Vector3<isize>>>()
        .expect("Failed to parse input");

    for _ in 0..6 {
        world = world.tick()
    }

    world.active_cubes()
}

pub fn star_two(input: &str) -> usize {
    let mut world = input
        .parse::<World<Vector4<isize>>>()
        .expect("Failed to parse input");

    for _ in 0..6 {
        world = world.tick()
    }

    world.active_cubes()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = ".#.
..#
###";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 112);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 848);
    }
}
