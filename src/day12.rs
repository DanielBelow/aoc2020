use aoc_runner_derive::{aoc, aoc_generator};

use super::ship::{NavigationAction, Ship, Simple, WithWaypoint};

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<NavigationAction> {
    inp.lines()
        .filter_map(|it| it.parse().map_err(|e| println!("Error: {e}")).ok())
        .collect()
}

fn run_actions<T>(actions: &[NavigationAction], ship: &mut T) -> i64
where
    T: Ship,
{
    actions.iter().for_each(|it| ship.perform_action(it));
    ship.get_distance()
}

#[aoc(day12, part1)]
pub fn part1(v: &[NavigationAction]) -> i64 {
    run_actions(v, &mut Simple::new())
}

#[aoc(day12, part2)]
pub fn part2(v: &[NavigationAction]) -> i64 {
    run_actions(v, &mut WithWaypoint::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let inp = vec![
            NavigationAction::Forward(10),
            NavigationAction::North(3),
            NavigationAction::Forward(7),
            NavigationAction::Right(90),
            NavigationAction::Forward(11),
        ];

        assert_eq!(25, part1(&inp));
    }

    #[test]
    fn test_sample_part2() {
        let inp = vec![
            NavigationAction::Forward(10),
            NavigationAction::North(3),
            NavigationAction::Forward(7),
            NavigationAction::Right(90),
            NavigationAction::Forward(11),
        ];

        assert_eq!(286, part2(&inp));
    }
}
