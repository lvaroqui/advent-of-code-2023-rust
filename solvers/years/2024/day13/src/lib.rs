use common::{
    map::{vec2, Vec2},
    prelude::*,
};

use chumsky::prelude::*;

register_solver!(2024, 13, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let machines = parser().parse(input).unwrap();

        let res = machines.into_iter().filter_map(compute_cost).sum::<i64>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let machines = parser().parse(input).unwrap();

        let res = machines
            .into_iter()
            .map(|m| Machine {
                prize: m.prize + 10000000000000,
                ..m
            })
            .filter_map(compute_cost)
            .sum::<i64>();

        PartResult::new(res)
    }
}

fn compute_cost(Machine { a, b, prize }: Machine) -> Option<i64> {
    // Solving the linear system
    let a_num = b.x * prize.y - b.y * prize.x;
    let a_denom = a.y * b.x - a.x * b.y;

    let b_num = a.x * prize.y - a.y * prize.x;
    let b_denom = b.y * a.x - b.x * a.y;

    // There is always exactly one solution in the input, the "finding the
    // minimum bit" is just to make us do harder math than needed :D
    assert!(a_denom != 0 && b_denom != 0, "More than one solution found");

    let a_remainder = a_num % a_denom;
    let b_remainder = b_num % b_denom;

    if a_remainder != 0 || b_remainder != 0 {
        return None;
    }

    let a = a_num / a_denom;
    let b = b_num / b_denom;

    Some(3 * a + b)
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

fn parser() -> impl Parser<char, Vec<Machine>, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<i64>().unwrap());

    let button_vector = just("X+")
        .ignore_then(number)
        .then_ignore(just(", Y+"))
        .then(number)
        .map(|(x, y)| vec2(x, y))
        .boxed();

    let button_a = just("Button A: ").ignore_then(button_vector.clone());
    let button_b = just("Button B: ").ignore_then(button_vector);
    let prize = just("Prize: X=")
        .ignore_then(number)
        .then_ignore(just(", Y="))
        .then(number)
        .map(|(x, y)| vec2(x, y))
        .boxed();

    let machine = button_a
        .then_ignore(text::newline())
        .then(button_b)
        .then_ignore(text::newline())
        .then(prize)
        .map(|((a, b), prize)| Machine { a, b, prize });

    machine.separated_by(text::whitespace())
}
