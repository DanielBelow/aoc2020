use crate::iterator_ext::IteratorExt;

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
        let num_chars = inp.chars().count_if(|it| it == self.chr);
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

fn to_password_data(line: &str) -> PasswordData {
    assert!(RE.is_match(line));

    let captures = RE.captures(line).unwrap();

    let policy = PasswordPolicy::new(
        captures["Min"].parse().unwrap(),
        captures["Max"].parse().unwrap(),
        captures["Char"].parse().unwrap(),
    );

    PasswordData {
        policy,
        password: String::from(&captures["Text"]),
    }
}

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<PasswordData> {
    inp.lines().map(to_password_data).collect()
}

#[aoc(day2, part1)]
pub fn part1(v: &[PasswordData]) -> usize {
    v.iter().count_if(|it| it.policy.matches_p1(&it.password))
}

#[aoc(day2, part2)]
pub fn part2(v: &[PasswordData]) -> usize {
    v.iter().count_if(|it| it.policy.matches_p2(&it.password))
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
