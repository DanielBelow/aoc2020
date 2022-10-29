use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
pub fn generate(inp: &str) -> Vec<usize> {
    inp.lines()
        .filter_map(|it| it.parse().map_err(|e| println!("Error: {e}")).ok())
        .collect()
}

fn step(value: usize, subject_num: usize) -> usize {
    (value * subject_num) % 20_201_227
}

fn get_loop_size(target: usize) -> usize {
    const SUBJECT_NUM: usize = 7;

    let mut loop_size = 0;
    let mut val = 1;

    while val != target {
        val = step(val, SUBJECT_NUM);
        loop_size += 1;
    }

    loop_size
}

fn get_encryption_key(key: usize, size: usize) -> usize {
    let mut enc_key = 1;
    for _ in 0..size {
        enc_key = step(enc_key, key);
    }

    enc_key
}

#[aoc(day25, part1)]
pub fn part1(keys: &[usize]) -> usize {
    let card = keys[0];
    let door = keys[1];

    let card_loop_size = get_loop_size(card);
    let door_loop_size = get_loop_size(door);

    let card_key = get_encryption_key(door, card_loop_size);
    let door_key = get_encryption_key(card, door_loop_size);

    assert_eq!(card_key, door_key);

    card_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let keys = vec![5_764_801, 17_807_724];
        assert_eq!(14_897_079, part1(&keys));
    }
}
