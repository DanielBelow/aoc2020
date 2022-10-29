use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

pub struct SimulationContext {
    single_step: bool,
    crowd_limit: usize,
}

impl SimulationContext {
    pub fn new(single_step: bool, crowd_limit: usize) -> Self {
        Self {
            single_step,
            crowd_limit,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapTile {
    Floor,
    Empty,
    Occupied,
}

type MapElements = HashMap<(usize, usize), MapTile>;

#[derive(Clone)]
pub struct MapData {
    width: usize,
    height: usize,
    elements: MapElements,
}

impl MapData {
    fn is_valid_pos(&self, row: i64, col: i64) -> bool {
        row >= 0 && col >= 0 && row < self.height as i64 && col < self.width as i64
    }

    fn get_next_state(
        &self,
        row: usize,
        col: usize,
        context: &SimulationContext,
    ) -> Option<MapTile> {
        let cur_seat = self.elements.get(&(row, col))?;
        if *cur_seat == MapTile::Floor {
            return Some(MapTile::Floor);
        }

        let num_occupied_adjacent =
            self.count_visible_occupied_seats(row as i64, col as i64, context.single_step);

        Some(match *cur_seat {
            MapTile::Empty if num_occupied_adjacent == 0 => MapTile::Occupied,
            MapTile::Occupied if num_occupied_adjacent >= context.crowd_limit => MapTile::Empty,
            seat => seat,
        })
    }

    fn count_visible_occupied_seats(&self, row: i64, col: i64, single_step: bool) -> usize {
        const DIRECTIONS: &[(i64, i64); 8] = &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        DIRECTIONS
            .iter()
            .map(|(rd, cd)| {
                self.count_occupied_seats(row, col, *rd, *cd, single_step)
                    .unwrap_or(0)
            })
            .sum()
    }

    fn count_occupied_seats(
        &self,
        row: i64,
        col: i64,
        row_dir: i64,
        col_dir: i64,
        single_step: bool,
    ) -> Option<usize> {
        let mut cur_row = row + row_dir;
        let mut cur_col = col + col_dir;

        loop {
            if !self.is_valid_pos(cur_row, cur_col) || (cur_row == row && cur_col == col) {
                return None;
            }

            let cur_seat = self.elements.get(&(cur_row as usize, cur_col as usize))?;

            match *cur_seat {
                MapTile::Occupied => return Some(1),
                MapTile::Empty => return None,
                MapTile::Floor => {}
            };

            if single_step {
                return None;
            }

            cur_row += row_dir;
            cur_col += col_dir;
        }
    }

    pub fn count_occupied(&self) -> usize {
        self.elements
            .iter()
            .filter(|(_, seat)| **seat == MapTile::Occupied)
            .count()
    }

    pub fn new(width: usize, height: usize, elements: MapElements) -> Self {
        Self {
            width,
            height,
            elements,
        }
    }
}

fn run_iteration(prev_map: &MapData, context: &SimulationContext) -> Option<(MapElements, bool)> {
    let mut next_map_elements = MapElements::new();
    let mut has_changes = false;

    for (row, col) in iproduct!(0..prev_map.height, 0..prev_map.width) {
        let cur_state = prev_map.elements.get(&(row, col))?;
        let next_state = prev_map.get_next_state(row, col, context)?;
        next_map_elements.insert((row, col), next_state);

        if next_state != *cur_state {
            has_changes = true;
        }
    }

    Some((next_map_elements, has_changes))
}

pub fn run_simulation_until_stable(map: &MapData, context: &SimulationContext) -> Option<usize> {
    let mut prev_map = map.clone();
    loop {
        let (next_map_elems, has_changes) = run_iteration(&prev_map, context)?;

        if !has_changes {
            break;
        }

        prev_map.elements = next_map_elems;
    }

    Some(prev_map.count_occupied())
}

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
