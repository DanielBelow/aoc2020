use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct CrabGame {
    indices: Vec<usize>,
}

impl CrabGame {
    fn new(nums: Vec<usize>, len: usize) -> Self {
        let mut v = (1..len + 2).collect_vec();
        v[len] = nums[0];
        let mut indices = nums.windows(2).fold(v, |mut acc, it| {
            acc[it[0]] = it[1];
            acc
        });

        indices[nums[nums.len() - 1]] = if nums.len() == len {
            nums[0]
        } else {
            nums.len() + 1
        };

        Self { indices }
    }

    fn play_rounds(&mut self, rounds: usize, first: usize) {
        let mut cur = first;
        for _ in 0..rounds {
            cur = self.play(cur);
        }
    }

    fn resolve_indices(&self, from: usize) -> Vec<usize> {
        let mut res = Vec::new();

        let mut cur = from;
        loop {
            cur = self.indices[cur];
            if cur == from {
                break;
            }

            res.push(cur);
        }

        res
    }

    fn get_number_after(&self, after: usize) -> usize {
        self.indices[after]
    }

    fn get_next_index(&self, cur: usize) -> usize {
        let len = self.indices.len() - 1;
        if cur > 1 {
            cur - 1
        } else {
            len
        }
    }

    fn set_next(&mut self, index: usize, next: usize) {
        self.indices[index] = next;
    }

    fn play(&mut self, cur: usize) -> usize {
        let mut next_three = [0usize; 3];
        next_three[0] = self.get_number_after(cur);
        next_three[1] = self.get_number_after(next_three[0]);
        next_three[2] = self.get_number_after(next_three[1]);

        self.set_next(cur, self.indices[next_three[2]]);

        let mut destination = self.get_next_index(cur);
        while next_three.contains(&destination) {
            destination = self.get_next_index(destination);
        }

        self.set_next(next_three[2], self.indices[destination]);
        self.set_next(destination, next_three[0]);

        self.indices[cur]
    }
}

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<usize> {
    inp.chars()
        .filter_map(|it| it.to_digit(10).map(|it| it as usize))
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(nums: &[usize]) -> Option<usize> {
    let mut cg = CrabGame::new(nums.to_owned(), nums.len());
    cg.play_rounds(100, nums[0]);

    let res = cg.resolve_indices(1);
    res.iter().join("").parse().ok()
}

#[aoc(day23, part2)]
pub fn part2(nums: &[usize]) -> Option<usize> {
    let mut cg = CrabGame::new(nums.to_owned(), 1_000_000);
    cg.play_rounds(10_000_000, nums[0]);

    let res = cg.resolve_indices(1);
    Some(res[0] * res[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    const INP: &str = "389125467";

    #[test]
    fn test_part1() {
        let nums = generate(INP);
        assert_eq!(Some(67384529), part1(&nums));
    }

    #[test]
    fn test_part2() {
        let nums = generate(INP);
        assert_eq!(Some(149245887792), part2(&nums));
    }
}
