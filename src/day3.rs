use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generate(inp: &str) -> Vec<Vec<bool>> {
    inp.lines().fold(Vec::new(), |mut acc, line| {
        let inner = line.chars().fold(Vec::new(), |mut acc, chr| {
            match chr {
                '.' | '#' => acc.push(chr == '.'),
                _ => panic!("Invalid character!"),
            };

            acc
        });

        acc.push(inner);
        acc
    })
}

fn count_trees_on_slope(right: usize, down: usize, v: &[Vec<bool>]) -> usize {
    v.iter()
        .step_by(down)
        .enumerate()
        .fold(0, |acc, (idx, inner)| {
            let res = inner.iter().cycle().step_by(right).nth(idx).unwrap();
            acc + if *res { 0 } else { 1 }
        })
}

#[aoc(day3, part1)]
pub fn part1(v: &[Vec<bool>]) -> usize {
    count_trees_on_slope(3, 1, v)
}

#[aoc(day3, part2)]
pub fn part2(v: &[Vec<bool>]) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes.iter().fold(1, |acc, &(right, down)| {
        acc * count_trees_on_slope(right, down, v)
    })
}
