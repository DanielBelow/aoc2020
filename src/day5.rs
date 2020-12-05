use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day5)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.lines()
        .map(String::from)
        .map(|it| get_seat_id(&it))
        .sorted()
        .collect()
}

fn to_seat_id(row: usize, column: usize) -> i64 {
    (row * 8 + column) as i64
}

fn find_in_range(l_bound: usize, u_bound: usize, inp: &str) -> usize {
    let res = inp.chars().fold(l_bound..=u_bound, |range, it| {
        let lower = *range.start();
        let upper = *range.end();

        let half = ((upper - lower) as f32 / 2.0).ceil() as usize;
        match it {
            'F' | 'L' => lower..=(upper - half),
            'B' | 'R' => (lower + half)..=upper,
            _ => panic!("Invalid character"),
        }
    });

    assert_eq!(res.start(), res.end());
    *res.start()
}

fn find_seat(inp: &str) -> (usize, usize) {
    let row = find_in_range(0, 127, &inp[..7]);
    let column = find_in_range(0, 7, &inp[7..]);

    (row, column)
}

fn get_seat_id(inp: &str) -> i64 {
    let (row, col) = find_seat(inp);
    to_seat_id(row, col)
}

#[aoc(day5, part1)]
pub fn part1(boarding_passes: &[i64]) -> Option<i64> {
    boarding_passes.last().copied()
}

#[aoc(day5, part2)]
pub fn part2(boarding_passes: &[i64]) -> Option<i64> {
    boarding_passes
        .iter()
        .sorted()
        .as_slice()
        .windows(2)
        .find_map(|it| {
            if (it[0] - it[1]).abs() == 2 {
                Some((it[0] + it[1]) / 2)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let inp = "FBFBBFFRLR";

        let (row, col) = find_seat(inp);

        assert_eq!(44, row);
        assert_eq!(5, col);

        assert_eq!(357, to_seat_id(row, col));
    }

    #[test]
    fn test_sample_2() {
        let inp = "BFFFBBFRRR";

        let (row, col) = find_seat(inp);

        assert_eq!(70, row);
        assert_eq!(7, col);

        assert_eq!(567, to_seat_id(row, col));
    }

    #[test]
    fn test_sample_3() {
        let inp = "FFFBBBFRRR";

        let (row, col) = find_seat(inp);

        assert_eq!(14, row);
        assert_eq!(7, col);

        assert_eq!(119, to_seat_id(row, col));
    }

    #[test]
    fn test_sample_4() {
        let inp = "BBFFBBFRLL";

        let (row, col) = find_seat(inp);

        assert_eq!(102, row);
        assert_eq!(4, col);

        assert_eq!(820, to_seat_id(row, col));
    }
}
