use std::collections::HashMap;

use crate::parse_custom_separated;

struct Entry {
    first_spoken: usize,
    last_spoken: [usize; 2],
}

impl Entry {
    fn new(first_spoken: usize) -> Self {
        Self {
            first_spoken,
            last_spoken: [first_spoken, first_spoken],
        }
    }

    fn is_first(&self) -> bool {
        self.first_spoken == self.last_spoken[1]
    }

    fn age(&self, turn: usize) -> usize {
        self.last_spoken[1] - self.last_spoken[0]
    }

    fn spoken(&mut self, turn: usize) {
        self.last_spoken[0] = self.last_spoken[1];
        self.last_spoken[1] = turn;
    }
}

pub fn star_one(input: &str, nth_number: usize) -> usize {
    let numbers: Vec<usize> = parse_custom_separated(input, ",").collect();
    let mut last_spoken_at: HashMap<usize, Entry> = HashMap::default();
    let mut most_recently_spoken = 0;

    for (turn, n) in numbers.iter().enumerate().map(|(i, n)| (i + 1, n)) {
        most_recently_spoken = *n;
        last_spoken_at
            .entry(*n)
            .and_modify(|e| e.spoken(turn))
            .or_insert(Entry::new(turn));
    }

    for turn in numbers.len() + 1..=nth_number {
        let previous_entry = last_spoken_at.get(&most_recently_spoken).unwrap();

        let spoken = if previous_entry.is_first() {
            last_spoken_at
                .entry(0)
                .and_modify(|e| e.spoken(turn))
                .or_insert_with(|| Entry::new(turn));

            0
        } else {
            let number = previous_entry.age(turn);
            last_spoken_at
                .entry(number)
                .and_modify(|e| e.spoken(turn))
                .or_insert_with(|| Entry::new(turn));

            number
        };

        most_recently_spoken = spoken;
    }

    most_recently_spoken
}

pub fn star_two(input: &str, nth_number: usize) -> usize {
    // There's probably a cycle or something that can be absued to make this faster
    // but Rust is fasten enough that just doing all the calculations is fine
    // TODO: Maybe find the fast solution
    star_one(input, nth_number)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("0,3,6", 2020), 436);
        assert_eq!(star_one("1,3,2", 2020), 1);
        assert_eq!(star_one("1,3,2", 2020), 1);
        assert_eq!(star_one("1,3,2", 2020), 1);
        assert_eq!(star_one("2,1,3", 2020), 10);
        assert_eq!(star_one("1,2,3", 2020), 27);
        assert_eq!(star_one("2,3,1", 2020), 78);
        assert_eq!(star_one("3,2,1", 2020), 438);
        assert_eq!(star_one("3,1,2", 2020), 1836);
    }
}
