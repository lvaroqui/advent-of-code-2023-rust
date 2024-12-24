use cached::{Cached, UnboundCache};
use common::prelude::*;

use chumsky::prelude::*;
use rayon::prelude::*;

register_solver!(2024, 19, Solver);
pub struct Solver;

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let input = Box::leak(Box::new(parser().parse(input).unwrap()));
        let mut res = vec![];
        input
            .designs
            .par_iter()
            .map(|d| validate(&mut UnboundCache::new(), d, &input.patterns, 0))
            .collect_into_vec(&mut res);

        (
            PartResult::new(res.iter().filter(|i| **i > 0).count()),
            PartResult::new(res.iter().sum::<usize>()),
        )
    }
}

fn validate(
    cache: &mut UnboundCache<(u8, u8), usize>,
    target: &'static [Color],
    patterns: &'static [Vec<Color>],
    pattern_index: usize,
) -> usize {
    let cache_key = (target.len() as u8, pattern_index as u8);
    if let Some(c) = cache.cache_get(&cache_key) {
        return *c;
    }

    if pattern_index >= patterns.len() {
        return 0;
    }

    let pattern = patterns[pattern_index].as_slice();

    let res = if pattern.len() <= target.len() && target.starts_with(pattern) {
        if target.len() == pattern.len() {
            1
        } else {
            validate(cache, &target[pattern.len()..], patterns, 0)
        }
    } else {
        0
    };

    let res = res + validate(cache, target, patterns, pattern_index + 1);
    cache.cache_set(cache_key, res);
    res
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
