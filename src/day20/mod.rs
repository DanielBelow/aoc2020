use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};

const GRID_SIZE: usize = 10;

#[derive(Default, Clone, Debug)]
pub struct MapTile {
    id: usize,
    map: Vec<Vec<char>>,
}

const MONSTER_OFFSETS: &[(i64, i64); 15] = &[
    (0, 0),
    (1, 1),
    (1, 4),
    (0, 5),
    (0, 6),
    (1, 7),
    (1, 10),
    (0, 11),
    (0, 12),
    (1, 13),
    (1, 16),
    (0, 17),
    (0, 18),
    (-1, 18),
    (0, 19),
];

impl MapTile {
    fn is_sea_monster_at(&self, x: usize, y: usize) -> bool {
        MONSTER_OFFSETS
            .iter()
            .all(|(dy, dx)| self.map[(y as i64 + dy) as usize][(x as i64 + dx) as usize] == '#')
    }

    fn replace_sea_monsters(&mut self) {
        for (y, x) in iproduct!(1..self.map.len() - 1, 0..self.map.len() - 19) {
            if self.is_sea_monster_at(x, y) {
                for (dy, dx) in MONSTER_OFFSETS.iter() {
                    self.map[(y as i64 + dy) as usize][(x as i64 + dx) as usize] = 'O';
                }
            }
        }
    }

    fn contains_sea_monster(&self) -> bool {
        for (y, row) in self.map.iter().enumerate().skip(1).take(self.map.len() - 2) {
            for x in 0..(row.len() - 19) {
                if self.is_sea_monster_at(x, y) {
                    return true;
                }
            }
        }

        false
    }

    fn is_match(lhs: &[char], rhs: &[char]) -> bool {
        lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(l, r)| *l == *r)
    }

    fn matches_bottom(&self, other: &Self) -> bool {
        self.map.last().map_or(false, |self_last| {
            other
                .map
                .first()
                .map_or(false, |other_first| Self::is_match(self_last, other_first))
        })
    }

    fn matches_right(&self, other: &Self) -> bool {
        let self_right = self
            .map
            .iter()
            .filter_map(|it| it.last().copied())
            .collect_vec();

        let other_left = other
            .map
            .iter()
            .filter_map(|it| it.first().copied())
            .collect_vec();

        Self::is_match(&self_right, &other_left)
    }
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Vec<MapTile> {
    inp.split("\n\n").fold(Vec::new(), |mut acc, it| {
        let lines = &mut it.lines();
        if let Some(title_line) = lines.next() {
            if let Some(id) = title_line
                .strip_prefix("Tile ")
                .and_then(|it| it.strip_suffix(':'))
                .and_then(|it| it.parse::<usize>().ok())
            {
                let mut map = vec![vec!['?'; GRID_SIZE]; GRID_SIZE];
                for (y, l) in lines.enumerate() {
                    for (x, chr) in l.char_indices() {
                        match chr {
                            '.' => map[y][x] = '.',
                            '#' => map[y][x] = '#',
                            _ => panic!("Invalid character for map"),
                        }
                    }
                }

                let tile = MapTile { id, map };
                acc.push(tile);
            }
        }

        acc
    })
}

fn flip_x(tile: &MapTile) -> MapTile {
    let mut flipped = Vec::new();

    for row in &tile.map {
        let flipped_row = row.iter().rev().copied().collect_vec();
        flipped.push(flipped_row);
    }

    MapTile {
        id: tile.id,
        map: flipped,
    }
}

fn flips_and_rotations(tile: &MapTile) -> Vec<MapTile> {
    let dim = tile.map.len();

    let mut res = Vec::new();

    let mut add_rotations = |tile: &MapTile| {
        res.push(tile.clone());
        {
            let mut acc = tile.map.clone();
            for (x, y) in iproduct!(0..dim, 1..=dim) {
                acc[y - 1][x] = tile.map[dim - y][x];
            }

            let mt = MapTile {
                id: tile.id,
                map: acc,
            };
            res.push(mt);
        }
        {
            let mut acc = tile.map.clone();
            for (x, y) in iproduct!(1..=dim, 1..=dim) {
                acc[y - 1][x - 1] = tile.map[dim - x][dim - y];
            }

            let mt = MapTile {
                id: tile.id,
                map: acc,
            };
            res.push(mt);
        }

        {
            let mut acc = tile.map.clone();
            for (x, y) in iproduct!(1..=dim, 0..dim) {
                acc[x - 1][y] = tile.map[y][dim - x];
            }

            let mt = MapTile {
                id: tile.id,
                map: acc,
            };
            res.push(mt);
        }
    };

    add_rotations(tile);

    let flipped = flip_x(tile);
    add_rotations(&flipped);

    res
}

fn fill_grid(
    x: usize,
    y: usize,
    data: &[MapTile],
    visited: &mut HashSet<usize>,
    grid: &mut Vec<Vec<MapTile>>,
) -> bool {
    if visited.len() == data.len() {
        return true;
    }

    let unvisited = data
        .iter()
        .filter(|it| !visited.contains(&it.id))
        .collect_vec();

    for tile in unvisited {
        let cur_id = tile.id;
        visited.insert(cur_id);

        let flips_rots = flips_and_rotations(tile);
        for fr in &flips_rots {
            let x_match = x == 0 || grid[y][x - 1].matches_right(fr);
            let y_match = y == 0 || grid[y - 1][x].matches_bottom(fr);

            if x_match && y_match {
                grid[y][x] = fr.clone();

                let next_x = (x + 1) % grid.len();
                let next_y = if next_x == 0 { y + 1 } else { y };
                if fill_grid(next_x, next_y, data, visited, grid) {
                    return true;
                }
            }
        }

        visited.remove(&cur_id);
    }

    false
}

fn reconstruct_image(dim: usize, data: &[MapTile]) -> Vec<Vec<MapTile>> {
    let mut grid = vec![vec![MapTile::default(); dim]; dim];

    let mut visited = HashSet::new();

    for tile in data.iter() {
        let cur_id = tile.id;
        visited.insert(cur_id);

        let flips_rots = flips_and_rotations(tile);
        for fr in &flips_rots {
            grid[0][0] = fr.clone();
            if fill_grid(1, 0, data, &mut visited, &mut grid) {
                return grid;
            }
        }

        visited.clear();
    }

    panic!("Wrong starting thingy")
}

fn merge_without_border(image: &[Vec<MapTile>]) -> Vec<Vec<char>> {
    image.iter().fold(Vec::new(), |mut acc, cur_row| {
        for idx in 1..GRID_SIZE - 1 {
            let tmp = cur_row.iter().fold(Vec::new(), |inner_acc, cur_tile| {
                let first_row = &cur_tile.map[idx];
                first_row.iter().dropping(1).dropping_back(1).fold(
                    inner_acc,
                    |mut inner_acc, it| {
                        inner_acc.push(*it);
                        inner_acc
                    },
                )
            });

            acc.push(tmp);
        }

        acc
    })
}

#[aoc(day20, part1)]
#[allow(clippy::cast_precision_loss)]
pub fn part1(data: &[MapTile]) -> usize {
    let grid_dim = (data.len() as f64).sqrt() as usize;
    let image = reconstruct_image(grid_dim, data);

    image[0][0].id
        * image[0][grid_dim - 1].id
        * image[grid_dim - 1][0].id
        * image[grid_dim - 1][grid_dim - 1].id
}

#[aoc(day20, part2)]
#[allow(clippy::cast_precision_loss)]
pub fn part2(data: &[MapTile]) -> Option<usize> {
    let grid_dim = (data.len() as f64).sqrt() as usize;
    let image = reconstruct_image(grid_dim, data);

    let merged = merge_without_border(&image);

    let combined_tile = MapTile { id: 0, map: merged };

    let mut frs = flips_and_rotations(&combined_tile);
    let with_monsters = frs.iter_mut().find(|it| it.contains_sea_monster())?;
    with_monsters.replace_sea_monsters();

    let water_roughness = with_monsters
        .map
        .iter()
        .map(|it| it.iter().filter(|c| **c == '#').count())
        .sum();
    Some(water_roughness)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INP: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_part1() {
        let data = generate(INP);
        assert_eq!(20_899_048_083_289, part1(&data));
    }

    #[test]
    fn test_part2() {
        let data = generate(INP);
        assert_eq!(Some(273), part2(&data));
    }
}
