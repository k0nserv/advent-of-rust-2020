use std::str::FromStr;

pub type ParserResult<I, O> = Result<(I, O), String>;

pub fn skip_whitespace0(input: &str) -> ParserResult<&str, ()> {
    Ok((input.trim_start(), ()))
}

pub fn take_while<F>(mut f: F) -> impl FnMut(&str) -> ParserResult<&str, &str>
where
    F: FnMut(&str) -> bool,
{
    move |input: &str| {
        for i in 0..input.len() {
            // This will break with non ASCII
            if !f(&input[i..i + 1]) {
                return Ok((&input[i..], &input[0..i]));
            }
        }

        Ok(("", input))
    }
}

pub fn integer<T: FromStr>(input: &str) -> ParserResult<&str, T> {
    let first_non_digit_index = input
        .find(|c: char| !c.is_numeric())
        .unwrap_or_else(|| input.len());

    let number = input[0..first_non_digit_index].parse::<T>();

    number
        .map(|n| (&input[first_non_digit_index..], n))
        .map_err(|_| {
            format!(
                "Failed to parse integer `{}`",
                &input[0..first_non_digit_index],
            )
        })
}

pub fn match_char(expected: char) -> impl FnMut(&str) -> ParserResult<&str, &str> {
    move |input: &str| {
        input
            .chars()
            .nth(0)
            .map(|c| c == expected)
            .ok_or_else(|| format!("Cannot match `{}` in empty string", expected))
            .and_then(|matched| {
                if matched {
                    Ok((&input[1..], &input[0..1]))
                } else {
                    Err(format!("Did not match `{}` in `{}`", expected, input))
                }
            })
    }
}

pub fn one_of_2<'a, P1, P2, O>(
    mut p1: P1,
    mut p2: P2,
) -> impl FnMut(&'a str) -> ParserResult<&'a str, O>
where
    P1: FnMut(&'a str) -> ParserResult<&'a str, O>,
    P2: FnMut(&'a str) -> ParserResult<&'a str, O>,
{
    move |input: &str| {
        if let Ok(p1_result) = p1(input) {
            Ok(p1_result)
        } else if let Ok(p2_result) = p2(input) {
            Ok(p2_result)
        } else {
            Err(format!("Failed to match one_of_2 in `{}`", input))
        }
    }
}

pub fn one_of_3<'a, P1, P2, P3, O>(
    mut p1: P1,
    mut p2: P2,
    mut p3: P3,
) -> impl FnMut(&'a str) -> ParserResult<&'a str, O>
where
    P1: FnMut(&'a str) -> ParserResult<&'a str, O>,
    P2: FnMut(&'a str) -> ParserResult<&'a str, O>,
    P3: FnMut(&'a str) -> ParserResult<&'a str, O>,
{
    move |input: &str| {
        if let Ok(p1_result) = p1(input) {
            Ok(p1_result)
        } else if let Ok(p2_result) = p2(input) {
            Ok(p2_result)
        } else if let Ok(p3_result) = p3(input) {
            Ok(p3_result)
        } else {
            Err(format!("Failed to match one_of_3 in `{}`", input))
        }
    }
}

pub fn eof(input: &str) -> ParserResult<&str, ()> {
    if input.len() == 0 {
        Ok((input, ()))
    } else {
        Err(format!("Expected `eof` found `{}`", input))
    }
}

pub fn enrich<I, O, P, F>(
    mut parser: P,
    mut enricher: F,
) -> impl FnMut(&str) -> ParserResult<&str, O>
where
    P: FnMut(&str) -> ParserResult<&str, I>,
    F: FnMut(I) -> O,
{
    move |input: &str| parser(input).map(|(rest, output)| (rest, enricher(output)))
}

pub fn many0<P, O>(mut parser: P) -> impl FnMut(&str) -> ParserResult<&str, Vec<O>>
where
    P: FnMut(&str) -> ParserResult<&str, O>,
{
    move |input: &str| {
        let mut result = vec![];
        let mut rest = input;

        while rest.len() > 0 {
            match parser(rest) {
                Ok((new_rest, r)) => {
                    result.push(r);
                    rest = new_rest;
                }
                Err(_) => return Ok((rest, result)),
            }
        }

        Ok((rest, result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_whitespace0() {
        assert_eq!(skip_whitespace0("     a"), Ok(("a", ())));
        assert_eq!(skip_whitespace0("a"), Ok(("a", ())));
    }

    #[test]
    fn test_integer() {
        assert_eq!(integer("123 +982"), Ok((" +982", 123)));
        assert_eq!(
            integer::<isize>("a123 +982"),
            Err("Failed to parse integer ``".to_string())
        );
        assert_eq!(
            integer::<u8>("1234"),
            Err("Failed to parse integer `1234`".to_string())
        );
        assert_eq!(integer::<isize>("9+3*4*3"), Ok(("+3*4*3", 9)));
    }

    #[test]
    fn test_match_char() {
        let mut match_plus = match_char('+');

        assert_eq!(match_plus("+ 923"), Ok((" 923", "+")));
    }

    #[test]
    fn test_one_of_2() {
        let mut match_plus_or_minus = one_of_2(match_char('+'), match_char('-'));

        assert_eq!(match_plus_or_minus("+ 923"), Ok((" 923", "+")));
        assert_eq!(match_plus_or_minus("- 923"), Ok((" 923", "-")));
        assert_eq!(
            match_plus_or_minus("923"),
            Err("Failed to match one_of_2 in `923`".to_string())
        );
    }

    #[test]
    fn test_one_of_3() {
        let mut match_op = one_of_3(match_char('+'), match_char('-'), match_char('*'));

        assert_eq!(match_op("+ 923"), Ok((" 923", "+")));
        assert_eq!(match_op("- 923"), Ok((" 923", "-")));
        assert_eq!(match_op("* 923"), Ok((" 923", "*")));
        assert_eq!(
            match_op("923"),
            Err("Failed to match one_of_3 in `923`".to_string())
        );
    }

    #[test]
    fn test_eof() {
        assert_eq!(eof(""), Ok(("", ())));
        assert_eq!(eof("a"), Err("Expected `eof` found `a`".to_string()));
    }

    #[test]
    fn test_enrich() {
        let mut parser = enrich(integer::<usize>, |x| x * 10);

        assert_eq!(parser("12 + 19"), Ok((" + 19", 120)));
    }

    #[test]
    fn test_take_while() {
        let mut parser = take_while(|s| s != "(" && s != ")");

        assert_eq!(parser("5+8+3)+3)+(3+9*7)"), Ok((")+3)+(3+9*7)", "5+8+3")));
    }

    #[test]
    fn test_many0() {
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

        assert_eq!(expr("1"), Ok(("", 1)));
        assert_eq!(expr("1+2+3+4"), Ok(("", 10)));
        assert_eq!(expr("1+2*3+4"), Ok(("", 21)));
        assert_eq!(expr("1+(2*3)+4"), Ok(("", 11)));
        assert_eq!(expr("((2+4*9)*(6+9*8+6)+6)+2+4*2"), Ok(("", 23340)));
    }
}
