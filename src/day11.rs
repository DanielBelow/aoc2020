use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::iterator_ext::IteratorExt;

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

    MapData {
        width,
        height,
        elements,
    }
}

fn get_next_state(
    row: usize,
    col: usize,
    single_step: bool,
    occupation_limit: usize,
    map: &MapData,
) -> Option<MapTile> {
    let cur_seat = map.elements.get(&(row, col))?;
    if *cur_seat == MapTile::Floor {
        return Some(MapTile::Floor);
    }

    let num_occupied_adjacent =
        count_visible_occupied_seats(row as i64, col as i64, single_step, map);

    Some(match *cur_seat {
        MapTile::Empty if num_occupied_adjacent == 0 => MapTile::Occupied,
        MapTile::Occupied if num_occupied_adjacent >= occupation_limit => MapTile::Empty,
        seat => seat,
    })
}

fn fill_new_seats(
    prev_map: &MapData,
    single_step: bool,
    occupation_limit: usize,
) -> Option<(MapElements, bool)> {
    let mut next_map_elements = MapElements::new();
    let mut has_changes = false;

    for row in 0..prev_map.height {
        for col in 0..prev_map.width {
            let cur_state = prev_map.elements.get(&(row, col))?;
            let next_state = get_next_state(row, col, single_step, occupation_limit, &prev_map)?;
            next_map_elements.insert((row, col), next_state);

            if next_state != *cur_state {
                has_changes = true;
            }
        }
    }

    Some((next_map_elements, has_changes))
}

fn run_simulation_until_stable(
    map: &MapData,
    single_step: bool,
    occupation_limit: usize,
) -> Option<usize> {
    let mut prev_map = map.clone();
    loop {
        let (next_map_elems, has_changes) =
            fill_new_seats(&prev_map, single_step, occupation_limit)?;

        if !has_changes {
            break;
        }

        prev_map = MapData {
            width: map.width,
            height: map.height,
            elements: next_map_elems,
        };
    }

    Some(
        prev_map
            .elements
            .iter()
            .count_if(|(_, seat)| *seat == MapTile::Occupied),
    )
}

#[aoc(day11, part1)]
pub fn part1(map: &MapData) -> Option<usize> {
    run_simulation_until_stable(map, true, 4)
}

fn is_valid_pos(row: i64, col: i64, height: usize, width: usize) -> bool {
    row >= 0 && col >= 0 && row < height as i64 && col < width as i64
}

fn count_occupied_seats(
    row: i64,
    col: i64,
    row_dir: i64,
    col_dir: i64,
    single_step: bool,
    map: &MapData,
) -> Option<usize> {
    let mut cur_row = row + row_dir;
    let mut cur_col = col + col_dir;

    loop {
        if !is_valid_pos(cur_row, cur_col, map.height, map.width)
            || (cur_row == row && cur_col == col)
        {
            return None;
        }

        let cur_seat = map.elements.get(&(cur_row as usize, cur_col as usize))?;

        match *cur_seat {
            MapTile::Occupied => return Some(1),
            MapTile::Empty => return None,
            _ => {}
        };

        if single_step {
            return None;
        }

        cur_row += row_dir;
        cur_col += col_dir;
    }
}

fn count_visible_occupied_seats(row: i64, col: i64, single_step: bool, map: &MapData) -> usize {
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

    DIRECTIONS.iter().fold(0, |acc, (rd, cd)| {
        acc + count_occupied_seats(row, col, *rd, *cd, single_step, map).unwrap_or(0)
    })
}

#[aoc(day11, part2)]
pub fn part2(map: &MapData) -> Option<usize> {
    run_simulation_until_stable(map, false, 5)
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
