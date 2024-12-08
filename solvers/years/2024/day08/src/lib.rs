use std::collections::{HashMap, HashSet};

use common::{map::Map, prelude::*};

use chumsky::prelude::*;
use itertools::Itertools;

register_solver!(2024, 8, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let (map, antennas) = parse(input);

        let mut antinodes = HashSet::new();

        for antenna_positions in antennas.values() {
            for pair in antenna_positions.iter().permutations(2) {
                let a = *pair[0];
                let b = *pair[1];

                for antinode in [a + (a - b), b + (b - a)] {
                    if map.get(antinode).is_some() {
                        antinodes.insert(antinode);
                    }
                }
            }
        }

        PartResult::new(antinodes.len())
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let (map, antennas) = parse(input);

        let mut antinodes = HashSet::new();

        for antenna_positions in antennas.values() {
            for pair in antenna_positions.iter().permutations(2) {
                let a = *pair[0];
                let b = *pair[1];

                for (pos, _) in map
                    .iter_from_point(a, a - b)
                    .chain(map.iter_from_point(b, b - a))
                {
                    antinodes.insert(pos);
                }
            }
        }

        PartResult::new(antinodes.len())
    }
}

fn parse(input: &str) -> (Map<Cell>, HashMap<char, Vec<common::map::Vec2>>) {
    let map = parser().parse(input).unwrap();

    let antennas = map
        .iter()
        .filter_map(|(pos, c)| match c {
            Cell::Empty => None,
            Cell::Antenna(c) => Some((*c, pos)),
        })
        .into_group_map();
    (map, antennas)
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Antenna(char),
}

fn parser() -> impl Parser<char, Map<Cell>, Error = Simple<char>> {
    let cell = just(".")
        .to(Cell::Empty)
        .or(text::newline().not().map(Cell::Antenna));

    let line = cell.repeated();

    line.separated_by(text::newline()).map(Map::new)
}
