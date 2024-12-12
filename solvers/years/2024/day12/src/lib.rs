use std::collections::HashSet;

use common::{
    map::{Map, Vec2},
    prelude::*,
};

use chumsky::prelude::*;
use itertools::Itertools;

register_solver!(2024, 12, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let mut visited = HashSet::new();

        let res = input
            .iter()
            .map(|(pos, _c)| {
                let rs = explore_region(&mut visited, &input, pos);
                rs.area * rs.perimeters.len()
            })
            .sum::<usize>();

        PartResult::new(res)
    }
    fn solve_2(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let mut visited = HashSet::new();

        let res = input
            .iter()
            .map(|(pos, _c)| {
                let rs = explore_region(&mut visited, &input, pos);
                let sides = rs.sides();
                rs.area * sides
            })
            .sum::<usize>();

        PartResult::new(res)
    }
}

#[derive(Debug, Default, Clone)]
struct RegionStats {
    area: usize,
    perimeters: Vec<Perimeter>,
}

impl RegionStats {
    fn sides(&self) -> usize {
        let directions = self
            .perimeters
            .iter()
            .copied()
            .map(|d| {
                if d.direction.is_horizontal() {
                    (d.direction, (d.start_pos.x, d.start_pos.y))
                } else {
                    (d.direction, (d.start_pos.y, d.start_pos.x))
                }
            })
            .into_group_map();

        directions
            .into_values()
            .map(|lines| {
                let lines = lines.into_iter().into_group_map();
                lines
                    .into_values()
                    .map(|mut points| {
                        points.sort();
                        let mut prec = None;

                        points
                            .into_iter()
                            .filter(|value| {
                                let res = if let Some(prec) = prec {
                                    *value != prec + 1
                                } else {
                                    true
                                };
                                prec = Some(*value);
                                res
                            })
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl std::ops::Add<RegionStats> for RegionStats {
    type Output = RegionStats;

    fn add(mut self, rhs: RegionStats) -> Self::Output {
        self.area += rhs.area;
        self.perimeters.extend(rhs.perimeters);
        self
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

#[derive(Debug, Clone, Copy)]
struct Perimeter {
    start_pos: Vec2,
    direction: Direction,
}

fn explore_region(visited: &mut HashSet<Vec2>, map: &Map<char>, pos: Vec2) -> RegionStats {
    if visited.contains(&pos) {
        return RegionStats::default();
    }
    visited.insert(pos);

    let region_stats = RegionStats {
        area: 1,
        perimeters: map
            .four_adjacent_pos_iter_unchecked(pos)
            .filter(|p| match map.get(*p) {
                Some(c) => map[pos] != *c,
                None => true,
            })
            .map(|p| match p - pos {
                Vec2::NORTH => Perimeter {
                    start_pos: pos,
                    direction: Direction::Up,
                },
                Vec2::SOUTH => Perimeter {
                    start_pos: pos + Vec2::SOUTH,
                    direction: Direction::Down,
                },
                Vec2::WEST => Perimeter {
                    start_pos: pos,
                    direction: Direction::Left,
                },
                Vec2::EAST => Perimeter {
                    start_pos: pos + Vec2::EAST,
                    direction: Direction::Right,
                },
                _ => unreachable!(),
            })
            .collect(),
    };

    map.four_adjacent_pos_iter(pos)
        .filter(|p| map[pos] == map[*p])
        .map(|p| explore_region(visited, map, p))
        .fold(region_stats, |acc, rs| acc + rs)
}

fn parser() -> impl Parser<char, Map<char>, Error = Simple<char>> {
    let c = text::newline().not();

    let line = c.repeated();

    line.separated_by(text::newline()).map(Map::new)
}
