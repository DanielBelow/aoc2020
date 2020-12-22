use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use map_data::{run_simulation_until_stable, MapData, MapTile, SimulationContext};

mod map_data;

#[aoc_generator(day11)]
pub fn generate(inp: &str) -> MapData {
    let height = inp.lines().count();

    let mut width = 0;

    let elements = inp
        .lines()
        .enumerate()
        .fold(HashMap::new(), |acc, (row, it)| {
            width = it.len();

            it.chars().enumerate().fold(acc, |mut acc, (col, chr)| {
                match chr {
                    '#' => acc.insert((row, col), MapTile::Occupied),
                    'L' => acc.insert((row, col), MapTile::Empty),
                    '.' => acc.insert((row, col), MapTile::Floor),
                    _ => panic!("Invalid char in input"),
                };

                acc
            })
        });

    MapData::new(width, height, elements)
}

#[aoc(day11, part1)]
pub fn part1(map: &MapData) -> Option<usize> {
    let context = SimulationContext::new(true, 4);
    run_simulation_until_stable(map, &context)
}

#[aoc(day11, part2)]
pub fn part2(map: &MapData) -> Option<usize> {
    let context = SimulationContext::new(false, 5);
    run_simulation_until_stable(map, &context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let inp = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let data = generate(inp);
        assert_eq!(Some(37), part1(&data));
    }

    #[test]
    fn test_sample_part2() {
        let inp = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let data = generate(inp);
        assert_eq!(Some(26), part2(&data));
    }
}
