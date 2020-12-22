use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display as PDisplay, FromStr as PFromStr};

use crate::iterator_ext::IteratorExt;

#[derive(PDisplay, PFromStr)]
#[display("{min}-{max} {chr}")]
pub struct PasswordPolicy {
    min: usize,
    max: usize,
    chr: char,
}

#[derive(PDisplay, PFromStr)]
#[display("{policy}: {password}")]
pub struct PasswordData {
    policy: PasswordPolicy,
    password: String,
}

impl PasswordPolicy {
    pub fn matches_p1(&self, password: &str) -> bool {
        let num_chars = password.chars().count_if(|it| it == self.chr);
        (self.min..=self.max).contains(&num_chars)
    }

    pub fn matches_p2(&self, password: &str) -> bool {
        let first = password.chars().nth(self.min - 1);
        let second = password.chars().nth(self.max - 1);

        match (first, second) {
            (Some(f), Some(s)) => (f == self.chr) ^ (s == self.chr),
            _ => false,
        }
    }
}

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<PasswordData> {
    inp.lines()
        .filter_map(|it| it.parse().map_err(|e| println!("Error: {}", e)).ok())
        .collect()
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

    fn check_p1(pw: &str) -> bool {
        pw.parse::<PasswordData>()
            .map(|pd| pd.policy.matches_p1(&pd.password))
            .unwrap_or(false)
    }

    fn check_p2(pw: &str) -> bool {
        pw.parse::<PasswordData>()
            .map(|pd| pd.policy.matches_p2(&pd.password))
            .unwrap_or(false)
    }

    #[test]
    fn test_policy_part1() {
        assert!(check_p1("1-2 a: aab"));
        assert!(check_p1("1-2 a: aba"));

        assert!(!check_p1("1-2 a: bbc"));
        assert!(!check_p1("1-2 a: aaa"));
    }

    #[test]
    fn test_policy_part2() {
        assert!(check_p2("1-2 a: abcd"));
        assert!(check_p2("1-2 a: dabc"));

        assert!(!check_p2("1-2 a: aabc"));
    }
}
