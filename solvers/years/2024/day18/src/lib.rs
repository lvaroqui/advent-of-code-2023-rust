use std::collections::HashMap;

use common::{
    map::{vec2, Vec2},
    math_helper::partition_point,
    prelude::*,
};

use chumsky::prelude::*;
use pathfinding::prelude::astar;

register_solver!(2024, 18, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let cost = solve(&input, input.target(), input.steps()).unwrap();

        PartResult::new(cost)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let step = partition_point(input.steps() + 1..input.bytes.len(), |i| {
            solve(&input, input.target(), i).is_some()
        });
        let index = step - 1;

        let res = input.bytes.iter().find(|(_, i)| **i == index).unwrap().0;
        PartResult::new(format!("{},{}", res.x, res.y))
    }
}

fn solve(input: &Input, target: Vec2, steps: usize) -> Option<i32> {
    let start_pos = vec2(0, 0);

    astar(
        &start_pos,
        |p| {
            p.four_adjacent_iter()
                .filter(|n| {
                    let is_blocked = input.bytes.get(n).map(|i| *i < steps).unwrap_or(false);
                    n.x >= 0 && n.x <= target.x && n.y >= 0 && n.y <= target.y && !is_blocked
                })
                .map(|n| (n, 1))
        },
        |_| 0,
        |p| *p == target,
    )
    .map(|(_, cost)| cost)
}

#[derive(Debug)]
struct Input {
    is_test: bool,
    bytes: HashMap<Vec2, usize>,
}

impl Input {
    fn target(&self) -> Vec2 {
        if self.is_test {
            vec2(6, 6)
        } else {
            vec2(70, 70)
        }
    }

    fn steps(&self) -> usize {
        if self.is_test {
            12
        } else {
            1024
        }
    }
}

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<i64>().unwrap());

    let vec = number
        .then_ignore(just(","))
        .then(number)
        .map(|(x, y)| vec2(x, y));

    let is_test = just("test")
        .then_ignore(text::newline())
        .or_not()
        .map(|is_test| is_test.is_some());

    is_test
        .then(vec.separated_by(text::newline()))
        .then_ignore(end())
        .map(|(is_test, bytes)| Input {
            is_test,
            bytes: bytes.into_iter().enumerate().map(|(i, v)| (v, i)).collect(),
        })
}
