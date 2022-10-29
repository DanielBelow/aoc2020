use aoc_runner_derive::{aoc, aoc_generator};

use map_data::{
    run_simulation_steps_3d, run_simulation_steps_4d, Data3d, Data4d, SimulationContext,
};

mod map_data;

const NUM_ITER: usize = 6;

// 8 lines, growing by at most 1 per iteration, for 6 iterations, shifted into the middle
const DIMENSION: usize = (8 + NUM_ITER) * 2;

#[aoc_generator(day17, part1)]
pub fn generate_p1(inp: &str) -> Data3d {
    let midway_pt = DIMENSION / 2;

    let mut elements = vec![vec![vec![false; DIMENSION]; DIMENSION]; DIMENSION];

    for (row, l) in inp.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            elements[midway_pt + row][midway_pt + col][midway_pt] = c == '#';
        }
    }

    Data3d::new(DIMENSION, elements)
}

#[aoc_generator(day17, part2)]
pub fn generate_p2(inp: &str) -> Data4d {
    let midway_pt = DIMENSION / 2;

    let mut elements = vec![vec![vec![vec![false; DIMENSION]; DIMENSION]; DIMENSION]; DIMENSION];

    for (row, l) in inp.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            elements[midway_pt + row][midway_pt + col][midway_pt][midway_pt] = c == '#';
        }
    }

    Data4d::new(DIMENSION, elements)
}

#[aoc(day17, part1)]
pub fn part1(md: &Data3d) -> usize {
    let context = SimulationContext::new(NUM_ITER);
    run_simulation_steps_3d(md, &context)
}

#[aoc(day17, part2)]
pub fn part2(md: &Data4d) -> usize {
    let context = SimulationContext::new(NUM_ITER);
    run_simulation_steps_4d(md, &context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let inp = ".#.
..#
###";

        let data = generate_p1(inp);
        assert_eq!(112, part1(&data));
    }

    #[test]
    fn test_part2_sample() {
        let inp = ".#.
..#
###";

        let data = generate_p2(inp);
        assert_eq!(848, part2(&data));
    }
}
