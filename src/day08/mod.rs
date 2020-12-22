use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr, Debug, Clone)]
pub enum Instruction {
    #[display("acc {0}")]
    Acc(i64),

    #[display("jmp {0}")]
    Jmp(i64),

    #[display("nop {0}")]
    Nop(i64),
}

#[aoc_generator(day8)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines()
        .filter_map(|it| it.parse().map_err(|e| println!("Error: {}", e)).ok())
        .collect()
}

fn execute(insts: &[Instruction]) -> (i64, bool) {
    let mut visited = HashSet::new();
    let mut pc = 0i64;

    let mut acc = 0;

    while (pc as usize) < insts.len() {
        if visited.contains(&pc) {
            return (acc, false);
        }

        visited.insert(pc);

        match insts[pc as usize] {
            Instruction::Acc(a) => {
                pc += 1;
                acc += a;
            }
            Instruction::Jmp(j) => {
                pc += j;
            }
            Instruction::Nop(_) => {
                pc += 1;
            }
        }
    }

    (acc, true)
}

#[aoc(day8, part1)]
pub fn part1(v: &[Instruction]) -> Option<i64> {
    match execute(v) {
        (acc, false) => Some(acc),
        _ => None,
    }
}

fn replace_inst(v: &[Instruction], idx: usize, new_inst: Instruction) -> Vec<Instruction> {
    let mut insts = v.to_vec();
    if let Some(old_inst) = insts.get_mut(idx) {
        *old_inst = new_inst;
    }

    insts
}

fn try_replace_and_run(v: &[Instruction], inst: &Instruction, idx: usize) -> Option<(i64, bool)> {
    match inst {
        Instruction::Jmp(j) => {
            let insts = replace_inst(v, idx, Instruction::Nop(*j));
            Some(execute(&insts))
        }
        Instruction::Nop(n) => {
            let insts = replace_inst(v, idx, Instruction::Jmp(*n));
            Some(execute(&insts))
        }
        _ => None,
    }
}

#[aoc(day8, part2)]
pub fn part2(v: &[Instruction]) -> Option<i64> {
    (0..v.len()).find_map(|it| {
        let inst = &v[it];
        match try_replace_and_run(v, inst, it) {
            Some((acc, true)) => Some(acc),
            _ => None,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        let insts = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];

        assert_eq!(Some(5), part1(&insts));
        assert_eq!(Some(8), part2(&insts));
    }
}
