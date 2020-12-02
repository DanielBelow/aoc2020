use aoc_runner_derive::{aoc, aoc_generator};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<Min>\d+)-(?P<Max>\d+) (?P<Char>[A-Za-z]): (?P<Text>.*)").unwrap();
}

pub struct PasswordPolicy {
    min: usize,
    max: usize,
    chr: char,
}

impl PasswordPolicy {
    pub fn new(min: usize, max: usize, chr: char) -> Self {
        Self { min, max, chr }
    }

    pub fn matches_p1(&self, inp: &str) -> bool {
        let num_chars = inp.chars().filter(|it| *it == self.chr).count();
        (self.min..=self.max).contains(&num_chars)
    }

    pub fn matches_p2(&self, inp: &str) -> bool {
        let first = inp.chars().nth(self.min - 1).unwrap();
        let second = inp.chars().nth(self.max - 1).unwrap();

        (first == self.chr) ^ (second == self.chr)
    }
}

pub struct PasswordData {
    policy: PasswordPolicy,
    password: String,
}

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<PasswordData> {
    inp.lines()
        .map(|it| {
            assert!(RE.is_match(it));

            let captures = RE.captures(it).unwrap();

            let policy = PasswordPolicy::new(
                captures["Min"].parse::<usize>().unwrap(),
                captures["Max"].parse::<usize>().unwrap(),
                captures["Char"].parse::<char>().unwrap(),
            );

            PasswordData {
                policy,
                password: String::from(&captures["Text"]),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(v: &[PasswordData]) -> usize {
    v.iter().fold(0, |acc, it| {
        acc + it.policy.matches_p1(&it.password) as usize
    })
}

#[aoc(day2, part2)]
pub fn part2(v: &[PasswordData]) -> usize {
    v.iter().fold(0, |acc, it| {
        acc + it.policy.matches_p2(&it.password) as usize
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_part1() {
        let policy = PasswordPolicy::new(1, 2, 'a');
        assert!(policy.matches_p1("aab"));
        assert!(policy.matches_p1("abc"));

        assert!(!policy.matches_p1("bbbb"));
        assert!(!policy.matches_p1("aaaa"));
    }

    #[test]
    fn test_policy_part2() {
        let policy = PasswordPolicy::new(1, 2, 'a');
        assert!(policy.matches_p2("abcd"));
        assert!(policy.matches_p2("dabc"));

        assert!(!policy.matches_p2("aabc"));
    }
}
