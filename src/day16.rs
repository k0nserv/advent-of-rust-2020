use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::Either;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Validation {
    category: String,
    valid_ranges: Vec<RangeInclusive<usize>>,
}

fn merge<T>(opt: (Option<T>, Option<T>)) -> Option<(T, T)> {
    match opt {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

fn parse_range(s: &str) -> Option<RangeInclusive<usize>> {
    let mut parts = s.split('-');

    merge((parts.next().map(str::trim), parts.next().map(str::trim))).and_then(
        |(lower_bound, upper_bound)| {
            lower_bound.parse::<usize>().ok().and_then(|lower| {
                let upper = upper_bound.parse::<usize>().ok();

                upper.map(|upper| (lower..=upper))
            })
        },
    )
}

impl Validation {
    fn is_valid(&self, value: usize) -> bool {
        self.valid_ranges.iter().any(|range| range.contains(&value))
    }
}

impl Hash for Validation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.category.hash(state);
    }
}

impl FromStr for Validation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');

        merge((parts.next().map(str::trim), parts.next().map(str::trim)))
            .and_then(|(category, rest)| {
                let mut range_parts = rest.split("or");

                merge((
                    range_parts.next().map(str::trim),
                    range_parts.next().map(str::trim),
                ))
                .and_then(|(r1, r2)| {
                    merge((parse_range(r1), parse_range(r2))).map(|(parsed_r1, parsed_r2)| Self {
                        category: category.to_owned(),
                        valid_ranges: vec![parsed_r1, parsed_r2],
                    })
                })
            })
            .ok_or_else(|| format!("Could not parse `{}` as validation", s))
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    digits: Vec<usize>,
}

impl Ticket {
    fn is_valid(&self, validations: &[Validation]) -> bool {
        if self.digits.len() != validations.len() {
            return false;
        }

        self.digits
            .iter()
            .zip(validations)
            .all(|(&d, validation)| validation.is_valid(d))
    }
}

impl FromStr for Ticket {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .split(',')
            .map(str::parse::<usize>)
            .collect::<Result<_, _>>()
            .map_err(|e| format!("Failed to parse ticker `{}`. {}", s, e))?;

        Ok(Self { digits })
    }
}

enum ParserState {
    Validations,
    YourTicket,
    NearbyTickets,
}

fn parse(input: &str) -> Vec<Either<Validation, Either<Ticket, Ticket>>> {
    let (_, items) = input
        .lines()
        .filter_map(|l| {
            let trimmed = l.trim();

            if trimmed.len() > 0 {
                Some(trimmed)
            } else {
                None
            }
        })
        .fold(
            (ParserState::Validations, vec![]),
            |(state, mut items), line| {
                if line == "your ticket:" {
                    (ParserState::YourTicket, items)
                } else if line == "nearby tickets:" {
                    (ParserState::NearbyTickets, items)
                } else {
                    let new_item = match state {
                        ParserState::Validations => {
                            let validation = line.parse::<Validation>().expect(&format!(
                                "Should be able to parse `{}` as Validation",
                                line
                            ));

                            Either::Left(validation)
                        }
                        ParserState::YourTicket => Either::Right(Either::Left(
                            line.parse::<Ticket>()
                                .expect(&format!("Should be able to parse `{}` as Ticket", line)),
                        )),
                        ParserState::NearbyTickets => Either::Right(Either::Right(
                            line.parse::<Ticket>()
                                .expect(&format!("Should be able to parse `{}` as Ticket", line)),
                        )),
                    };

                    items.push(new_item);

                    (state, items)
                }
            },
        );

    items
}

pub fn star_one(input: &str) -> usize {
    let items = parse(input);

    let validations: Vec<_> = items.iter().filter_map(|i| i.clone().left()).collect();
    let nearby_tickets: Vec<_> = items
        .into_iter()
        .filter_map(|i| i.right().and_then(Either::right))
        .collect();

    nearby_tickets
        .into_iter()
        .map(|ticket| {
            ticket
                .digits
                .iter()
                .map(|&d| {
                    let is_valid = validations.iter().any(|v| v.is_valid(d));

                    if is_valid {
                        0
                    } else {
                        d
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn star_two(input: &str) -> usize {
    let items = parse(input);

    let validations: Vec<_> = items.iter().filter_map(|i| i.clone().left()).collect();
    let mut nearby_tickets: Vec<_> = items
        .iter()
        .filter_map(|i| i.clone().right().and_then(Either::right))
        .filter(|t| {
            t.digits
                .iter()
                .all(|&d| validations.iter().any(|v| v.is_valid(d)))
        })
        .collect();
    let your_ticket: _ = items
        .iter()
        .filter_map(|i| i.clone().right().and_then(Either::left))
        .nth(0)
        .unwrap();
    nearby_tickets.push(your_ticket.clone());

    // All possibly candidates for each digit position
    let mut candidates: Vec<HashSet<Validation>> = (0..nearby_tickets[0].digits.len())
        .map(|idx| {
            validations
                .iter()
                .filter(|v| nearby_tickets.iter().all(|t| v.is_valid(t.digits[idx])))
                .map(|v| v.clone())
                .collect()
        })
        .collect();

    // The candidate indices sorted from least number of candidates
    // to most
    let sorted_indices: Vec<_> = {
        let mut indices: Vec<_> = candidates
            .iter()
            .enumerate()
            .map(|(original_idx, c)| (original_idx, c.len()))
            .collect();
        indices.sort_by_key(|(_, l)| *l);

        indices.into_iter().map(|(idx, _)| idx).collect()
    };

    for sorted_idx in 0..candidates.len() {
        let idx = sorted_indices[sorted_idx];
        let validation = {
            let c = &candidates[idx];

            // This algorithm relies on the fact that there's always
            // a set of candidates of length 1 to select
            assert!(
                c.len() == 1,
                "Expected a single candidate found {:?} for {}",
                c,
                idx
            );
            c.iter().cloned().nth(0).unwrap()
        };

        for other_idx in (sorted_idx + 1)..candidates.len() {
            candidates[sorted_indices[other_idx]].remove(&validation);
        }
    }

    let validations: Vec<_> = candidates
        .into_iter()
        .map(|c| c.into_iter().nth(0).unwrap())
        .collect();

    validations
        .iter()
        .enumerate()
        .filter(|(_, v)| v.category.starts_with("departure"))
        .map(|(idx, _)| your_ticket.digits[idx])
        .product()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 71);
    }
}
