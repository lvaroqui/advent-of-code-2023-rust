use common::prelude::*;

use chumsky::prelude::*;
use itertools::Itertools;

register_solver!(2024, 19, Solver);
pub struct Solver;

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let input = Box::leak(Box::new(parser().parse(input).unwrap()));

        let (a, b) = input
            .designs
            .iter()
            .enumerate()
            .map(|(i, d)| {
                println!("{}", i);
                validate(d, &input.patterns, 0)
            })
            .tee();

        (
            PartResult::new(a.filter(|i| *i > 0).count()),
            PartResult::new(b.sum::<usize>()),
        )
    }
}

#[cached::proc_macro::cached]
fn validate(
    target: &'static [Color],
    patterns: &'static [Vec<Color>],
    pattern_index: usize,
) -> usize {
    if pattern_index >= patterns.len() {
        return 0;
    }

    let pattern = patterns[pattern_index].as_slice();

    let res = if pattern.len() <= target.len() && target.starts_with(pattern) {
        if target.len() == pattern.len() {
            1
        } else {
            validate(&target[pattern.len()..], patterns, 0)
        }
    } else {
        0
    };

    res + validate(target, patterns, pattern_index + 1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug)]
struct Input {
    patterns: Vec<Vec<Color>>,
    designs: Vec<Vec<Color>>,
}

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let color = choice([
        just("w").to(Color::White),
        just("u").to(Color::Blue),
        just("b").to(Color::Black),
        just("r").to(Color::Red),
        just("g").to(Color::Green),
    ]);

    let pattern = color.repeated();

    let patterns = pattern.separated_by(just(",").padded());
    let designs = pattern.separated_by(text::newline());

    patterns
        .then_ignore(text::whitespace())
        .then(designs)
        .map(|(patterns, designs)| Input { patterns, designs })
}
