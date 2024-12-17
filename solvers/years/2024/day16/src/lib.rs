use std::{collections::HashSet, f32::consts::FRAC_PI_2};

use common::{
    map::{Map, Vec2},
    prelude::*,
};

use chumsky::prelude::*;
use itertools::Itertools;
use pathfinding::directed::astar::astar_bag;
use petgraph::{algo::astar, prelude::*};

register_solver!(2024, 16, Solver);
pub struct Solver;

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let map = parser().parse(input).unwrap();

        let start_pos = map.iter().find(|(_, c)| **c == Cell::Start).unwrap().0;
        let end_pos = map.iter().find(|(_, c)| **c == Cell::End).unwrap().0;

        let (solution, cost) = astar_bag(
            &(start_pos, Vec2::EAST),
            |(p, d)| {
                let p = *p;
                let d = *d;
                let mut edges = tinyvec::ArrayVec::<[_; 3]>::default();
                if map.get(p + d).map(|c| c.is_path()).unwrap_or(false) {
                    edges.push(((p + d, d), 1));
                }
                edges.push(((p, d.rotate(FRAC_PI_2)), 1000));
                edges.push(((p, d.rotate(-FRAC_PI_2)), 1000));
                edges
            },
            |_| 0,
            |n| n.0 == end_pos,
        )
        .unwrap();

        (
            PartResult::new(cost),
            PartResult::new(
                solution
                    .flatten()
                    .map(|(p, _)| p)
                    .collect::<HashSet<_>>()
                    .len(),
            ),
        )
    }
}

register_solver!(2024, 16, SolverPetgraph, "petgraph");
pub struct SolverPetgraph;

impl MonoDaySolver for SolverPetgraph {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let map = parser().parse(input).unwrap();

        let graph = build_graph(&map);

        let start_pos = map.iter().find(|(_, c)| **c == Cell::Start).unwrap().0;
        let end_pos = map.iter().find(|(_, c)| **c == Cell::End).unwrap().0;

        let mut nodes = HashSet::new();
        let mut edges: HashSet<((Vec2, Vec2), (Vec2, Vec2))> = HashSet::new();

        // For part 2, we repeat A* algorithm in a loop.
        //
        // On each iteration, all edges on the shortest path are marked as
        // visited and will be weighted a tiny bit more for next iteration.
        //
        // If another shorter path passing by new nodes exists, we should find
        // it, as its edges won't have the penalty and therefore weight less.
        //
        // If we do not find any new node on an iteration, we found all shorter
        // paths.
        loop {
            let (cost, path) = astar(
                &graph,
                (start_pos, Vec2::EAST),
                |n| n.0 == end_pos,
                |e| {
                    let bias = if edges.contains(&(e.source(), e.target())) {
                        1
                    } else {
                        0
                    };
                    *e.weight() * 1000 + bias
                },
                |_| 0, // Ensure to find shortest path with estimator to 0
            )
            .unwrap();

            edges.extend(path.iter().tuple_windows().map(|(a, b)| (*a, *b)));
            let before = nodes.len();
            nodes.extend(path.iter().map(|(p, _)| *p));
            if nodes.len() == before {
                let biases = path
                    .iter()
                    .tuple_windows()
                    .filter(|(a, b)| edges.contains(&(**a, **b)))
                    .count();
                let path_cost = (cost - biases as i64) / 1000;

                return (PartResult::new(path_cost), PartResult::new(nodes.len()));
            }
        }
    }
}

fn build_graph(map: &Map<Cell>) -> GraphMap<(Vec2, Vec2), i64, Directed> {
    let mut graph: GraphMap<(Vec2, Vec2), i64, Directed> = GraphMap::new();

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
        edges.push(((p, d), (p, d.rotate(-FRAC_PI_2)), 1000));
    }

    for (from, to, weight) in edges {
        graph.add_edge(from, to, weight);
    }
    graph
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

    line.separated_by(text::newline())
        .map(Map::new)
        .then_ignore(end())
}
