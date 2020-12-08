use std::collections::HashMap;
use std::str::FromStr;

use crate::parse_lines;

fn normalize_bag_name(name: &str) -> &str {
    name.trim()
        .trim_end_matches('.')
        .trim_end_matches("bags")
        .trim_end_matches("bag")
        .trim()
}

fn parse_bag_list(list: &str) -> Result<Option<Vec<(usize, String)>>, String> {
    list.split(",")
        .map(str::trim)
        .map(|s| {
            if s == "no other bags." {
                return Ok(None);
            }

            let mut parts = s.splitn(2, char::is_whitespace);

            match (parts.next(), parts.next()) {
                (Some(count), Some(name)) => count
                    .parse::<usize>()
                    .map(|c| Some((c, normalize_bag_name(name).to_owned())))
                    .map_err(|e| format!("Failed to parse bag list `{}`. Error: {}", list, e)),
                _ => Err(format!("Failed to parse bag list `{}`", list)),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Bag {
    name: String,
    contents: Vec<(usize, String)>,
}

impl Bag {
    fn is_shiny(&self) -> bool {
        self.name == "shiny gold"
    }
}

impl FromStr for Bag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("contain");
        let (name, rest) = match (parts.next(), parts.next()) {
            (Some(name), Some(rest)) => Ok((normalize_bag_name(name), rest)),
            _ => Err(format!("Invalid bag definition `{}`", s)),
        }?;

        let contents = parse_bag_list(rest)?.unwrap_or(vec![]);

        Ok(Self {
            name: name.to_owned(),
            contents,
        })
    }
}

fn find(name: &str, bag_map: &HashMap<String, Bag>) -> bool {
    let bag = &bag_map[name];
    if bag.is_shiny() {
        return true;
    }

    bag.contents
        .iter()
        .any(|(_, content_name)| find(&content_name, bag_map))
}

fn count(name: &str, bag_map: &HashMap<String, Bag>) -> usize {
    let bag = &bag_map[name];

    bag.contents.iter().fold(0, |acc, (b_count, b_name)| {
        acc + b_count * (1 + count(&b_name, bag_map))
    })
}

pub fn star_one(input: &str) -> usize {
    let bags = parse_lines::<Bag>(input);
    let bag_map: HashMap<_, _> = bags.map(|b| (b.name.clone(), b)).collect();

    bag_map
        .iter()
        .filter(|&(_, b)| !b.is_shiny())
        .filter(|&(k, _)| find(k, &bag_map))
        .count()
}

pub fn star_two(input: &str) -> usize {
    let bags = parse_lines::<Bag>(input);
    let bag_map: HashMap<_, _> = bags.map(|b| (b.name.clone(), b)).collect();

    let shiny_bag = bag_map.iter().find(|&(_, b)| b.is_shiny()).map(|(_, b)| b);

    shiny_bag.map(|bag| count(&bag.name, &bag_map)).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    const INPUT_TWO: &'static str = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 4);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 32);
        assert_eq!(star_two(INPUT_TWO), 126);
    }
}
