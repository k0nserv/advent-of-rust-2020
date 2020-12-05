use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use crate::parse_lines;

#[derive(Debug, Eq, PartialEq, Hash)]
struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn all() -> impl Iterator<Item = BoardingPass> {
        (0..127).flat_map(move |r| (0..8).map(move |c| BoardingPass::new(r, c)))
    }

    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn reduce_range(input: &str, range: Range<usize>, chars: (char, char)) -> Range<usize> {
    input.chars().fold(range, |range, c| {
        let half_length = range.len() / 2;
        let (high, low) = chars;
        let lowered = c.to_ascii_lowercase();

        if lowered == high {
            (range.start + half_length)..range.end
        } else if lowered == low {
            range.start..(range.start + half_length)
        } else {
            panic!("Invalid boarding pass {}", input)
        }
    })
}
impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(format!(
                "Invalid boarding pass `{}`. Should have exactly 10 characters found {}",
                s,
                s.len()
            ));
        }
        let row_valid = s[0..7].chars().filter(|&c| c == 'B' || c == 'F').count() == 8;
        let col_valid = s[7..10].chars().filter(|&c| c == 'L' || c == 'R').count() == 3;

        if !row_valid && !col_valid {
            return Err(format!(
                "Invalid boarding pass `{}`. Contains illegal characters",
                s
            ));
        }

        let row = reduce_range(&s[0..7], 0..128, ('b', 'f'));
        if row.len() != 1 {
            return Err(format!("Invalid boarding pass `{}`. After reducing first 7 characters a row number should be deduced", s));
        }

        let column = reduce_range(&s[7..10], 0..8, ('r', 'l'));
        if column.len() != 1 {
            return Err(format!("Invalid boarding pass `{}`. After reducing first 7 characters a column number should be deduced", s));
        }

        Ok(Self {
            row: row.start,
            column: column.start,
        })
    }
}

pub fn star_one(input: &str) -> usize {
    parse_lines::<BoardingPass>(input)
        .map(|b| b.id())
        .max()
        .unwrap()
}

pub fn star_two(input: &str) -> usize {
    let in_input: HashMap<_, _> = parse_lines::<BoardingPass>(input)
        .map(|b| (b.id(), b))
        .collect();

    // Skip row 0 because it's not relevant and would cause overflow
    for b in BoardingPass::all().filter(|b| b.row != 0) {
        let n1 = b.id() - 1;
        let n2 = b.id() + 1;

        if !in_input.contains_key(&b.id())
            && in_input.contains_key(&n1)
            && in_input.contains_key(&n2)
        {
            return b.id();
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::BoardingPass;

    #[test]
    fn test_parse_boarding_pass() {
        assert_eq!("BFFFBBFRRR".parse(), Ok(BoardingPass::new(70, 7)));
        assert_eq!("FFFBBBFRRR".parse(), Ok(BoardingPass::new(14, 7)));
        assert_eq!("BBFFBBFRLL".parse(), Ok(BoardingPass::new(102, 4)));
    }
}
