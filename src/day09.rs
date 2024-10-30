use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.lines()
        .filter_map(|it| it.parse().map_err(|e| println!("Error: {e}")).ok())
        .collect()
}

const fn contains_sum(num: i64, slice: &[i64]) -> bool {
    let mut low = 0;
    let mut high = slice.len() - 1;

    while low < high {
        let elem_sum = slice[low] + slice[high];
        if elem_sum == num {
            return true;
        }

        if elem_sum > num {
            high -= 1;
        } else {
            low += 1;
        }
    }

    false
}

fn find_elem_without_sum(slice: &[i64], preamble_size: usize) -> Option<i64> {
    let last_num = slice.last()?;

    let mut slice = slice[..preamble_size].to_vec();
    slice.sort_unstable();

    if contains_sum(*last_num, &slice) {
        None
    } else {
        Some(*last_num)
    }
}

fn find_invalid_number(numbers: &[i64], preamble_size: usize) -> Option<i64> {
    numbers
        .iter()
        .as_slice()
        .windows(preamble_size + 1)
        .skip(preamble_size)
        .find_map(|it| find_elem_without_sum(it, preamble_size))
}

#[aoc(day9, part1)]
pub fn part1(numbers: &[i64]) -> Option<i64> {
    find_invalid_number(numbers, 25)
}

fn find_invalid_sum(numbers: &[i64], to_find: i64) -> Option<i64> {
    let find_idx = numbers.iter().position(|it| *it == to_find)?;

    (3..find_idx).find_map(|win_size| {
        numbers.windows(win_size).find_map(|it| {
            if it.iter().sum::<i64>() == to_find {
                let (min, max) = it.iter().minmax().into_option()?;
                Some(*min + *max)
            } else {
                None
            }
        })
    })
}

#[aoc(day9, part2)]
pub fn part2(numbers: &[i64]) -> Option<i64> {
    find_invalid_sum(numbers, 400_480_901)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let inp: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(Some(127), find_invalid_number(&inp, 5));
        assert_eq!(Some(62), find_invalid_sum(&inp, 127));
    }
}
