use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::iterator_ext::IteratorExt;

pub struct Group {
    answers: HashMap<char, usize>,
    num_people: usize,
}

#[aoc_generator(day6)]
pub fn generate(inp: &str) -> Vec<Group> {
    inp.split("\n\n")
        .map(|it| to_group(it.lines().collect()))
        .collect()
}

fn to_group(inp: Vec<&str>) -> Group {
    let answers = get_answers_map(&inp);
    let num_people = inp.len();
    Group {
        answers,
        num_people,
    }
}

fn get_answers_map(group_answers: &[&str]) -> HashMap<char, usize> {
    group_answers.iter().fold(HashMap::new(), |acc, it| {
        it.chars().fold(acc, |mut acc, it| {
            acc.entry(it).and_modify(|it| *it += 1).or_insert(1);
            acc
        })
    })
}

#[aoc(day6, part1)]
pub fn part1(groups: &[Group]) -> usize {
    groups.iter().sum_by(|it| it.answers.values().count())
}

#[aoc(day6, part2)]
pub fn part2(groups: &[Group]) -> usize {
    groups
        .iter()
        .sum_by(|it| it.answers.values().count_if(|v| *v == it.num_people))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let inp = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        let data = generate(inp);
        assert_eq!(11, part1(&data));
        assert_eq!(6, part2(&data));
    }
}
