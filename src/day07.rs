use std::collections::HashMap;

// To say there's room for improvement in this solution is an understatement,
// unfortunately I was very tired when I wrote this so it will have to exist
// in all its glorious imperfection.

type BagId = usize;

// An arena for bags, a `Bagrena`, because doing pointers/references in Rust
// is just no fun.
struct Bagrena {
    next: usize,
    bags: HashMap<BagId, String>,
}

impl Bagrena {
    fn new() -> Self {
        Self {
            next: Default::default(),
            bags: Default::default(),
        }
    }

    fn make_or_find_bag(&mut self, name: &str) -> BagId {
        // O(n) !!!!
        let existing_bag = self.bags.iter().find(|&(_, v)| v == name);

        match existing_bag {
            Some((&k, _)) => k,
            None => {
                let id = self.next;
                self.bags.insert(id, name.to_owned());
                self.next += 1;

                id
            }
        }
    }

    fn parse_bags(&mut self, input: &str) -> Result<Vec<Bag>, String> {
        // Forgive me, I was really tired when I wrote this...
        input
            .lines()
            .map(str::trim)
            .filter(|l| l.len() > 0)
            .map(|l| {
                let mut parts = l.split("contain").map(str::trim);
                match (parts.next(), parts.next()) {
                    (Some(name), Some(rest)) => {
                        let name = name.trim_end_matches('.').trim_end_matches('s');
                        let id = self.make_or_find_bag(name);

                        let contains: Result<Option<Vec<_>>, _> = match rest {
                            "no other bags." => Ok(None),
                            _ => rest
                                .split(',')
                                .map(str::trim)
                                .map(|b| {
                                    let mut parts = b.splitn(2, ' ').map(str::trim);

                                    match (parts.next().and_then(|c| c.parse().ok()), parts.next())
                                    {
                                        (Some(count), Some(name)) => {
                                            let name =
                                                name.trim_end_matches('.').trim_end_matches('s');
                                            Ok(Some((count, self.make_or_find_bag(name))))
                                        }
                                        _ => Err(format!("Invalid contents `{}`.", rest)),
                                    }
                                })
                                .collect(),
                        };

                        match contains {
                            Err(e) => Err(format!("Invalid line `{}`. Failed with error {}", l, e)),
                            Ok(contains) => Ok(Bag {
                                id,
                                name: name.to_owned(),
                                contents: contains,
                            }),
                        }
                    }
                    (_, _) => Err(format!("invalid line `{}`", l)),
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Bag {
    id: BagId,
    name: String,
    contents: Option<Vec<(usize, BagId)>>,
}

fn find(id: BagId, bag_map: &HashMap<BagId, Bag>) -> bool {
    let bag = &bag_map[&id];
    if bag.name == "shiny gold bag" {
        return true;
    }

    bag.contents
        .as_ref()
        .map(|c| c.iter().any(|&(_, content_id)| find(content_id, bag_map)))
        .unwrap_or(false)
}

fn count(id: BagId, bag_map: &HashMap<BagId, Bag>) -> usize {
    let bag = &bag_map[&id];

    bag.contents
        .as_ref()
        .map(|c| {
            c.iter().fold(0, |acc, &(b_count, b_id)| {
                acc + b_count * (1 + count(b_id, bag_map))
            })
        })
        .unwrap_or(0)
}

pub fn star_one(input: &str) -> usize {
    let mut bagrena = Bagrena::new();
    let bags = bagrena.parse_bags(input).expect("input should be parsable");
    let bag_map: HashMap<_, _> = bags.into_iter().map(|b| (b.id, b)).collect();

    bag_map
        .iter()
        .filter(|&(_, b)| b.name != "shiny gold bag")
        .filter(|&(&k, _)| find(k, &bag_map))
        .count()
}

pub fn star_two(input: &str) -> usize {
    let mut bagrena = Bagrena::new();
    let bags = bagrena.parse_bags(input).expect("input should be parsable");
    let bag_map: HashMap<_, _> = bags.into_iter().map(|b| (b.id, b)).collect();

    let shiny_bag = bag_map
        .iter()
        .find(|&(_, b)| b.name == "shiny gold bag")
        .map(|(_, b)| b);

    shiny_bag.map(|bag| count(bag.id, &bag_map)).unwrap_or(0)
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
