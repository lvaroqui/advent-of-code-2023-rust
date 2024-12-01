use std::collections::HashMap;

use chumsky::prelude::*;
use common::prelude::*;

use itertools::Itertools;

register_solver!(2023, 8, Solver);
pub struct Solver;

#[derive(Debug, Clone, Copy)]
enum Direction {
    L,
    R,
}

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        solve(
            input,
            |name| name == "AAA", //
            |name| name == "ZZZ",
        )
    }

    fn solve_2(&self, input: &str) -> DayResult {
        solve(
            input,
            |name| name.ends_with('A'),
            |name| name.ends_with('Z'),
        )
    }
}

fn solve(
    input: &str,
    start_condition: impl Fn(&str) -> bool,
    end_condition: impl Fn(&str) -> bool,
) -> DayResult {
    let input = parser().parse(input).unwrap();
    let starts = input
        .nodes
        .keys()
        .filter(|name| start_condition(name))
        .collect_vec();

    let mut counts = vec![];

    for start in starts {
        let mut step_count = 0_u64;
        let mut current = start;

        while !end_condition(current) {
            for d in &input.directions {
                step_count += 1;
                let children = input.nodes.get(current).unwrap();
                current = match d {
                    Direction::L => &children.0,
                    Direction::R => &children.1,
                }
            }
        }

        counts.push(step_count);
    }

    DayResult::new(counts.into_iter().reduce(num::integer::lcm).unwrap())
}

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let direction = just('R').to(Direction::R).or(just('L').to(Direction::L));
    let directions = direction.repeated();

    let node_name = text::ident();
    let children = node_name
        .separated_by(just(',').padded())
        .exactly(2)
        .delimited_by(just('('), just(')'));
    let node = node_name.then_ignore(just('=').padded()).then(children);

    directions
        .then_ignore(text::newline())
        .then(node.padded().repeated())
        .then_ignore(end())
        .map(|(directions, nodes)| Input {
            directions,
            nodes: nodes
                .into_iter()
                .map(|(name, mut children)| {
                    (
                        name,
                        (
                            std::mem::take(&mut children[0]),
                            std::mem::take(&mut children[1]),
                        ),
                    )
                })
                .collect(),
        })
}
