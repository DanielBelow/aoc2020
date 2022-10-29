use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> Option<Vec<usize>> {
    inp.lines().next().map(|it| {
        it.split(',')
            .filter_map(|it| it.parse::<usize>().ok())
            .collect()
    })
}

fn setup_starting_numbers(v: &[usize]) -> HashMap<usize, usize> {
    v.iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, it)| {
            acc.insert(*it, idx);
            acc
        })
}

fn play_n_rounds(v: &[usize], rounds: usize) -> Option<usize> {
    let mut cache = setup_starting_numbers(v);

    let mut last_spoken = *v.last()?;
    for iter in v.len() - 1..rounds - 1 {
        let prev_spoken = cache.get(&last_spoken).copied();

        cache.insert(last_spoken, iter);
        match prev_spoken {
            None => last_spoken = 0,
            Some(idx) => last_spoken = iter - idx,
        }
    }

    Some(last_spoken)
}

#[aoc(day15, part1)]
pub fn part1(v: &[usize]) -> Option<usize> {
    play_n_rounds(v, 2020)
}

#[aoc(day15, part2)]
pub fn part2(v: &[usize]) -> Option<usize> {
    play_n_rounds(v, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_samples(samples: &[(&str, usize)], rounds: usize) {
        for (inp, exp) in samples {
            let data = generate(inp);
            assert!(data.is_some());
            assert_eq!(Some(*exp), play_n_rounds(&data.unwrap(), rounds));
        }
    }

    #[test]
    fn test_part1_samples() {
        let samples = &[
            ("0,3,6", 436),
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ];

        test_samples(samples, 2020);
    }

    #[test]
    fn test_part2() {
        let samples = &[
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];

        test_samples(samples, 30_000_000);
    }
}
