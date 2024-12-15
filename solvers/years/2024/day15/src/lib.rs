use common::{
    map::{Map, Vec2},
    prelude::*,
};

use chumsky::prelude::*;

register_solver!(2024, 15, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let mut input = parser(map_parser_1()).parse(input).unwrap();
        input.simulate();
        PartResult::new(input.compute_sum_of_gps_coordinates())
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let mut input = parser(map_parser_2()).parse(input).unwrap();
        input.simulate();
        PartResult::new(input.compute_sum_of_gps_coordinates())
    }
}

fn attempt_move(map: &mut Map<Cell>, from: Vec2, mov: Vec2) -> Option<Vec2> {
    debug_assert!(matches!(
        map[from],
        Cell::Box | Cell::LeftBox | Cell::RightBox | Cell::Robot
    ));

    if is_move_valid(map, from, mov) {
        apply_move_unchecked(map, from, mov);
        Some(from + mov)
    } else {
        None
    }
}

fn is_move_valid(map: &Map<Cell>, from: Vec2, mov: Vec2) -> bool {
    debug_assert!(matches!(
        map[from],
        Cell::Box | Cell::LeftBox | Cell::RightBox | Cell::Robot
    ));

    let next_pos = from + mov;
    match map[next_pos] {
        Cell::Empty => true,
        Cell::Box => is_move_valid(map, next_pos, mov),
        Cell::LeftBox if mov == Vec2::WEST => true,
        Cell::RightBox if mov == Vec2::EAST => true,
        c @ Cell::LeftBox | c @ Cell::RightBox => {
            let other_part_offset = match c {
                Cell::LeftBox => Vec2::EAST,
                Cell::RightBox => Vec2::WEST,
                _ => unreachable!(),
            };
            is_move_valid(map, next_pos, mov)
                && is_move_valid(map, next_pos + other_part_offset, mov)
        }
        Cell::Wall => false,
        Cell::Robot => unreachable!(),
    }
}

fn apply_move_unchecked(map: &mut Map<Cell>, from: Vec2, mov: Vec2) {
    let next_pos = from + mov;
    match map[next_pos] {
        Cell::Empty => (),
        Cell::Box => {
            apply_move_unchecked(map, next_pos, mov);
        }
        Cell::RightBox if mov == Vec2::WEST => {
            apply_move_unchecked(map, next_pos + Vec2::WEST, mov);
            apply_move_unchecked(map, next_pos, mov);
        }
        Cell::LeftBox if mov == Vec2::EAST => {
            apply_move_unchecked(map, next_pos + Vec2::EAST, mov);
            apply_move_unchecked(map, next_pos, mov);
        }
        c @ Cell::LeftBox | c @ Cell::RightBox => {
            let other_part_offset = match c {
                Cell::LeftBox => Vec2::EAST,
                Cell::RightBox => Vec2::WEST,
                _ => unreachable!(),
            };
            apply_move_unchecked(map, next_pos, mov);
            apply_move_unchecked(map, next_pos + other_part_offset, mov);
        }
        Cell::Robot | Cell::Wall => unreachable!(),
    }

    map[next_pos] = map[from];
    map[from] = Cell::Empty;
}

#[derive(Debug, Clone)]
struct Input {
    map: Map<Cell>,
    moves: Vec<Vec2>,
}

impl Input {
    fn simulate(&mut self) {
        let mut robot_pos = self.map.iter().find(|(_, c)| **c == Cell::Robot).unwrap().0;

        for m in &self.moves {
            if let Some(next_pos) = attempt_move(&mut self.map, robot_pos, *m) {
                robot_pos = next_pos;
            }
        }
    }

    fn compute_sum_of_gps_coordinates(&self) -> i64 {
        self.map
            .iter()
            .filter(|(_, c)| matches!(c, Cell::Box | Cell::LeftBox))
            .map(|(p, _)| p.x + p.y * 100)
            .sum::<i64>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Box,
    LeftBox,
    RightBox,
    Wall,
    Robot,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Cell::Empty => ".",
            Cell::Box => "O",
            Cell::LeftBox => "[",
            Cell::RightBox => "]",
            Cell::Wall => "#",
            Cell::Robot => "@",
        })
    }
}

fn map_parser_1() -> impl Parser<char, Map<Cell>, Error = Simple<char>> {
    let cell = choice([
        just(".").to(Cell::Empty),
        just("#").to(Cell::Wall),
        just("@").to(Cell::Robot),
        just("O").to(Cell::Box),
    ]);

    let line = cell.repeated().at_least(1);

    line.separated_by(text::newline()).map(Map::new)
}

fn map_parser_2() -> impl Parser<char, Map<Cell>, Error = Simple<char>> {
    let cell = choice([
        just(".").to(vec![Cell::Empty, Cell::Empty]),
        just("#").to(vec![Cell::Wall, Cell::Wall]),
        just("@").to(vec![Cell::Robot, Cell::Empty]),
        just("O").to(vec![Cell::LeftBox, Cell::RightBox]),
    ]);

    let line = cell.repeated().at_least(1).flatten();

    line.separated_by(text::newline()).map(Map::new)
}

fn parser(
    map_parser: impl Parser<char, Map<Cell>, Error = Simple<char>>,
) -> impl Parser<char, Input, Error = Simple<char>> {
    let moves = choice([
        just("^").to(Vec2::NORTH),
        just("v").to(Vec2::SOUTH),
        just("<").to(Vec2::WEST),
        just(">").to(Vec2::EAST),
    ])
    .separated_by(text::newline().or_not());

    map_parser
        .then_ignore(text::newline().repeated())
        .then(moves)
        .map(|(map, moves)| Input { map, moves })
        .then_ignore(end())
}
