use std::collections::HashSet;
use std::str::FromStr;

use crate::parse_lines;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instruction {
    fn is_nop(&self) -> bool {
        match self {
            Instruction::Nop(_) => true,
            _ => false,
        }
    }

    fn is_jmp(&self) -> bool {
        match self {
            Instruction::Jmp(_) => true,
            _ => false,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().map(str::trim);

        match (parts.next(), parts.next()) {
            (Some(i), Some(value)) => {
                let parsed_value = value
                    .parse::<isize>()
                    .map_err(|e| format!("Failed to parse instruction `{}`. {}", s, e));

                match i {
                    "nop" => parsed_value.map(|v| Instruction::Nop(v)),
                    "acc" => parsed_value.map(|v| Instruction::Acc(v)),
                    "jmp" => parsed_value.map(|v| Instruction::Jmp(v)),
                    _ => Err(format!("Invalid instruction `{}`", s)),
                }
            }
            _ => Err(format!("Invalid instruction `{}`", s)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HaltStatus {
    Done,
    InfiniteLoop,
}

struct VM {
    acc: isize,
    instructions: Vec<Instruction>,
    ip: usize,
}

impl VM {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            acc: 0,
            instructions,
            ip: 0,
        }
    }

    fn run_until_done_or_infinite_loop(&mut self) -> HaltStatus {
        let mut executed_instructions = HashSet::<usize>::default();

        while self.ip < self.instructions.len() {
            let instruction = self.instructions[self.ip];

            let (new_ip, new_acc) = match instruction {
                Instruction::Jmp(offset) => ((self.ip as isize) + offset, self.acc),
                Instruction::Acc(change) => (self.ip as isize + 1, self.acc + change),
                Instruction::Nop(_) => ((self.ip as isize + 1, self.acc)),
            };

            if !executed_instructions.insert(self.ip) {
                return HaltStatus::InfiniteLoop;
            }
            self.ip = new_ip as usize;
            self.acc = new_acc;
        }

        HaltStatus::Done
    }
}

pub fn star_one(input: &str) -> isize {
    let instructions = parse_lines::<Instruction>(input);
    let mut vm = VM::new(instructions.collect());

    vm.run_until_done_or_infinite_loop();

    vm.acc
}

pub fn star_two(input: &str) -> isize {
    let instructions: Vec<_> = parse_lines::<Instruction>(input).collect();

    instructions
        .iter()
        .enumerate()
        .find_map(|(idx, &i)| {
            if i.is_jmp() || i.is_nop() {
                let mut new_instructions = instructions.clone();

                match i {
                    Instruction::Nop(v) => {
                        new_instructions[idx] = Instruction::Jmp(v);
                    }
                    Instruction::Jmp(v) => {
                        new_instructions[idx] = Instruction::Nop(v);
                    }
                    _ => {}
                };

                let mut vm = VM::new(new_instructions);

                let halt_status = vm.run_until_done_or_infinite_loop();

                if halt_status == HaltStatus::Done {
                    Some(vm.acc)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 5);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 8);
    }
}
