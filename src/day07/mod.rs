use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use regex::Regex;

use crate::iterator_ext::IteratorExt;

lazy_static! {
    static ref CLEAN_BAGS: Regex = Regex::new(r#"(\.|bags|bag)"#).unwrap();
}

#[derive(PDisplay, PFromStr)]
#[display("{amount} {color}")]
struct NestedBag {
    amount: usize,
    color: String,
}

type BagsWithContents = HashMap<String, HashMap<String, usize>>;

fn clean_input_line(line: &str) -> String {
    CLEAN_BAGS.replace_all(line, "").replace("contain", ",")
}

fn parse_nested_bags(line: &[&str]) -> HashMap<String, usize> {
    line.iter()
        .skip(1)
        .fold(HashMap::new(), |mut sub_acc, nested_bag| {
            nested_bag
                .parse::<NestedBag>()
                .ok()
                .map(|it| sub_acc.insert(it.color, it.amount));
            sub_acc
        })
}

#[aoc_generator(day7)]
pub fn generate(inp: &str) -> BagsWithContents {
    inp.lines()
        .map(clean_input_line)
        .fold(HashMap::new(), |mut acc, line| {
            let split = line.split(',').map(|l| l.trim()).collect_vec();

            let nested_bags = parse_nested_bags(&split);

            split
                .first()
                .map(|&it| acc.insert(it.to_string(), nested_bags));

            acc
        })
}

fn can_hold_shiny_gold(
    check: &HashMap<String, usize>,
    all_bags: &BagsWithContents,
    bag_cache: &mut HashMap<String, bool>,
) -> bool {
    if check.contains_key("shiny gold") {
        return true;
    }

    check.keys().any(|k| {
        if let Some(res) = bag_cache.get(k) {
            return *res;
        }

        let res = all_bags
            .get(k)
            .map(|it| can_hold_shiny_gold(it, all_bags, bag_cache))
            .unwrap_or(false);

        bag_cache.insert(k.to_string(), res);

        res
    })
}

#[aoc(day7, part1)]
pub fn part1(bags: &BagsWithContents) -> usize {
    let mut bag_cache = HashMap::new();
    bags.values()
        .count_if(|it| can_hold_shiny_gold(it, bags, &mut bag_cache))
}

fn count_nested_bags(bag: &HashMap<String, usize>, all_bags: &BagsWithContents) -> usize {
    bag.iter().fold(0, |acc, (k, v)| {
        let num_nested = all_bags
            .get(k)
            .map(|it| count_nested_bags(it, all_bags))
            .unwrap_or(0);

        let total_num_nested = *v * num_nested;

        acc + *v + total_num_nested
    })
}

#[aoc(day7, part2)]
pub fn part2(bags: &BagsWithContents) -> Option<usize> {
    bags.get("shiny gold").map(|it| count_nested_bags(it, bags))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let inp = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let data = generate(inp);
        assert_eq!(4, part1(&data));
        assert_eq!(Some(32), part2(&data));
    }

    #[test]
    fn test_sample2() {
        let inp = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let data = generate(inp);
        assert_eq!(Some(126), part2(&data));
    }
}
