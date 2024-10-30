use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Game {
    player: VecDeque<usize>,
    ferris: VecDeque<usize>,
}

impl Game {
    fn with_cards(
        old_player: &VecDeque<usize>,
        p_num: usize,
        old_ferris: &VecDeque<usize>,
        f_num: usize,
    ) -> Self {
        let mut player = old_player.clone();
        player.truncate(p_num);

        let mut ferris = old_ferris.clone();
        ferris.truncate(f_num);

        Self { player, ferris }
    }

    fn calc_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn player_won(&self) -> bool {
        self.ferris.is_empty()
    }

    fn can_recurse(&self, p_first: usize, f_first: usize) -> bool {
        self.player.len() >= p_first && self.ferris.len() >= f_first
    }

    fn calculate_winning_score(&self) -> usize {
        let cards = if self.player_won() {
            &self.player
        } else {
            &self.ferris
        };

        cards
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, it)| (idx + 1) * it)
            .sum()
    }
}

fn parse_card_deck(lines: &str) -> VecDeque<usize> {
    lines
        .lines()
        .skip(1)
        .filter_map(|it| it.parse().map_err(|e| println!("Error: {e}")).ok())
        .collect()
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Option<Game> {
    let mut spl = inp.split("\n\n");

    let player = parse_card_deck(spl.next()?);
    let ferris = parse_card_deck(spl.next()?);
    Some(Game { player, ferris })
}

fn play_combat(game: &mut Game) {
    while !game.player.is_empty() && !game.ferris.is_empty() {
        let p_first = game.player.pop_front().expect("Shouldn't be empty!");
        let f_first = game.ferris.pop_front().expect("Shouldn't be empty!");

        if p_first > f_first {
            game.player.extend([p_first, f_first].iter());
        } else {
            game.ferris.extend([f_first, p_first].iter());
        }
    }
}

fn play_recursive_combat(game: &mut Game) {
    let mut cache = HashSet::new();

    while !game.player.is_empty() && !game.ferris.is_empty() {
        let hash = game.calc_hash();
        if !cache.insert(hash) {
            game.ferris.clear();
            return;
        }

        let p_first = game.player.pop_front().expect("Shouldn't be empty!");
        let f_first = game.ferris.pop_front().expect("Shouldn't be empty!");

        let player_won = if game.can_recurse(p_first, f_first) {
            let mut sub_game = Game::with_cards(&game.player, p_first, &game.ferris, f_first);
            play_recursive_combat(&mut sub_game);
            sub_game.player_won()
        } else {
            p_first > f_first
        };

        if player_won {
            game.player.extend([p_first, f_first].iter());
        } else {
            game.ferris.extend([f_first, p_first].iter());
        }
    }
}

#[aoc(day22, part1)]
pub fn part1(cards: &Game) -> usize {
    let mut game = cards.clone();
    play_combat(&mut game);
    game.calculate_winning_score()
}

#[aoc(day22, part2)]
pub fn part2(cards: &Game) -> usize {
    let mut game = cards.clone();
    play_recursive_combat(&mut game);
    game.calculate_winning_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INP: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part1() {
        let Some(game) = generate(INP) else {
            panic!("Could not parse test input")
        };
        assert_eq!(306, part1(&game));
    }

    #[test]
    fn test_part2() {
        let Some(game) = generate(INP) else {
            panic!("Could not parse test input")
        };
        assert_eq!(291, part2(&game));
    }
}
