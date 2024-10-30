use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Vec<Vec<(i64, i64)>> {
    inp.lines()
        .map(|it| {
            let mut dirs = Vec::new();

            let mut iter = it.chars().peekable();

            while let Some(next) = iter.next() {
                let dir = match next {
                    'e' => (-1, 0),
                    'w' => (1, 0),
                    's' if iter.peek() == Some(&'e') => (-1, 1),
                    's' if iter.peek() == Some(&'w') => (0, 1),
                    'n' if iter.peek() == Some(&'w') => (1, -1),
                    'n' if iter.peek() == Some(&'e') => (0, -1),
                    _ => panic!("Invalid char"),
                };

                if dir != (-1, 0) && dir != (1, 0) {
                    iter.next();
                }

                dirs.push(dir);
            }

            dirs
        })
        .collect()
}

fn setup_tiles(insts: &[Vec<(i64, i64)>]) -> Vec<Vec<bool>> {
    const LEN: usize = 500;

    insts
        .iter()
        .fold(vec![vec![true; LEN]; LEN], |mut acc, it| {
            let pos = it
                .iter()
                .fold((LEN / 2, LEN / 2), |(acc_l, acc_r), (it_l, it_r)| {
                    #[allow(clippy::cast_possible_wrap)]
                    let new_x = (acc_l as i64 + it_l) as usize;

                    #[allow(clippy::cast_possible_wrap)]
                    let new_y = (acc_r as i64 + it_r) as usize;
                    (new_x, new_y)
                });

            acc[pos.0][pos.1] = !acc[pos.0][pos.1];
            acc
        })
}

#[aoc(day24, part1)]
pub fn part1(insts: &[Vec<(i64, i64)>]) -> usize {
    let tiles = setup_tiles(insts);

    tiles
        .iter()
        .fold(0, |acc, it| acc + it.iter().filter(|v| !**v).count())
}

fn get_neighbor_indices() -> Vec<(i64, i64)> {
    vec![(0, -1), (1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0)]
}

#[aoc(day24, part2)]
pub fn part2(insts: &[Vec<(i64, i64)>]) -> usize {
    let neighbors = get_neighbor_indices();

    let mut cur_tiles = setup_tiles(insts);

    for _ in 0..100 {
        let mut next_tiles = cur_tiles.clone();

        for (x, y) in iproduct!(1..cur_tiles.len() - 1, 1..cur_tiles.len() - 1) {
            let cur = cur_tiles[x][y];

            let black_neighbors = neighbors
                .iter()
                .filter(|(l, r)| {
                    #[allow(clippy::cast_possible_wrap)]
                    let dx = (x as i64 + l) as usize;

                    #[allow(clippy::cast_possible_wrap)]
                    let dy = (y as i64 + r) as usize;
                    !cur_tiles[dx][dy]
                })
                .count();

            let next_state = if cur {
                black_neighbors != 2
            } else {
                black_neighbors == 0 || black_neighbors > 2
            };

            next_tiles[x][y] = next_state;
        }

        cur_tiles = next_tiles;
    }

    cur_tiles
        .iter()
        .fold(0, |acc, it| acc + it.iter().filter(|v| !**v).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INP: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part1() {
        let insts = generate(INP);
        assert_eq!(10, part1(&insts));
    }

    #[test]
    fn test_part2() {
        let insts = generate(INP);
        assert_eq!(2208, part2(&insts));
    }
}
