use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generate(inp: &str) -> Vec<u64> {
    let mut res = inp
        .lines()
        .map(|it| it.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    res.sort_unstable();
    res
}

fn find_indices(num: u64, v: &[u64]) -> Option<(usize, usize)> {
    if v.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = v.len() - 1;

    loop {
        let sum = v[low] + v[high];
        if sum == num {
            break;
        }

        if sum > num {
            high -= 1;
        } else {
            low += 1;
        }

        if low >= high {
            return None;
        }
    }

    Some((low, high))
}

#[aoc(day1, part1)]
pub fn part1(v: &Vec<u64>) -> u64 {
    let (low, high) = find_indices(2020, v.as_slice()).unwrap();
    v[low] * v[high]
}

#[aoc(day1, part2)]
pub fn part2(v: &Vec<u64>) -> u64 {
    let max_len = v.len() - 1;

    for i in 0..max_len {
        let curr = v[i];
        let rem = &v[i + 1..];

        if let Some((low, high)) = find_indices(2020 - curr, rem) {
            return curr * rem[low] * rem[high];
        }
    }

    panic!("No match found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_indices() {
        let inp = vec![1, 5, 2, 4, 3];
        let res = find_indices(7, inp.as_slice());
        assert!(res.is_some());
        assert_eq!(res.unwrap(), (1, 2));

        assert!(find_indices(2020, vec![].as_slice()).is_none());
        assert!(find_indices(2020, vec![1, 2, 3].as_slice()).is_none());
    }
}
