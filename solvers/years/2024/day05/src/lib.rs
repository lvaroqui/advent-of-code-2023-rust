use common::prelude::*;

use chumsky::prelude::*;

register_solver!(2024, 5, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let res = input
            .lists
            .iter()
            .filter(|list| input.rules.iter().all(|r| r.validate(list)))
            .map(|list| list[list.len() / 2] as u32)
            .sum::<u32>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        // See https://github.com/rust-lang/rust-clippy/issues/13185
        #![allow(clippy::manual_inspect)]

        let mut input = parser().parse(input).unwrap();

        let res = input
            .lists
            .iter_mut()
            .filter(|list| !input.rules.iter().all(|r| r.validate(list)))
            .map(|list| {
                loop {
                    let mut goods = 0;
                    for rule in &input.rules {
                        if rule.fix(list) {
                            goods += 1;
                        }
                    }
                    if goods == input.rules.len() {
                        break;
                    }
                }
                list
            })
            .map(|list| list[list.len() / 2] as u32)
            .sum::<u32>();

        PartResult::new(res)
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    before: u8,
    after: u8,
}

impl Rule {
    fn validate(&self, list: &[u8]) -> bool {
        let mut after_seen = false;
        for &val in list {
            if val == self.after {
                after_seen = true;
            } else if val == self.before {
                return !after_seen;
            }
        }
        true
    }

    fn fix(&self, list: &mut [u8]) -> bool {
        let mut after_index = None;
        let mut before_index = None;
        for (i, &val) in list.iter().enumerate() {
            if val == self.after {
                after_index = Some(i);
            } else if val == self.before {
                match after_index.is_none() {
                    // List is valid
                    true => return true,
                    false => {
                        before_index = Some(i);
                        break;
                    }
                }
            }
        }

        if let (Some(before_index), Some(after_index)) = (before_index, after_index) {
            list.swap(before_index, after_index);
            false
        } else {
            true
        }
    }
}

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    lists: Vec<Vec<u8>>,
}

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<u8>().unwrap());

    let rule = number
        .then_ignore(just("|"))
        .then(number)
        .map(|(before, after)| Rule { before, after });

    let list = number.separated_by(just(","));

    rule.separated_by(just("\n"))
        .then_ignore(just("\n\n"))
        .then(list.separated_by(just("\n")))
        .map(|(rules, lists)| Input { lists, rules })
}
