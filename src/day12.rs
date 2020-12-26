use core::f64::consts::PI;
use std::str::FromStr;

use crate::math::Vector2;
use crate::parse_lines;

const NORTH_VECTOR: Vector2<isize> = Vector2::new(0, 1);
const SOUTH_VECTOR: Vector2<isize> = Vector2::new(0, -1);
const EAST_VECTOR: Vector2<isize> = Vector2::new(1, 0);
const WEST_VECTOR: Vector2<isize> = Vector2::new(-1, 0);

fn rotate(vector: Vector2<isize>, degrees: isize) -> Vector2<isize> {
    let radians = (degrees as f64) * (PI / 180.0);
    let (x, y) = (vector.x() as f64, vector.y() as f64);

    let (new_x, new_y) = (
        x * radians.cos() - y * radians.sin(),
        x * radians.sin() + y * radians.cos(),
    );

    let new = Vector2::new(new_x.round() as isize, new_y.round() as isize);

    new
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    North(isize),
    South(isize),
    West(isize),
    East(isize),

    Rotate(isize),
    Forward(isize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = s.chars().nth(0);
        let rest = &s[1..];
        let value = rest
            .parse::<isize>()
            .map_err(|e| format!("Failed to parse instruction `{}` with error {}", s, e))?;

        t.and_then(|t| match t {
            'N' => Some(Self::North(value)),
            'S' => Some(Self::South(value)),
            'E' => Some(Self::East(value)),
            'W' => Some(Self::West(value)),

            'L' => Some(Self::Rotate(value)),
            'R' => Some(Self::Rotate(-value)),
            'F' => Some(Self::Forward(value)),
            _ => None,
        })
        .ok_or_else(|| format!("Failed to parse instruction `{}`", s))
    }
}

struct World {
    position: Vector2<isize>,
    dir: Vector2<isize>,
    waypoint: Vector2<isize>,
}

impl World {
    fn new() -> Self {
        Self {
            position: Vector2::new(0, 0),
            dir: Vector2::new(1, 0),
            waypoint: Vector2::new(0, 0),
        }
    }

    fn with_waypoint(waypoint_position: Vector2<isize>) -> Self {
        Self {
            position: Vector2::new(0, 0),
            dir: Vector2::new(1, 0),
            waypoint: waypoint_position,
        }
    }

    fn execute_instructions_part_1(&mut self, instructions: &[Instruction]) {
        let (mut position, mut dir) = (self.position, self.dir);
        for instruction in instructions {
            let r = Self::execute_instruction_part_1(instruction, position, dir);
            position = r.0;
            dir = r.1;
        }

        self.position = position;
        self.dir = dir;
    }

    fn execute_instruction_part_1(
        instruction: &Instruction,
        position: Vector2<isize>,
        dir: Vector2<isize>,
    ) -> (Vector2<isize>, Vector2<isize>) {
        match instruction {
            Instruction::North(steps) => (position + NORTH_VECTOR * *steps, dir),
            Instruction::South(steps) => (position + SOUTH_VECTOR * *steps, dir),
            Instruction::East(steps) => (position + EAST_VECTOR * *steps, dir),
            Instruction::West(steps) => (position + WEST_VECTOR * *steps, dir),

            Instruction::Rotate(degrees) => (position, rotate(dir, *degrees)),
            Instruction::Forward(steps) => (position + dir * *steps, dir),
        }
    }

    fn execute_instructions_part_2(&mut self, instructions: &[Instruction]) {
        let (mut position, mut waypoint) = (self.position, self.waypoint);
        for instruction in instructions {
            let r = Self::execute_instruction_part_2(instruction, position, waypoint);
            position = r.0;
            waypoint = r.1;
            println!(
                "After executing {:?} position is {:?} and waypoint is {:?}",
                instruction, position, waypoint
            );
        }

        self.position = position;
        self.waypoint = waypoint;
    }

    fn execute_instruction_part_2(
        instruction: &Instruction,
        position: Vector2<isize>,
        waypoint: Vector2<isize>,
    ) -> (Vector2<isize>, Vector2<isize>) {
        match instruction {
            Instruction::North(steps) => (position, waypoint + NORTH_VECTOR * *steps),
            Instruction::South(steps) => (position, waypoint + SOUTH_VECTOR * *steps),
            Instruction::East(steps) => (position, waypoint + EAST_VECTOR * *steps),
            Instruction::West(steps) => (position, waypoint + WEST_VECTOR * *steps),

            Instruction::Rotate(degrees) => (position, rotate(waypoint, *degrees)),
            Instruction::Forward(steps) => (position + waypoint * *steps, waypoint),
        }
    }

    fn ship_manhattan_distance(&self) -> isize {
        self.position.manhattan_distance(Vector2::new(0, 0))
    }
}

pub fn star_one(input: &str) -> isize {
    let instructions = parse_lines::<Instruction>(input).collect::<Vec<_>>();
    let mut world = World::new();

    world.execute_instructions_part_1(&instructions);

    world.ship_manhattan_distance()
}

pub fn star_two(input: &str) -> isize {
    let instructions = parse_lines::<Instruction>(input).collect::<Vec<_>>();
    let mut world = World::with_waypoint(Vector2::new(10, 1));

    world.execute_instructions_part_2(&instructions);

    world.ship_manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 25);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 286);
    }
}
