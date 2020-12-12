use crate::iterator_ext::IteratorExt;

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

pub struct Map {
    trees: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Map {
    fn count_trees_on_slope(&self, right: usize, down: usize) -> usize {
        let width = self.width;
        let trees = &self.trees;

        (0..=self.height)
            .step_by(down)
            .enumerate()
            .count_if(|(idx, y)| {
                let x = (idx * right) % width;
                trees.contains(&(x, y))
            })
    }
}

const TREE: char = '#';

#[aoc_generator(day3)]
pub fn generate(inp: &str) -> Map {
    let height = inp.lines().count();
    let width = inp.lines().next().map(|l| l.chars().count()).unwrap_or(0);

    let trees = inp
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(idx, chr)| if chr == TREE { Some(idx) } else { None })
                .for_each(|x| {
                    acc.insert((x, y));
                });

            acc
        });

    Map {
        trees,
        width,
        height,
    }
}

#[aoc(day3, part1)]
pub fn part1(m: &Map) -> usize {
    m.count_trees_on_slope(3, 1)
}

#[aoc(day3, part2)]
pub fn part2(m: &Map) -> usize {
    const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    SLOPES
        .iter()
        .map(|&(right, down)| m.count_trees_on_slope(right, down))
        .product()
}
