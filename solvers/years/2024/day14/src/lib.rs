use std::ops::Neg;

use common::{
    map::{vec2, Vec2},
    prelude::*,
};

use chumsky::prelude::*;
use itertools::Itertools;

register_solver!(2024, 14, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let mut input = parser().parse(input).unwrap();

        for _ in 0..100 {
            input.step();
        }

        let res = input
            .robots
            .iter()
            .filter_map(|r| {
                let mid_x = input.map_size.x / 2;
                let mid_y = input.map_size.y / 2;

                use std::cmp::Ordering::*;

                match (r.pos.x.cmp(&mid_x), r.pos.y.cmp(&mid_y)) {
                    (Less, Less) => Some(0),
                    (Less, Greater) => Some(1),
                    (Greater, Less) => Some(2),
                    (Greater, Greater) => Some(3),
                    (Equal, _) | (_, Equal) => None,
                }
            })
            .counts()
            .into_values()
            .map(|i| i as i64)
            .product::<i64>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let mut input = parser().parse(input).unwrap();

        // Map size is made of prime number, robots move in a modulus way so
        // they will come back to their original position at the lowest common
        // multiple, which for prime numbers is a * b.
        let max_iterations = input.map_size.x * input.map_size.y;

        // Due to the nature of today's problem, we have to use an heuristic to
        // estimate when we encounter the christmas tree.
        //
        // Here we find the minimum of the 2D variance of robots position (as we
        // assume the robots will be packed together to form the image).
        let christmas_tree_index = (1..(max_iterations))
            .map(|i| {
                input.step();

                let mean =
                    input.robots.iter().map(|r| r.pos).sum::<Vec2>() / input.robots.len() as i64;
                let square_mean = input.robots.iter().map(|r| r.pos * r.pos).sum::<Vec2>()
                    / input.robots.len() as i64;

                let variance = square_mean - (mean * mean);

                (i, variance)
            })
            .min_by_key(|(_, variance)| variance.square_norm())
            .map(|(i, _)| i)
            .unwrap();

        PartResult::new(christmas_tree_index)
    }
}

#[derive(Debug, Clone)]
struct Input {
    map_size: Vec2,
    robots: Vec<Robot>,
}
impl Input {
    fn step(&mut self) {
        for r in &mut self.robots {
            r.step(self.map_size);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Vec2,
    velocity: Vec2,
}

impl Robot {
    fn step(&mut self, map_size: Vec2) {
        self.pos = self.pos + self.velocity;
        self.pos.x = self.pos.x.rem_euclid(map_size.x);
        self.pos.y = self.pos.y.rem_euclid(map_size.y);
    }
}

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let number =
        just("-")
            .or_not()
            .then(text::int(10))
            .map(|(negative, i): (Option<&str>, String)| {
                let mut num = i.parse::<i64>().unwrap();
                if negative.is_some() {
                    num = num.neg()
                }
                num
            });

    let vec = number
        .then_ignore(just(","))
        .then(number)
        .map(|(x, y)| vec2(x, y))
        .boxed();

    let robot = just("p=")
        .ignore_then(vec.clone())
        .then_ignore(just(" v="))
        .then(vec.clone())
        .map(|(pos, velocity)| Robot { pos, velocity });

    let robots = robot.separated_by(text::newline());

    (vec.clone().then_ignore(text::newline()))
        .or_not()
        .then(robots)
        .map(|(map_size, robots)| Input {
            map_size: map_size.unwrap_or(vec2(101, 103)),
            robots,
        })
}
