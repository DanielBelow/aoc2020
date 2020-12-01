#[aoc_generator(day1)]
pub fn generate(inp: &str) -> Vec<u64> {
    let mut res = inp
        .lines()
        .map(|it| it.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    res.sort();
    res
}

fn find_indices(num: u64, v: &Vec<u64>, l: usize, h: usize) -> Option<(usize, usize)> {
    let mut low = l;
    let mut high = h;

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
    let (low, high) = find_indices(2020, v, 0, v.len() - 1).unwrap();
    v[low] * v[high]
}

#[aoc(day1, part2)]
pub fn part2(v: &Vec<u64>) -> u64 {
    let max_len = v.len() - 1;

    for i in 0..max_len {
        let curr = v[i];

        let start = i + 1;

        if let Some((low, high)) = find_indices(2020 - curr, v, start, max_len) {
            return curr * v[low] * v[high];
        }
    }

    panic!("No match found!")
}
