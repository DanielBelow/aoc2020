use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr, PartialEq, Clone)]
#[display("{from}-{to}")]
pub struct Rule {
    from: usize,
    to: usize,
}

#[derive(PDisplay, PFromStr, PartialEq, Clone)]
#[display("{field}: {rule1} or {rule2}")]
pub struct FieldRules {
    field: String,
    rule1: Rule,
    rule2: Rule,
}

impl FieldRules {
    fn matches(&self, num: usize) -> bool {
        (num >= self.rule1.from && num <= self.rule1.to)
            || (num >= self.rule2.from && num <= self.rule2.to)
    }
}

pub struct TicketData {
    rules: Vec<FieldRules>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl TicketData {
    fn matches_any_rule(&self, num: usize) -> bool {
        self.rules.iter().any(|rule| rule.matches(num))
    }

    fn get_valid_tickets(&self) -> Vec<Vec<usize>> {
        self.nearby_tickets
            .iter()
            .filter(|it| it.iter().all(|field| self.matches_any_rule(*field)))
            .cloned()
            .collect()
    }

    fn multiply_departure_fields(&self, order: &[FieldRules]) -> usize {
        self.my_ticket
            .iter()
            .zip(order)
            .filter_map(|(field, rule)| {
                if rule.field.starts_with("departure") {
                    Some(*field)
                } else {
                    None
                }
            })
            .product()
    }
}

fn parse_field_rules(inp: &str) -> Vec<FieldRules> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn parse_ticket_fields(inp: &str) -> Vec<usize> {
    inp.split(',')
        .filter_map(|l| l.parse::<usize>().ok())
        .collect()
}

fn parse_my_ticket(inp: &str) -> Option<Vec<usize>> {
    inp.lines().nth(1).map(parse_ticket_fields)
}

fn parse_nearby_tickets(inp: &str) -> Vec<Vec<usize>> {
    inp.lines().skip(1).map(parse_ticket_fields).collect()
}

#[aoc_generator(day16)]
pub fn generate(inp: &str) -> Option<TicketData> {
    let mut spl = inp.split("\n\n");

    let rules = parse_field_rules(spl.next()?);
    let my_ticket = parse_my_ticket(spl.next()?)?;
    let nearby_tickets = parse_nearby_tickets(spl.next()?);

    Some(TicketData {
        rules,
        my_ticket,
        nearby_tickets,
    })
}

#[aoc(day16, part1)]
pub fn part1(v: &TicketData) -> usize {
    v.nearby_tickets.iter().fold(0, |acc, it| {
        let invalid_field = it.iter().find(|field| !v.matches_any_rule(**field));
        match invalid_field {
            None => acc,
            Some(f) => acc + f,
        }
    })
}

fn try_remove_single_rule(rule: &FieldRules, remove_from: &mut Vec<Vec<&FieldRules>>) -> bool {
    remove_from
        .iter_mut()
        .filter(|it| it.len() > 1)
        .any(|all_rules| {
            if let Some(idx) = all_rules.iter().position(|it| *it == rule) {
                all_rules.remove(idx);
                true
            } else {
                false
            }
        })
}

fn find_rule_order(mut rules: Vec<Vec<&FieldRules>>) -> Vec<FieldRules> {
    loop {
        let single_rules = rules
            .iter()
            .filter_map(|it| it.iter().exactly_one().ok())
            .copied()
            .collect_vec();

        let any_changed = single_rules
            .iter()
            .any(|rule| try_remove_single_rule(*rule, &mut rules));

        if !any_changed {
            break;
        }
    }

    rules.iter().map(|it| it[0].clone()).collect()
}

#[aoc(day16, part2)]
pub fn part2(v: &TicketData) -> usize {
    let valid_tickets = v.get_valid_tickets();

    let possible_rules_per_field = (0..v.my_ticket.len()).fold(Vec::new(), |mut acc, idx| {
        let possible_rules = v
            .rules
            .iter()
            .filter(|it| {
                let nth_fields = valid_tickets
                    .iter()
                    .filter_map(|fields| fields.get(idx))
                    .collect_vec();

                nth_fields.iter().all(|field| it.matches(**field))
            })
            .collect_vec();

        acc.push(possible_rules);
        acc
    });

    let rules_order = find_rule_order(possible_rules_per_field);

    v.multiply_departure_fields(&rules_order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let inp = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        let data = generate(inp);
        assert!(data.is_some());
        assert_eq!(71, part1(&data.unwrap()));
    }

    #[test]
    fn test_part2_sample() {
        let inp = "departure location: 0-1 or 4-19
departure station: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let data = generate(inp);
        assert!(data.is_some());
        assert_eq!(11 * 12, part2(&data.unwrap()));
    }
}
