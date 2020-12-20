use std::collections::HashMap;

use itertools::iproduct;

use crate::iterator_ext::IteratorExt;

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

        DIRECTIONS.iter().sum_by(|(rd, cd)| {
            self.count_occupied_seats(row, col, *rd, *cd, single_step)
                .unwrap_or(0)
        })
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
                _ => {}
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
            .count_if(|(_, seat)| *seat == MapTile::Occupied)
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

        prev_map = MapData {
            width: map.width,
            height: map.height,
            elements: next_map_elems,
        };
    }

    Some(prev_map.count_occupied())
}
