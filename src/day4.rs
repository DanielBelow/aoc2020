use crate::iterator_ext::IteratorExt;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref WS_RE: Regex = Regex::new(r"\s{2,}").unwrap();
}

#[derive(PDisplay, PFromStr)]
pub enum HeightUnit {
    #[display("{0}cm")]
    Centimeter(usize),

    #[display("{0}in")]
    Inch(usize),
}

#[derive(PDisplay, PFromStr, Eq, PartialEq, Debug, Hash)]
pub enum PassportEntry {
    #[display("byr:{0}")]
    BirthYear(usize),

    #[display("iyr:{0}")]
    IssueYear(usize),

    #[display("eyr:{0}")]
    ExpirationYear(usize),

    #[display("hgt:{0}")]
    Height(String),

    #[display("hcl:{0}")]
    HairColor(String),

    #[display("ecl:{0}")]
    EyeColor(String),

    #[display("pid:{0}")]
    PassportID(String),

    #[display("cid:{0}")]
    CountryID(String),
}

fn remove_whitespace(s: &str) -> String {
    let rem_linebreaks = s.replace("\n", " ");
    WS_RE.replace_all(rem_linebreaks.trim(), " ").to_string()
}

#[aoc_generator(day4)]
pub fn generate(inp: &str) -> Vec<HashSet<PassportEntry>> {
    inp.lines()
        .join("\n")
        .split("\n\n")
        .map(remove_whitespace)
        .fold(Vec::new(), |mut acc, it| {
            let entries = it
                .split(&" ")
                .map(|it| it.parse::<PassportEntry>().unwrap())
                .collect::<HashSet<_>>();

            acc.push(entries);
            acc
        })
}

fn has_required_fields(pd: &HashSet<PassportEntry>) -> bool {
    let num_fields = pd.len();
    if num_fields == 8 {
        return true;
    }

    let has_country_id = pd
        .iter()
        .any(|it| matches!(it, PassportEntry::CountryID(_)));

    !has_country_id && num_fields == 7
}

#[aoc(day4, part1)]
pub fn part1(pd: &[HashSet<PassportEntry>]) -> usize {
    pd.iter().count_if(has_required_fields)
}

fn validate_range(low: usize, high: usize, value: usize) -> bool {
    (low..=high).contains(&value)
}

fn validate_height(height: &str) -> bool {
    height
        .parse::<HeightUnit>()
        .ok()
        .map(|it| match it {
            HeightUnit::Centimeter(num) => validate_range(150, 193, num),
            HeightUnit::Inch(num) => validate_range(59, 76, num),
        })
        .unwrap_or(false)
}

fn is_valid(pd: &HashSet<PassportEntry>) -> bool {
    const EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    pd.iter().all(|it| match it {
        PassportEntry::BirthYear(y) => validate_range(1920, 2002, *y),
        PassportEntry::IssueYear(y) => validate_range(2010, 2020, *y),
        PassportEntry::ExpirationYear(y) => validate_range(2020, 2030, *y),
        PassportEntry::Height(h) => validate_height(&h.as_str()),
        PassportEntry::HairColor(cl) => HAIR_RE.is_match(cl),
        PassportEntry::EyeColor(cl) => EYE_COLORS.contains(&cl.as_str()),
        PassportEntry::PassportID(id) => PID_RE.is_match(&id.as_str()),
        PassportEntry::CountryID(_) => true,
    })
}

#[aoc(day4, part2)]
pub fn part2(pd: &[HashSet<PassportEntry>]) -> usize {
    pd.iter()
        .count_if(|it| has_required_fields(it) && is_valid(it))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_fields() {
        let byr = "byr:1929";
        assert_eq!(
            byr.parse::<PassportEntry>().unwrap(),
            PassportEntry::BirthYear(1929)
        );

        let iyr = "iyr:2017";
        assert_eq!(
            iyr.parse::<PassportEntry>().unwrap(),
            PassportEntry::IssueYear(2017)
        );

        let eyr = "eyr:2020";
        assert_eq!(
            eyr.parse::<PassportEntry>().unwrap(),
            PassportEntry::ExpirationYear(2020)
        );

        let hgt = "hgt:183cm";
        assert_eq!(
            hgt.parse::<PassportEntry>().unwrap(),
            PassportEntry::Height("183cm".to_string())
        );

        let hcl = "hcl:#AAACCC";
        assert_eq!(
            hcl.parse::<PassportEntry>().unwrap(),
            PassportEntry::HairColor("#AAACCC".to_string())
        );

        let ecl = "ecl:blu";
        assert_eq!(
            ecl.parse::<PassportEntry>().unwrap(),
            PassportEntry::EyeColor("blu".to_string())
        );

        let pid = "pid:123456789";
        assert_eq!(
            pid.parse::<PassportEntry>().unwrap(),
            PassportEntry::PassportID("123456789".to_string())
        );

        let cid = "cid:147";
        assert_eq!(
            cid.parse::<PassportEntry>().unwrap(),
            PassportEntry::CountryID("147".to_string())
        );
    }

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

    #[test]
    fn test_part2_valid_passports() {
        let inp = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                          hcl:#623a2f

                          eyr:2029 ecl:blu cid:129 byr:1989
                          iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

                          hcl:#888785
                          hgt:164cm byr:2001 iyr:2015 cid:88
                          pid:545766238 ecl:hzl
                          eyr:2022

                          iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let data = generate(inp);
        assert_eq!(4, part2(&data));
    }

    #[test]
    fn test_part2_invalid_passports() {
        let inp = "eyr:1972 cid:100
                   hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

                   iyr:2019
                   hcl:#602927 eyr:1967 hgt:170cm
                   ecl:grn pid:012533040 byr:1946

                   hcl:dab227 iyr:2012
                   ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

                   hgt:59cm ecl:zzz
                   eyr:2038 hcl:74454a iyr:2023
                   pid:3556412378 byr:2007";

        let data = generate(inp);
        assert_eq!(0, part2(&data));
    }
}
