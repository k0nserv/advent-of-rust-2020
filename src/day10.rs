use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

use crate::parse_lines;

pub fn star_one(input: &str) -> usize {
    let mut adapters: HashSet<_> = {
        let mut v: HashSet<_> = parse_lines::<usize>(input).collect();
        v.insert(v.iter().max().unwrap() + 3);

        v
    };

    let mut jolts = 0;
    let mut differences: HashMap<usize, usize> = HashMap::default();

    while adapters.len() != 0 {
        let next = (jolts..=jolts + 3)
            .into_iter()
            .find(|j| adapters.contains(j))
            .unwrap();

        differences
            .entry(next - jolts)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        adapters.remove(&next);

        jolts = next;
    }

    differences[&1] * differences[&3]
}

pub fn star_two(input: &str) -> usize {
    let adapters: HashSet<_> = {
        let mut v: HashSet<_> = parse_lines::<usize>(input).collect();
        v.insert(v.iter().max().unwrap() + 3);

        v
    };
    let incoming: HashMap<usize, HashSet<_>> = adapters
        .iter()
        .cloned()
        .filter_map(|n| {
            let incoming: HashSet<usize> = (1..=3)
                .filter_map(|d| {
                    if n > d && adapters.contains(&(n - d)) {
                        Some(n - d)
                    } else if n <= d && n <= 3 {
                        Some(0)
                    } else {
                        None
                    }
                })
                .collect();

            if incoming.is_empty() {
                None
            } else {
                Some((n, incoming))
            }
        })
        .collect();

    let mut incoming_multiplier: HashMap<usize, usize> = iter::once((0, 1)).collect();
    let mut queue: VecDeque<usize> = adapters.iter().min().into_iter().cloned().collect();

    while queue.len() > 0 {
        let jolts = queue.pop_front().unwrap();
        if incoming_multiplier.contains_key(&jolts) {
            continue;
        }

        let multiplier = incoming[&jolts]
            .iter()
            .flat_map(|inc| incoming_multiplier.get(inc))
            .sum();

        incoming_multiplier.entry(jolts).or_insert(multiplier);

        (1..=3)
            .filter(|n| adapters.contains(&(jolts + n)))
            .for_each(|n| queue.push_back(jolts + n));
    }

    incoming_multiplier[adapters.iter().max().unwrap()]
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT_SMALL: &'static str = "16
10
15
5
1
11
7
19
6
12
4";

    const INPUT_SMALL_SEQUENCE: &'static str = "1
2
3
4";

    const INPUT_LARGE: &'static str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT_SMALL), 35);
        assert_eq!(star_one(INPUT_LARGE), 220);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT_SMALL), 8);
        assert_eq!(star_two(INPUT_SMALL_SEQUENCE), 7);
        assert_eq!(star_two(INPUT_LARGE), 19208);
    }
}
