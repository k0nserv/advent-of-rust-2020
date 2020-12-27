use crate::parser_combinators::*;

pub fn star_one(input: &str) -> isize {
    // expr   <- term (('*' / '+') term)*
    // term   <- (number / '(' expr ')')
    // number <- [0-9]+

    fn term(input: &str) -> ParserResult<&str, isize> {
        fn sub_expression(input: &str) -> ParserResult<&str, isize> {
            let (rest, _) = match_char('(')(input)?;
            let (rest, sub_expression) = {
                let mut l = 0;

                take_while(move |s| {
                    if s == "(" {
                        l += 1;
                        true
                    } else if s == ")" {
                        if l == 0 {
                            false
                        } else {
                            l -= 1;
                            true
                        }
                    } else {
                        true
                    }
                })(rest)?
            };
            let (_, expression) = expr(sub_expression)?;
            let (rest, _) = match_char(')')(rest)?;

            Ok((rest, expression))
        }

        fn number_or_sub_expression(input: &str) -> ParserResult<&str, isize> {
            one_of_2(integer, sub_expression)(input)
        }

        number_or_sub_expression(input)
    }

    fn expr(input: &str) -> ParserResult<&str, isize> {
        let (rest, first) = term(input)?;

        fn expressions(input: &str) -> ParserResult<&str, (isize, Option<String>)> {
            let (rest, op) = one_of_2(match_char('*'), match_char('+'))(input)?;
            let (rest, v) = term(rest)?;

            Ok((rest, (v, Some(op.to_string()))))
        }

        let (rest, results) = many0(expressions)(rest)?;

        Ok((
            rest,
            std::iter::once((first, None))
                .chain(results.into_iter())
                .fold(0, |acc, (v, op)| match op {
                    Some(op) => match op.as_ref() {
                        "+" => acc + v,
                        "*" => acc * v,
                        _ => panic!("Invalid op `{}`", op),
                    },
                    None => v,
                }),
        ))
    };

    input
        .lines()
        .filter_map(|l| {
            let trimmed = l.trim();

            if trimmed.len() > 0 {
                Some(trimmed)
            } else {
                None
            }
        })
        .map(|line| {
            let stripped: String = line.chars().filter(|c| !c.is_whitespace()).collect();

            expr(&stripped)
                .expect("Should be able to parse all expressions")
                .1
        })
        .sum()
}

pub fn star_two(input: &str) -> isize {
    // expr   <- term ('*' term)*
    // term   <- (number / '(' expr ')') ('+' number / '(' expr ')')*
    // number <- [0-9]+

    fn term(input: &str) -> ParserResult<&str, isize> {
        fn sub_expression(input: &str) -> ParserResult<&str, isize> {
            let (rest, _) = match_char('(')(input)?;
            let (rest, sub_expression) = {
                let mut l = 0;

                take_while(move |s| {
                    if s == "(" {
                        l += 1;
                        true
                    } else if s == ")" {
                        if l == 0 {
                            false
                        } else {
                            l -= 1;
                            true
                        }
                    } else {
                        true
                    }
                })(rest)?
            };
            let (_, expression) = expr(sub_expression)?;
            let (rest, _) = match_char(')')(rest)?;

            Ok((rest, expression))
        }

        fn number_or_sub_expression(input: &str) -> ParserResult<&str, isize> {
            one_of_2(integer, sub_expression)(input)
        }
        let (rest, first) = number_or_sub_expression(input)?;

        let (rest, results) = many0(move |input: &str| -> ParserResult<&str, isize> {
            let (rest, _) = match_char('+')(input)?;
            number_or_sub_expression(rest)
        })(rest)?;

        Ok((
            rest,
            std::iter::once(first).chain(results.into_iter()).sum(),
        ))
    }

    fn expr(input: &str) -> ParserResult<&str, isize> {
        let (rest, first) = term(input)?;

        let (rest, results) = many0(move |input: &str| -> ParserResult<&str, isize> {
            let (rest, _) = match_char('*')(input)?;
            term(rest)
        })(rest)?;

        Ok((
            rest,
            std::iter::once(first).chain(results.into_iter()).product(),
        ))
    };

    input
        .lines()
        .filter_map(|l| {
            let trimmed = l.trim();

            if trimmed.len() > 0 {
                Some(trimmed)
            } else {
                None
            }
        })
        .map(|line| {
            let stripped: String = line.chars().filter(|c| !c.is_whitespace()).collect();

            expr(&stripped)
                .expect("Should be able to parse all expressions")
                .1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("2 * 3 + (4 * 5)"), 26);
        assert_eq!(star_one("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(star_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            star_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(star_two("2 * 3 + (4 * 5)"), 46);
        assert_eq!(star_two("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            star_two("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            star_two("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
