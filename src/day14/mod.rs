use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display as PDisplay, FromStr as PFromStr};

use crate::iterator_ext::IteratorExt;

#[derive(PDisplay, PFromStr)]
#[display("mask = {mask}")]
pub struct Mask {
    mask: String,
}

#[derive(PDisplay, PFromStr)]
#[display("mem[{index}] = {value}")]
pub struct MemAccess {
    index: i64,
    value: u64,
}

pub struct Initialization {
    mask: Mask,
    accesses: Vec<MemAccess>,
}

#[aoc_generator(day14)]
pub fn generate(inp: &str) -> Vec<Initialization> {
    let mut res = Vec::new();

    for l in inp.lines() {
        if let Ok(mask) = l.parse::<Mask>() {
            res.push(Initialization {
                mask,
                accesses: Vec::new(),
            });
        } else if let Ok(access) = l.parse::<MemAccess>() {
            if let Some(init) = res.last_mut() {
                init.accesses.push(MemAccess {
                    index: access.index,
                    value: access.value,
                });
            }
        }
    }

    res
}

type Ram = HashMap<i64, u64>;

fn apply_value_bitmask(inp: String, mask: &str) -> String {
    inp.chars()
        .zip(mask.chars())
        .map(|(l, r)| if r == 'X' { l } else { r })
        .collect()
}

fn apply_index_bitmask(inp: String, mask: &str) -> String {
    inp.chars()
        .zip(mask.chars())
        .map(|(l, r)| match r {
            '0' => l,
            _ => r,
        })
        .collect()
}

fn init_memory(init: &Initialization, memory: &mut Ram) {
    let mask = &init.mask.mask;

    init.accesses.iter().fold(memory, |acc, it| {
        let new_val = apply_value_bitmask(format!("{:036b}", it.value), mask);
        if let Ok(new_val) = u64::from_str_radix(&new_val, 2) {
            acc.insert(it.index, new_val);
        }

        acc
    });
}

fn to_padded_binary_string(inp: usize, padding: usize) -> String {
    format!("{:0>1$b}", inp, padding)
}

fn generate_floating_combinations(num_floating: usize) -> Vec<String> {
    (0..2_usize.pow(num_floating as u32))
        .map(|it| to_padded_binary_string(it, num_floating))
        .collect()
}

fn init_memory_v2(init: &Initialization, memory: &mut Ram) {
    let mask = &init.mask.mask;

    for acc in init.accesses.iter() {
        let new_index = apply_index_bitmask(format!("{:036b}", acc.index), mask);

        let num_floating = new_index.chars().count_if(|it| it == 'X');
        let combinations = generate_floating_combinations(num_floating);

        for comb in combinations {
            let mut chrs = comb.chars();
            let cur_val = new_index.to_string();

            let new_str = cur_val
                .chars()
                .filter_map(|it| match it {
                    'X' => chrs.next(),
                    _ => Some(it),
                })
                .collect::<String>();

            if let Ok(idx) = i64::from_str_radix(&new_str, 2) {
                memory.insert(idx, acc.value as u64);
            }
        }
    }
}

fn run_initialization(
    init_sequence: &[Initialization],
    memory: &mut Ram,
    init_func: impl Fn(&Initialization, &mut Ram),
) -> u64 {
    init_sequence.iter().for_each(|it| init_func(it, memory));
    memory.iter().sum_by(|(_, val)| *val)
}

#[aoc(day14, part1)]
pub fn part1(v: &[Initialization]) -> u64 {
    run_initialization(v, &mut Ram::new(), init_memory)
}

#[aoc(day14, part2)]
pub fn part2(v: &[Initialization]) -> u64 {
    run_initialization(v, &mut Ram::new(), init_memory_v2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inp = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let data = generate(inp);
        assert_eq!(165, part1(&data));
    }

    #[test]
    fn test_part2() {
        let inp = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        let data = generate(inp);
        assert_eq!(208, part2(&data));
    }
}
