use chumsky::prelude::*;

use common::prelude::*;

register_solver!(2023, 2, Solver);

pub struct Solver;

#[derive(Debug, Default)]
struct Set([u32; 3]);

impl Set {
    fn possible_with(&self, bag: &Set) -> bool {
        self.0.iter().zip(bag.0.iter()).all(|(l, r)| l <= r)
    }

    fn power(&self) -> u32 {
        self.0.iter().product()
    }

    fn max(mut self, other: &Set) -> Self {
        self.0
            .iter_mut()
            .zip(other.0.iter())
            .for_each(|(l, r)| *l = (*l).max(*r));
        self
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let lines = input.split('\n');
        let parser = parser();

        let bag = Set([12, 13, 14]);

        let res = lines
            .map(|line| parser.parse(line).unwrap())
            .filter_map(|game| {
                game.sets
                    .iter()
                    .all(|s| s.possible_with(&bag))
                    .then_some(game.id)
            })
            .sum::<u32>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let lines = input.split('\n');
        let parser = parser();

        let res = lines
            .map(|line| parser.parse(line).unwrap())
            .map(|game| {
                game.sets
                    .into_iter()
                    .reduce(|acc, val| acc.max(&val))
                    .unwrap()
                    .power()
            })
            .sum::<u32>();

        PartResult::new(res)
    }
}

fn parser() -> impl Parser<char, Game, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<u32>().unwrap());

    let set = number
        .padded()
        .then(
            just("red")
                .map(|_| 0)
                .or(just("green").map(|_| 1))
                .or(just("blue").map(|_| 2)),
        )
        .separated_by(just(','))
        .map(|colors| {
            let mut set = Set::default();
            for (value, color) in colors {
                set.0[color] += value
            }

            set
        });

    just("Game ")
        .ignore_then(number)
        .then_ignore(just(':'))
        .then(set.separated_by(just(';')))
        .map(|(id, sets)| Game { id, sets })
}
