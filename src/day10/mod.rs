use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::iterator_ext::IteratorExt;

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<i64> {
    let mut nums = inp
        .lines()
        .filter_map(|it| it.parse().map_err(|e| println!("Error: {}", e)).ok())
        .sorted()
        .collect_vec();

    let max = nums.iter().max().copied().unwrap();
    nums.insert(0, 0);
    nums.push(max + 3);

    nums
}

fn count_jolts(jolts: &[i64]) -> i64 {
    let mut cur_outlet_value = 0;

    let (num_one, num_three) = jolts.iter().fold((0, 0), |(ones, threes), it| {
        let diff = *it - cur_outlet_value;
        cur_outlet_value = *it;

        match diff {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        }
    });

    num_one * num_three
}

#[aoc(day10, part1)]
pub fn part1(jolts: &[i64]) -> i64 {
    count_jolts(jolts)
}

fn count_paths(v: &[i64], idx: usize, cache: &mut HashMap<usize, i64>) -> i64 {
    if let Some(val) = cache.get(&idx) {
        return *val;
    }

    if idx == v.len() - 1 {
        return 1;
    }

    let cur_elem = v[idx];

    let num_paths = v
        .iter()
        .enumerate()
        .filter_map(|(idx, it)| {
            if *it > cur_elem && *it <= cur_elem + 3 {
                Some(idx)
            } else {
                None
            }
        })
        .sum_by(|it| count_paths(v, it, cache));

    cache.insert(idx, num_paths);

    num_paths
}

#[aoc(day10, part2)]
pub fn part2(jolts: &[i64]) -> i64 {
    count_paths(jolts, 0, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_small() -> Vec<i64> {
        let mut res = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        res.push(0);
        res.push(22); // 19 + 3
        res.sort_unstable();
        res
    }

    fn input_large() -> Vec<i64> {
        let mut res = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        res.push(0);
        res.push(52); // 49 + 3
        res.sort_unstable();
        res
    }

    #[test]
    fn test_sample_part1() {
        assert_eq!(35, part1(&input_small()));
        assert_eq!(220, part1(&input_large()));
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(8, part2(&input_small()));
        assert_eq!(19208, part2(&input_large()));
    }
}
