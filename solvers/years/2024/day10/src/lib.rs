use std::collections::HashSet;

use common::{
    map::{Map, Vec2},
    prelude::*,
};

use chumsky::prelude::*;

register_solver!(2024, 10, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let res = input
            .iter()
            .filter(|(_, h)| **h == 0)
            .map(|(pos, h)| {
                let mut nines = HashSet::new();
                compute_score(&input, *h, pos, &mut nines);
                nines.len()
            })
            .sum::<usize>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let res = input
            .iter()
            .filter(|(_, h)| **h == 0)
            .map(|(pos, h)| compute_rating(&input, *h, pos))
            .sum::<usize>();

        PartResult::new(res)
    }
}

fn compute_score(map: &Map<u8>, h: u8, pos: common::map::Vec2, nines: &mut HashSet<Vec2>) {
    if h == 9 {
        nines.insert(pos);
        return;
    }

    for adjacent in map.four_adjacent_pos_iter(pos) {
        if map[adjacent] == h + 1 {
            compute_score(map, h + 1, adjacent, nines);
        }
    }
}

fn compute_rating(map: &Map<u8>, h: u8, pos: common::map::Vec2) -> usize {
    if h == 9 {
        return 1;
    }

    map.four_adjacent_pos_iter(pos)
        .map(|adjacent| {
            if map[adjacent] == h + 1 {
                compute_rating(map, h + 1, adjacent)
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn parser() -> impl Parser<char, Map<u8>, Error = Simple<char>> {
    let digit = text::newline()
        .not()
        .map(|c: char| c.to_digit(10).unwrap() as u8);

    let line = digit.repeated();

    line.separated_by(text::newline()).map(Map::new)
}
