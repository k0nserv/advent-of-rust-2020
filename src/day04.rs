use std::collections::HashMap;
use std::ops::RangeInclusive;

const REQUIRED_PROPS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_CM_HEIGHTS: RangeInclusive<usize> = 150..=193;
const VALID_IN_HEIGHTS: RangeInclusive<usize> = 59..=76;
const VALID_EYE_COLORS: &'static [&'static str] =
    &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,

    hgt: String,

    hcl: String,
    ecl: String,

    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn from_str<F>(s: &str, validator: F) -> Result<Self, String>
    where
        F: Fn(&HashMap<&str, &str>) -> bool,
    {
        let props: HashMap<&str, &str> = s
            .trim()
            .split_whitespace()
            .flat_map(|s| {
                let mut parts = s.split(":");

                match (parts.next(), parts.next()) {
                    (Some(key), Some(value)) => Some((key.trim(), value.trim())),
                    _ => None,
                }
            })
            .collect();
        let is_valid =
            REQUIRED_PROPS.iter().all(|key| props.contains_key(key)) && validator(&props);

        match is_valid {
            false => Err(format!("Invalid passport `{}`", s)),
            true => Ok(Self {
                byr: props["byr"].to_owned(),
                iyr: props["iyr"].to_owned(),
                eyr: props["eyr"].to_owned(),

                hgt: props["hgt"].to_owned(),

                hcl: props["hcl"].to_owned(),
                ecl: props["ecl"].to_owned(),

                pid: props["pid"].to_owned(),
                cid: props.get("cid").map(|s| s.to_string()),
            }),
        }
    }
}

fn number_valid(number: Option<&&str>, range: RangeInclusive<usize>) -> bool {
    number
        .and_then(|s| s.parse::<usize>().ok())
        .map(|n| range.contains(&n))
        .unwrap_or(false)
}

fn height_valid(height: Option<&&str>) -> bool {
    match height.and_then(|s| s.strip_suffix("cm")) {
        Some(rest) => rest
            .parse::<usize>()
            .ok()
            .map(|h| VALID_CM_HEIGHTS.contains(&h))
            .unwrap_or(false),
        None => match height.and_then(|s| s.strip_suffix("in")) {
            Some(rest) => rest
                .parse::<usize>()
                .ok()
                .map(|h| VALID_IN_HEIGHTS.contains(&h))
                .unwrap_or(false),
            None => false,
        },
    }
}

fn hair_color_valid(hcl: Option<&&str>) -> bool {
    match hcl.and_then(|s| s.strip_prefix('#')) {
        Some(rest) => u32::from_str_radix(rest, 16).is_ok(),
        None => false,
    }
}

fn pid_valid(pid: Option<&&str>) -> bool {
    pid.map(|p| p.matches(char::is_numeric).count() == 9)
        .unwrap_or(false)
}

fn ecl_valid(ecl: Option<&&str>) -> bool {
    ecl.map(|ecl| VALID_EYE_COLORS.contains(ecl))
        .unwrap_or(false)
}

fn fields_valid(props: &HashMap<&str, &str>) -> bool {
    [
        number_valid(props.get("byr"), 1920..=2002),
        number_valid(props.get("iyr"), 2010..=2020),
        number_valid(props.get("eyr"), 2020..=2030),
        height_valid(props.get("hgt")),
        hair_color_valid(props.get("hcl")),
        ecl_valid(props.get("ecl")),
        pid_valid(props.get("pid")),
    ]
    .iter()
    .all(|&x| x)
}

pub fn star_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .flat_map(|l| Passport::from_str(l, |_| true))
        .count()
}

pub fn star_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .flat_map(|l| Passport::from_str(l, fields_valid))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{fields_valid, star_one, star_two, Passport};
    const INPUT: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INVALID_PART_TWO: &'static str = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID_PART_TWO: &'static str = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 2);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }

    #[test]
    fn test_invalid_passports() {
        for p in INVALID_PART_TWO
            .split("\n\n")
            .map(str::trim)
            .filter(|l| l.len() > 0)
        {
            assert_eq!(Passport::from_str(p, fields_valid).is_ok(), false);
        }
    }

    #[test]
    fn test_valid_passports() {
        for p in VALID_PART_TWO
            .split("\n\n")
            .map(str::trim)
            .filter(|l| l.len() > 0)
        {
            assert_eq!(
                Passport::from_str(p, fields_valid).is_ok(),
                true,
                "{} should be valid, but wasn't",
                p
            );
        }
    }
}
