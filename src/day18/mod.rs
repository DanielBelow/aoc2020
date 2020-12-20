use aoc_runner_derive::{aoc, aoc_generator};

use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr, PartialEq, Copy, Clone, Debug)]
pub enum MathToken {
    #[display("+")]
    Add,
    #[display("*")]
    Mul,
    #[display("(")]
    LParen,
    #[display(")")]
    RParen,
    #[display("{0}")]
    Number(usize),
}

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Vec<Vec<MathToken>> {
    inp.lines()
        .map(|it| {
            it.replace("(", "( ")
                .replace(")", " )")
                .split(' ')
                .filter_map(|t| {
                    t.parse::<MathToken>()
                        .map_err(|e| println!("Error: {}", e))
                        .ok()
                })
                .collect()
        })
        .collect()
}

fn replace_simple_parens(idx: usize, tokens: &[MathToken]) -> Option<MathToken> {
    match (tokens[idx], tokens[idx + 1], tokens[idx + 2]) {
        (MathToken::LParen, op, MathToken::RParen) => Some(op),
        _ => None,
    }
}

fn replace_simple_arith(idx: usize, tokens: &[MathToken]) -> Option<MathToken> {
    match (tokens[idx], tokens[idx + 1], tokens[idx + 2]) {
        (MathToken::Number(lhs), op, MathToken::Number(rhs)) => match op {
            MathToken::Add => Some(MathToken::Number(lhs + rhs)),
            MathToken::Mul => Some(MathToken::Number(lhs * rhs)),
            _ => None,
        },
        _ => None,
    }
}

fn simplify_parens(tokens: &[MathToken]) -> (bool, Vec<MathToken>) {
    let mut res = Vec::new();
    let mut simplified = false;

    let mut idx = 0;
    loop {
        if idx + 2 >= tokens.len() {
            break;
        }

        if let Some(replace) = replace_simple_parens(idx, &tokens) {
            res.push(replace);
            idx += 3;
            simplified = true;
        } else {
            res.push(tokens[idx]);
            idx += 1;
        }
    }

    for tok in &tokens[idx..] {
        res.push(*tok);
    }

    (simplified, res)
}

fn simplify_left_to_right(tokens: &[MathToken]) -> (bool, Vec<MathToken>) {
    let mut res = Vec::new();
    let mut simplified = false;

    let mut idx = 0;
    loop {
        if idx + 2 >= tokens.len() {
            break;
        }

        if let Some(replace) = replace_simple_arith(idx, &tokens) {
            res.push(replace);
            simplified = true;
            idx += 3;
            break;
        } else {
            res.push(tokens[idx]);
            idx += 1;
        }
    }

    for tok in &tokens[idx..] {
        res.push(*tok);
    }

    (simplified, res)
}

fn simplify(toks: &[MathToken]) -> usize {
    let simplify_step = |toks: &[MathToken]| -> (bool, Vec<MathToken>) {
        let (ltr, res) = simplify_left_to_right(toks);
        let (paren, res) = simplify_parens(&res);

        (ltr || paren, res)
    };

    let mut cur_toks = toks.to_owned();
    loop {
        let (simplified, res) = simplify_step(&cur_toks);
        cur_toks = res;

        if !simplified {
            break;
        }
    }

    assert_eq!(1, cur_toks.len());

    if let MathToken::Number(n) = cur_toks[0] {
        n
    } else {
        panic!("Invalid remaining token")
    }
}

fn find_closing_paren(start_idx: usize, toks: &[MathToken]) -> usize {
    let mut paren_depth = 0;

    for (idx, tok) in toks.iter().enumerate().skip(start_idx) {
        if matches!(tok, MathToken::RParen) && paren_depth == 0 {
            return idx;
        }

        if matches!(tok, MathToken::LParen) {
            paren_depth += 1;
        }

        if matches!(tok, MathToken::RParen) && paren_depth > 0 {
            paren_depth -= 1;
        }
    }

    panic!("Didn't find RParen")
}

fn find_opening_paren(start_idx: usize, toks: &[MathToken]) -> usize {
    let mut paren_depth = 0;

    for (idx, tok) in toks.iter().enumerate().take(start_idx).rev() {
        if matches!(tok, MathToken::LParen) && paren_depth == 0 {
            return idx;
        }

        if matches!(tok, MathToken::RParen) {
            paren_depth += 1;
        }

        if matches!(tok, MathToken::LParen) && paren_depth > 0 {
            paren_depth -= 1;
        }
    }

    panic!("Didn't find LParen");
}

fn wrap_add_in_parens(toks: &[MathToken]) -> Vec<MathToken> {
    if !toks.iter().any(|it| matches!(it, MathToken::Add)) {
        return toks.to_owned();
    }

    let mut res = toks.to_owned();

    let mut last_add_idx = 0;
    loop {
        let add_pos = res
            .iter()
            .skip(last_add_idx)
            .position(|it| matches!(it, MathToken::Add));

        if let Some(add_pos) = add_pos {
            let add_pos = add_pos + last_add_idx;

            let start_pos = if matches!(res[add_pos - 1], MathToken::RParen) {
                find_opening_paren(add_pos - 2, &res)
            } else {
                add_pos - 1
            };

            let end_pos = if matches!(res[add_pos + 1], MathToken::LParen) {
                find_closing_paren(add_pos + 2, &res)
            } else {
                add_pos + 2
            };

            res.insert(end_pos, MathToken::RParen);
            res.insert(start_pos, MathToken::LParen);
            last_add_idx = add_pos + 2;
        } else {
            break;
        }
    }

    res
}

fn simplify_with_precedence(toks: &[MathToken]) -> usize {
    let cur_toks = wrap_add_in_parens(toks);
    simplify(&cur_toks)
}

#[aoc(day18, part1)]
pub fn part1(toks: &[Vec<MathToken>]) -> usize {
    toks.iter().map(|it| simplify(it)).sum()
}

#[aoc(day18, part2)]
pub fn part2(toks: &[Vec<MathToken>]) -> usize {
    toks.iter().map(|it| simplify_with_precedence(it)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_paren_1() {
        let toks = vec![MathToken::LParen, MathToken::Number(0), MathToken::RParen];
        let (simp, res) = simplify_parens(&toks);
        assert!(simp);
        assert_eq!(vec![MathToken::Number(0)], res);
    }

    #[test]
    fn simplify_paren_2() {
        let toks = vec![
            MathToken::LParen,
            MathToken::Number(0),
            MathToken::RParen,
            MathToken::Add,
            MathToken::LParen,
            MathToken::Number(5),
            MathToken::RParen,
        ];
        let (simp, res) = simplify_parens(&toks);
        assert!(simp);
        assert_eq!(
            vec![MathToken::Number(0), MathToken::Add, MathToken::Number(5)],
            res
        );
    }

    #[test]
    fn simplify_first_1() {
        let toks = vec![
            MathToken::Number(2),
            MathToken::Add,
            MathToken::Number(3),
            MathToken::Mul,
            MathToken::Number(4),
        ];

        let (simp, res) = simplify_left_to_right(&toks);
        assert!(simp);
        assert_eq!(
            vec![MathToken::Number(5), MathToken::Mul, MathToken::Number(4)],
            res
        );
    }

    #[test]
    fn test_part1_1() {
        let inp = "2 * 3 + (4 * 5)";
        let toks = generate(inp);
        assert_eq!(26, part1(&toks));
    }

    #[test]
    fn test_part1_2() {
        let inp = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let toks = generate(inp);
        assert_eq!(437, part1(&toks));
    }

    #[test]
    fn test_part1_3() {
        let inp = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let toks = generate(inp);
        assert_eq!(12240, part1(&toks));
    }

    #[test]
    fn test_part1_4() {
        let inp = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let toks = generate(inp);
        assert_eq!(13632, part1(&toks));
    }

    #[test]
    fn test_part2_1() {
        let inp = "2 * 3 + (4 * 5)";
        let toks = generate(inp);
        assert_eq!(46, part2(&toks));
    }

    #[test]
    fn test_part2_2() {
        let inp = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let toks = generate(inp);
        assert_eq!(1445, part2(&toks));
    }

    #[test]
    fn test_part2_3() {
        let inp = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let toks = generate(inp);
        assert_eq!(669060, part2(&toks));
    }

    #[test]
    fn test_part2_4() {
        let inp = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let toks = generate(inp);
        assert_eq!(23340, part2(&toks));
    }

    #[test]
    fn test_part2_5() {
        let inp = "5 * 9 * (7 * 3 * 3 + 9 * (8 + 6 * 4) + 3)";
        let toks = generate(inp);
        assert_eq!(669060, part2(&toks));
    }

    #[test]
    fn test_foo() {
        let inp ="7 + 3 * (9 * (4 + 9 + 6 + 2) + (5 + 9 * 8 + 6 * 5) * 6 * 8 * (3 + 9 * 8 + 8 + 5)) * 3 + 4 * 6";
        let toks = generate(inp);
        assert_eq!(45768602880, part2(&toks));
    }
}
