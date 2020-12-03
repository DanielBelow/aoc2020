use crate::iterator_ext::IteratorExt;

use parse_display::{Display as PDisplay, FromStr as PFromStr};

use aoc_runner_derive::{aoc, aoc_generator};

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
        let first = password.chars().nth(self.min - 1).unwrap();
        let second = password.chars().nth(self.max - 1).unwrap();

        (first == self.chr) ^ (second == self.chr)
    }
}

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<PasswordData> {
    inp.lines().map(|it| it.parse().unwrap()).collect()
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
        let pw_data = pw.parse::<PasswordData>().unwrap();
        pw_data.policy.matches_p1(&pw_data.password)
    }

    fn check_p2(pw: &str) -> bool {
        let pw_data = pw.parse::<PasswordData>().unwrap();
        pw_data.policy.matches_p2(&pw_data.password)
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
