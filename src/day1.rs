use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generate(inp: &str) -> Vec<u64> {
    let mut res = inp
        .lines()
        .map(|it| it.parse().unwrap())
        .collect::<Vec<_>>();
    res.sort_unstable();
    res
}

fn find_product(num: u64, v: &[u64]) -> Option<u64> {
    if v.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = v.len() - 1;

    while low < high {
        let sum = v[low] + v[high];
        if sum == num {
            return Some(v[low] * v[high]);
        }

        if sum > num {
            high -= 1;
        } else {
            low += 1;
        }
    }

    None
}

#[aoc(day1, part1)]
pub fn part1(v: &[u64]) -> u64 {
    find_product(2020, v).unwrap_or(0)
}

#[aoc(day1, part2)]
pub fn part2(v: &[u64]) -> u64 {
    (0..v.len() - 1)
        .map(|it| {
            let curr = v[it];
            let rem = &v[it + 1..];

            curr * find_product(2020 - curr, rem).unwrap_or(0)
        })
        .find(|it| *it != 0)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_indices() {
        let inp = vec![1, 5, 2, 4, 3];
        let res = find_product(7, inp.as_slice());
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 5 * 2);

        assert!(find_product(2020, vec![].as_slice()).is_none());
        assert!(find_product(2020, vec![1, 2, 3].as_slice()).is_none());
    }
}
