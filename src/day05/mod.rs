use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day5)]
pub fn generate(inp: &str) -> Vec<usize> {
    inp.lines()
        .map(String::from)
        .filter_map(|it| {
            get_seat_id(&it)
                .map_err(|it| println!("Error: {}", it))
                .ok()
        })
        .sorted()
        .collect()
}

fn find_in_range(inp: &str) -> Result<usize, ParseIntError> {
    let inp = inp
        .replace('F', "0")
        .replace('L', "0")
        .replace('B', "1")
        .replace('R', "1");

    usize::from_str_radix(&inp, 2)
}

fn get_seat_id(inp: &str) -> Result<usize, ParseIntError> {
    let row = find_in_range(&inp[..7])?;
    let column = find_in_range(&inp[7..])?;

    Ok(row * 8 + column)
}

#[aoc(day5, part1)]
pub fn part1(boarding_passes: &[usize]) -> Option<usize> {
    boarding_passes.last().copied()
}

fn find_missing_seat(boarding_passes: &[usize]) -> Option<usize> {
    let first_seat = boarding_passes.first();
    let last_seat = boarding_passes.last();

    let expected_sum = first_seat.and_then(|fs| last_seat.map(|ls| (*fs..=*ls).sum::<usize>()));
    let actual_sum = boarding_passes.iter().sum::<usize>();

    expected_sum.map(|it| it - actual_sum)
}

#[aoc(day5, part2)]
pub fn part2(boarding_passes: &[usize]) -> Option<usize> {
    find_missing_seat(boarding_passes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_ids() {
        assert_eq!(Ok(357), get_seat_id("FBFBBFFRLR"));
        assert_eq!(Ok(567), get_seat_id("BFFFBBFRRR"));
        assert_eq!(Ok(119), get_seat_id("FFFBBBFRRR"));
        assert_eq!(Ok(820), get_seat_id("BBFFBBFRLL"));
    }

    #[test]
    fn test_missing_int() {
        assert_eq!(Some(2), find_missing_seat(&[1, 3, 4, 5]));
    }
}
