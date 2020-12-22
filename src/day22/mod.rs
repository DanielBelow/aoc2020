use crate::iterator_ext::IteratorExt;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Game {
    player: VecDeque<usize>,
    ferris: VecDeque<usize>,
}

impl Game {
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
            .sum_by(|(idx, it)| (idx + 1) * *it)
    }
}

fn parse_card_deck(lines: &str) -> VecDeque<usize> {
    lines
        .lines()
        .skip(1)
        .filter_map(|it| it.parse::<usize>().ok())
        .collect::<VecDeque<_>>()
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
        let p_first = game.player.pop_front().unwrap();
        let f_first = game.ferris.pop_front().unwrap();

        if p_first > f_first {
            game.player.push_back(p_first);
            game.player.push_back(f_first);
        } else {
            game.ferris.push_back(f_first);
            game.ferris.push_back(p_first);
        }
    }
}

fn play_recursive_combat(game: &mut Game) {
    let mut cache = HashSet::new();

    while !game.player.is_empty() && !game.ferris.is_empty() {
        if cache.contains(game) {
            game.ferris.clear();
            return;
        }

        cache.insert(game.clone());

        let p_first = game.player.pop_front().unwrap();
        let f_first = game.ferris.pop_front().unwrap();

        let player_won = if game.can_recurse(p_first, f_first) {
            let mut sub_game = Game {
                player: game.player.iter().take(p_first).copied().collect(),
                ferris: game.ferris.iter().take(f_first).copied().collect(),
            };

            play_recursive_combat(&mut sub_game);
            sub_game.player_won()
        } else {
            p_first > f_first
        };

        if player_won {
            game.player.push_back(p_first);
            game.player.push_back(f_first);
        } else {
            game.ferris.push_back(f_first);
            game.ferris.push_back(p_first);
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
        let game = generate(INP);
        assert_eq!(306, part1(&game.unwrap()));
    }

    #[test]
    fn test_part2() {
        let game = generate(INP);
        assert_eq!(291, part2(&game.unwrap()));
    }
}
