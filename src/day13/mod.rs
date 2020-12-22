use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct BusData {
    id: i64,
    order: i64,
}

pub struct BusSchedule {
    earliest_depart: i64,
    buses: Vec<BusData>,
}

#[aoc_generator(day13)]
pub fn generate(inp: &str) -> Option<BusSchedule> {
    let lines = &mut inp.lines();

    let first_line = lines.next()?;
    let earliest_depart = first_line.parse::<i64>().ok()?;

    let second_line = lines.next()?;
    let buses = second_line
        .split(',')
        .enumerate()
        .filter_map(|(order, it)| {
            let order = order as i64;
            it.parse::<i64>().ok().map(|id| BusData { id, order })
        })
        .collect();

    Some(BusSchedule {
        earliest_depart,
        buses,
    })
}

fn find_lowest_gt(target: i64, multiple: i64) -> i64 {
    multiple * (target as f64 / multiple as f64).ceil() as i64
}

fn egcd(lhs: i64, rhs: i64) -> (i64, i64, i64) {
    if lhs == 0 {
        (rhs, 0, 1)
    } else {
        let (g, x, y) = egcd(rhs % lhs, lhs);
        (g, y - (rhs / lhs) * x, x)
    }
}

fn mod_inv(lhs: i64, rhs: i64) -> Option<i64> {
    let (g, x, _) = egcd(lhs, rhs);
    if g == 1 {
        Some((x % rhs + rhs) % rhs)
    } else {
        None
    }
}

fn chinese_remainder_theorem(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[aoc(day13, part1)]
pub fn part1(v: &BusSchedule) -> Option<i64> {
    let depart = v.earliest_depart;

    v.buses
        .iter()
        .map(|it| {
            let time = find_lowest_gt(depart, it.id);
            (it.id, time)
        })
        .sorted_by_key(|(_, time)| *time)
        .next()
        .map(|(id, time)| id * (time - depart))
}

#[aoc(day13, part2)]
pub fn part2(v: &BusSchedule) -> Option<i64> {
    let ids = v.buses.iter().map(|it| it.id).collect_vec();
    let order = v.buses.iter().map(|it| it.id - it.order).collect_vec();

    chinese_remainder_theorem(&order, &ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let inp = "939
7,13,x,x,59,x,31,19";
        let data = generate(inp).unwrap();
        assert_eq!(Some(295), part1(&data));
    }

    #[test]
    fn test_sample1_part2() {
        let inp = "0
17,x,13,19";

        let data = generate(inp).unwrap();
        assert_eq!(Some(3417), part2(&data));
    }

    #[test]
    fn test_sample2_part2() {
        let inp = "123
67,7,59,61";

        let data = generate(inp).unwrap();
        assert_eq!(Some(754018), part2(&data));
    }

    #[test]
    fn test_sample2_part3() {
        let inp = "123
67,x,7,59,61";

        let data = generate(inp).unwrap();
        assert_eq!(Some(779210), part2(&data));
    }

    #[test]
    fn test_sample2_part4() {
        let inp = "123
67,7,x,59,61";

        let data = generate(inp).unwrap();
        assert_eq!(Some(1261476), part2(&data));
    }

    #[test]
    fn test_sample2_part5() {
        let inp = "123
1789,37,47,1889";

        let data = generate(inp).unwrap();
        assert_eq!(Some(1202161486), part2(&data));
    }
}
