use std::ops::Range;

use chumsky::prelude::*;
use common::prelude::*;
use range_collections::{range_set::RangeSetRange, RangeSet2};

register_solver!(2023, 5, Solver);
pub struct Solver;

#[derive(Debug, Clone, Copy)]
struct RangeMapping {
    destination: i64,
    source: i64,
    size: i64,
}

impl RangeMapping {
    fn map(&self, element: i64) -> Option<i64> {
        if self.source_range().contains(&element) {
            Some(self.destination + (element - self.source))
        } else {
            None
        }
    }

    fn source_range(&self) -> Range<i64> {
        self.source..self.source + self.size
    }
}

#[derive(Debug)]
struct Mapping {
    _from: String,
    _to: String,
    mappings: Vec<RangeMapping>,
}
type RangeSet = RangeSet2<i64>;

impl Mapping {
    fn map(&self, element: i64) -> i64 {
        self.mappings
            .iter()
            .find_map(|r| r.map(element))
            .unwrap_or(element)
    }

    fn map_range(&self, mut range: RangeSet) -> RangeSet {
        let mut res = RangeSet::empty();
        for mapping in &self.mappings {
            // Extract from input range values that are mapped by the current
            // mapping and add it to ouput
            let to_map = RangeSet::from(mapping.source_range()) & &range;
            res |= offset_range(&to_map, mapping.destination - mapping.source);

            // Remove processed range from input
            range ^= to_map;
        }
        // Map unmapped ranges to identity
        res | range
    }
}

#[derive(Debug)]
struct Input {
    individual_seeds: Vec<i64>,
    range_seeds: RangeSet,
    mappings: Vec<Mapping>,
}

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let mut elements = input.individual_seeds.clone();

        for mapping in input.mappings {
            for element in &mut elements {
                *element = mapping.map(*element);
            }
        }

        PartResult::new(elements.iter().min().unwrap())
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let mut ranges = input.range_seeds.clone();

        for mapping in &input.mappings {
            ranges = mapping.map_range(ranges);
        }

        PartResult::new(match ranges.iter().next().unwrap() {
            RangeSetRange::Range(r) => r.start,
            RangeSetRange::RangeFrom(_) => unreachable!(),
        })
    }
}

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<i64>().unwrap());

    let seeds = just("seeds: ").ignore_then(
        number.separated_by(text::whitespace()).rewind().then(
            ((number
                .separated_by(text::whitespace())
                .exactly(2)
                .map(|n| n[0]..n[0] + n[1]))
            .separated_by(text::whitespace()))
            .map(|ranges| {
                ranges
                    .into_iter()
                    .fold(RangeSet::empty(), |acc, r| acc | RangeSet::from(r))
            }),
        ),
    );

    let range = number
        .separated_by(text::whitespace())
        .exactly(3)
        .map(|numbers| RangeMapping {
            destination: numbers[0],
            source: numbers[1],
            size: numbers[2],
        });

    let mapping = text::ident()
        .then_ignore(just("-to-"))
        .then(text::ident())
        .then_ignore(just(" map:").then(text::newline()))
        .then(range.separated_by(text::newline()))
        .map(|((from, to), ranges)| Mapping {
            _from: from,
            _to: to,
            mappings: ranges,
        });

    seeds
        .then_ignore(text::whitespace())
        .then(mapping.padded().repeated())
        .map(|((individual_seeds, range_seeds), mappings)| Input {
            individual_seeds,
            range_seeds,
            mappings,
        })
        .then_ignore(end())
}

// Offset a range by the provided amount.
fn offset_range(range: &RangeSet, offset: i64) -> RangeSet {
    let mut res = RangeSet::empty();
    for part in range.iter() {
        res |= match part {
            RangeSetRange::Range(s) => RangeSet::from(*s.start + offset..*s.end + offset),
            RangeSetRange::RangeFrom(_) => unreachable!(),
        };
    }
    res
}
