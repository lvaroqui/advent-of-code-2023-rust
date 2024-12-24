use std::collections::{HashMap, HashSet};

use common::{
    map::{Map, Vec2},
    prelude::*,
};

use chumsky::prelude::*;
use itertools::Itertools;

register_solver!(2024, 20, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let map = parser().parse(input).unwrap();

        let start = map
            .iter()
            .find_map(|(v, s)| if let Cell::Start = s { Some(v) } else { None })
            .unwrap();

        let (path, _cost) = pathfinding::directed::astar::astar(
            &start,
            |p| {
                p.four_adjacent_iter()
                    .filter(|n| map.get(*n).map(|n| *n != Cell::Wall).unwrap_or(false))
                    .map(|n| (n, 1))
            },
            |_| 0,
            |p| map[*p] == Cell::End,
        )
        .unwrap();

        let path: HashMap<_, _> = path
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p, i as i32))
            .collect();

        let res = path
            .iter()
            .map(|(p, step)| {
                map.four_adjacent_pos_iter(*p)
                    .filter(|n| {
                        let vec = *n - *p;
                        map[*n] == Cell::Wall
                            && path
                                .get(&(*n + vec))
                                .map(|s| s - *step > 100)
                                .unwrap_or(false)
                    })
                    .count()
            })
            .sum::<usize>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let map = parser().parse(input).unwrap();

        let start = map
            .iter()
            .find_map(|(v, s)| if let Cell::Start = s { Some(v) } else { None })
            .unwrap();

        let (path, _cost) = pathfinding::directed::astar::astar(
            &start,
            |p| {
                p.four_adjacent_iter()
                    .filter(|n| map.get(*n).map(|n| *n != Cell::Wall).unwrap_or(false))
                    .map(|n| (n, 1))
            },
            |_| 0,
            |p| map[*p] == Cell::End,
        )
        .unwrap();

        let path: HashMap<_, _> = path
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p, i as i32))
            .collect();

        let res = path
            .keys()
            .map(|p| {
                let mut visited = HashMap::new();
                let mut next = p.four_adjacent_iter().collect_vec();
                let mut step = 1;

                while !next.is_empty() && step != 20 {
                    let mut new_next = vec![];
                    for n in next {
                        if visited.contains_key(&n) {
                            continue;
                        }
                        visited.insert(n, step);
                        if map[n] == Cell::Wall {
                            new_next.extend(map.four_adjacent_pos_iter(n));
                        }
                    }
                    next = new_next;
                    step += 1;
                }

                visited
                    .iter()
                    .filter(|(shortcut_dest, shortcut_cost)| {
                        path.get(shortcut_dest)
                            .inspect(|initial_cost| {
                                if *initial_cost - path[p] - **shortcut_cost > 0 {
                                    println!("from {:?} to {:?}, cost start {}, cost dest {}, cost shortcut {}, save: {}", p, shortcut_dest, path[p], initial_cost, shortcut_cost, *initial_cost - path[p] - **shortcut_cost)
                                }
                            })
                            .map(|initial_cost| *initial_cost - path[p] - **shortcut_cost > 50)
                            .unwrap_or(false)
                    })
                    .count()
            })
            .sum::<usize>();

        PartResult::new(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Cell::Empty => ".",
            Cell::Start => "S",
            Cell::End => "E",
            Cell::Wall => "#",
        })
    }
}

fn parser() -> impl Parser<char, Map<Cell>, Error = Simple<char>> {
    let cell = choice([
        just(".").to(Cell::Empty),
        just("#").to(Cell::Wall),
        just("S").to(Cell::Start),
        just("E").to(Cell::End),
    ]);

    let line = cell.repeated().at_least(1);

    line.separated_by(text::newline())
        .map(Map::new)
        .then_ignore(end())
}
