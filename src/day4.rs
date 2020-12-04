use crate::iterator_ext::IteratorExt;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref WS_RE: Regex = Regex::new(r"\s{2,}").unwrap();
}

pub struct PassportData {
    fields: HashMap<String, String>,
}

#[derive(PDisplay, PFromStr)]
#[display("{key}:{value}")]
struct KeyValuePair {
    key: String,
    value: String,
}

#[aoc_generator(day4)]
pub fn generate(inp: &str) -> Vec<PassportData> {
    inp.lines()
        .join("\n")
        .split("\n\n")
        .map(|it| {
            let data = it.replace("\n", " ");
            let fields = WS_RE
                .replace_all(data.as_str().trim(), " ")
                .split(&" ")
                .fold(HashMap::new(), |mut acc, spl| {
                    let kvp = spl.parse::<KeyValuePair>().unwrap();
                    acc.insert(kvp.key, kvp.value);
                    acc
                });

            PassportData { fields }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(pd: &[PassportData]) -> usize {
    pd.iter().count_if(|it| {
        it.fields.len() == 8
            || (it.fields.len() == 7 && !it.fields.contains_key(&String::from("cid")))
    })
}

fn validate_range(low: usize, high: usize, value: &str) -> bool {
    value
        .parse::<usize>()
        .ok()
        .filter(|it| (low..=high).contains(&it))
        .is_some()
}

fn validate_height(value: &str) -> bool {
    if !value.ends_with("cm") && !value.ends_with("in") {
        return false;
    }

    let num = value.chars().dropping_back(2).as_str();
    if value.ends_with("cm") {
        validate_range(150, 193, num)
    } else {
        validate_range(59, 76, num)
    }
}

fn is_valid(pd: &PassportData) -> bool {
    const FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    const EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    FIELDS.iter().all(|it| {
        if !pd.fields.contains_key(*it) {
            return false;
        }

        let value = pd.fields.get(*it).unwrap();
        match *it {
            "byr" => validate_range(1920, 2002, value),
            "iyr" => validate_range(2010, 2020, value),
            "eyr" => validate_range(2020, 2030, value),
            "hgt" => validate_height(value),
            "hcl" => HAIR_RE.is_match(value),
            "ecl" => EYE_COLORS.contains(&value.as_str()),
            "pid" => PID_RE.is_match(value),
            _ => false,
        }
    })
}

#[aoc(day4, part2)]
pub fn part2(pd: &[PassportData]) -> usize {
    pd.iter().count_if(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let inp = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                          byr:1937 iyr:2017 cid:147 hgt:183cm

                          iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                          hcl:#cfa07d byr:1929

                          hcl:#ae17e1 iyr:2013
                          eyr:2024
                          ecl:brn pid:760753108 byr:1931
                          hgt:179cm

                          hcl:#cfa07d eyr:2025 pid:166559648
                          iyr:2011 ecl:brn hgt:59in";

        let data = generate(inp);
        assert_eq!(data.len(), 4);

        assert_eq!(part1(&data), 2);
    }
}
