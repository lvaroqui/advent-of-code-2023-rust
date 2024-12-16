use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, PI},
};

use common::{
    map::{Map, Vec2},
    prelude::*,
};

use chumsky::prelude::*;
use petgraph::{algo::dijkstra, prelude::*};

register_solver!(2024, 16, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let map = parser().parse(input).unwrap();

        let mut graph: GraphMap<(Vec2, Vec2), i64, Undirected> = GraphMap::new();

        for (p, _) in map.iter().filter(|(_, c)| c.is_path()) {
            for d in [Vec2::NORTH, Vec2::EAST, Vec2::SOUTH, Vec2::WEST] {
                graph.add_node((p, d));
            }
        }
        let mut edges = vec![];
        for (p, d) in graph.nodes() {
            if map.get(p + d).map(|c| c.is_path()).unwrap_or(false) {
                edges.push(((p, d), (p + d, d), 1));
            }
            edges.push(((p, d), (p, d.rotate(FRAC_PI_2)), 1000));
        }

        for (from, to, weight) in edges {
            graph.add_edge(from, to, weight);
        }

        let start_pos = map.iter().find(|(_, c)| **c == Cell::Start).unwrap().0;
        let end_pos = map.iter().find(|(_, c)| **c == Cell::End).unwrap().0;

        let res = dijkstra(
            &graph,
            (start_pos, Vec2::EAST),
            Some((end_pos, Vec2::NORTH)),
            |(_from, _to, weight)| *weight,
        );

        let res = res
            .iter()
            .filter(|((p, _), _)| *p == end_pos)
            .map(|(_, cost)| cost)
            .min()
            .unwrap();

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
impl Cell {
    fn is_path(&self) -> bool {
        matches!(self, Cell::Empty | Cell::Start | Cell::End)
    }
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

    line.separated_by(text::newline()).map(Map::new)
}
