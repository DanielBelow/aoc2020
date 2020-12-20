use crate::iterator_ext::IteratorExt;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Debug)]
pub enum Production {
    Terminal(char),
    Compound(Vec<usize>),
    Or((Vec<usize>, Vec<usize>)),
}

#[derive(Clone, Debug)]
pub struct Rule {
    idx: usize,
    grule: Production,
}

#[derive(Debug)]
pub struct Messages {
    rules: Vec<Rule>,
    text: Vec<String>,
}

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> Option<Messages> {
    let mut spl = inp.split("\n\n");

    let rules = spl.next()?;
    let rules = rules
        .lines()
        .filter_map(|it| {
            let colon_pos = it.chars().position(|it| it == ':')?;
            let from = it[..colon_pos].parse::<usize>().ok()?;

            let prods = &it[colon_pos + 1..];
            let production = if prods.contains('|') {
                // OR
                let mut psplt = prods.split('|');
                let lhs = psplt.next()?;
                let lhs = lhs
                    .split(' ')
                    .filter_map(|it| it.parse::<usize>().ok())
                    .collect_vec();
                let rhs = psplt.next()?;
                let rhs = rhs
                    .split(' ')
                    .filter_map(|it| it.parse::<usize>().ok())
                    .collect_vec();
                Production::Or((lhs, rhs))
            } else if prods.contains('"') {
                // TERM
                let chr = prods.chars().nth(2)?;
                Production::Terminal(chr)
            } else {
                // COMPOUND
                let comp = prods
                    .split(' ')
                    .filter_map(|it| it.parse::<usize>().ok())
                    .collect_vec();
                Production::Compound(comp)
            };

            Some(Rule {
                idx: from,
                grule: production,
            })
        })
        .collect_vec();

    let messages = spl.next()?;
    let text = messages.lines().map(String::from).collect_vec();

    let rules = rules.into_iter().sorted_by_key(|it| it.idx).collect_vec();

    Some(Messages { rules, text })
}

fn to_regex(idx: usize, rules: &[Rule], p2: bool) -> Option<String> {
    if p2 {
        if idx == 8 {
            return match to_regex(42, rules, p2) {
                Some(rgx) => Some(format!("{}+", rgx)),
                _ => None,
            };
        } else if idx == 11 {
            return to_regex(42, rules, p2).and_then(|fourty_two| {
                to_regex(31, rules, p2)
                    .map(|thirty_one| {
                        let mut res = String::new();
                        res.push('(');

                        // 11: 42 31
                        res.push_str(fourty_two.as_str());
                        res.push_str(thirty_one.as_str());

                        // 11: 42 11 31
                        for idx in 2..10 {
                            let num_rep = idx.to_string();

                            res.push('|');

                            res.push_str(fourty_two.as_str());
                            res.push('{');
                            res.push_str(num_rep.as_str());
                            res.push('}');

                            res.push_str(thirty_one.as_str());
                            res.push('{');
                            res.push_str(num_rep.as_str());
                            res.push('}');
                        }

                        res.push(')');

                        Some(res)
                    })
                    .unwrap_or_default()
            });
        }
    }

    let rule = rules.iter().find(|it| it.idx == idx)?;

    let mut res = String::new();
    match &rule.grule {
        Production::Terminal(c) => {
            let re = format!("[{}]", c);
            res.push_str(re.as_str());
        }
        Production::Compound(v) => {
            let v = v.iter().filter_map(|it| to_regex(*it, rules, p2)).join("");
            res.push_str(v.as_str());
        }
        Production::Or((lhs, rhs)) => {
            let lhs = lhs
                .iter()
                .filter_map(|it| to_regex(*it, rules, p2))
                .join("");
            let rhs = rhs
                .iter()
                .filter_map(|it| to_regex(*it, rules, p2))
                .join("");
            let re = format!("({}|{})", lhs, rhs);
            res.push_str(re.as_str());
        }
    }

    Some(res)
}

fn rules_to_regex(rules: &[Rule], p2: bool) -> Option<Regex> {
    match to_regex(0, rules, p2) {
        Some(rgx) => {
            let re = format!("^{}$", rgx);
            Regex::new(re.as_str()).ok()
        }
        _ => None,
    }
}

fn count_matches(msgs: &[String], re: &Regex) -> usize {
    msgs.iter().count_if(|it| re.is_match(it))
}

#[aoc(day19, part1)]
pub fn part1(data: &Messages) -> Option<usize> {
    match rules_to_regex(&data.rules, false) {
        Some(re) => Some(count_matches(&data.text, &re)),
        _ => None,
    }
}

#[aoc(day19, part2)]
pub fn part2(data: &Messages) -> Option<usize> {
    match rules_to_regex(&data.rules, true) {
        Some(re) => Some(count_matches(&data.text, &re)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

        let data = generate(inp).unwrap();
        assert_eq!(Some(2), part1(&data));
    }

    #[test]
    fn test_sample_p1_2() {
        let inp = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

        let data = generate(inp).unwrap();
        assert_eq!(Some(3), part1(&data));
    }

    #[test]
    fn test_sample_p2() {
        let inp = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

        let data = generate(inp).unwrap();
        assert_eq!(Some(12), part2(&data));
    }
}
