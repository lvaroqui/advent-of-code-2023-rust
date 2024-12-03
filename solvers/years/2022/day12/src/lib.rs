use common::prelude::*;
use petgraph::{algo::dijkstra, graphmap::GraphMap, visit::Reversed, Directed};

register_solver!(2022, 12, Solver);
pub struct Solver;

type Map = Vec<Vec<u8>>;

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let mut start = None;
        let mut end = None;
        let map: Map = input
            .split('\n')
            .enumerate()
            .map(|(y, l)| {
                l.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(x, e)| {
                        let e = match *e {
                            b'S' => {
                                start = Some((x, y));
                                b'a'
                            }
                            b'E' => {
                                end = Some((x, y));
                                b'z'
                            }
                            e => e,
                        };
                        e - b'a'
                    })
                    .collect()
            })
            .collect();
        let (start, end) = (start.unwrap(), end.unwrap());

        let mut graph = GraphMap::<(usize, usize), (), Directed>::new();
        for (h, node) in tree_iter(&map) {
            for (other_h, other_node) in around_iter(&map, node) {
                if h + 1 >= other_h {
                    graph.add_edge(node, other_node, ());
                }
            }
        }

        let res1 = dijkstra(&graph, start, Some(end), |_| 1);
        let res1 = res1.get(&end).unwrap();

        let res2 = dijkstra(Reversed(&graph), end, None, |_| 1);
        let res2 = res2
            .iter()
            .filter_map(
                |((x, y), score)| {
                    if map[*y][*x] == 0 {
                        Some(score)
                    } else {
                        None
                    }
                },
            )
            .min()
            .unwrap();

        (PartResult::new(res1), PartResult::new(res2))
    }
}

fn tree_iter(map: &Map) -> impl Iterator<Item = (u8, (usize, usize))> + '_ {
    map.iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, v)| (*v, (x, y))))
}

fn around_iter(map: &Map, (x, y): (usize, usize)) -> impl Iterator<Item = (u8, (usize, usize))> {
    let mut res = Vec::with_capacity(4);
    if x > 0 {
        res.push((map[y][x - 1], (x - 1, y)));
    }
    if x < map[0].len() - 1 {
        res.push((map[y][x + 1], (x + 1, y)));
    }
    if y > 0 {
        res.push((map[y - 1][x], (x, y - 1)));
    }
    if y < map.len() - 1 {
        res.push((map[y + 1][x], (x, y + 1)));
    }
    res.into_iter()
}
