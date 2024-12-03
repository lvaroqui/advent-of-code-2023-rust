use common::prelude::*;
use rayon::prelude::*;
use take_until::TakeUntilExt;

register_solver!(2022, 8, Solver);
pub struct Solver;

type Map = Vec<Vec<u32>>;

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let map: Map = input
            .split('\n')
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let visible_trees = tree_iter(&map)
            .filter(|(current, (x, y))| {
                line_col_iter(&map, (*x, *y)).any(|mut line| line.all(|h| h < *current))
            })
            .count();

        let max_scenic_score = tree_iter(&map)
            .map(|(current, (x, y))| {
                line_col_iter(&map, (x, y))
                    .map(|line| line.take_until(|h| *h >= current).count())
                    .product::<usize>()
            })
            .max()
            .unwrap();

        (
            PartResult::new(visible_trees),
            PartResult::new(max_scenic_score),
        )
    }
}

fn tree_iter(map: &Map) -> impl ParallelIterator<Item = (u32, (usize, usize))> + '_ {
    map.par_iter()
        .enumerate()
        .flat_map(|(y, l)| l.par_iter().enumerate().map(move |(x, v)| (*v, (x, y))))
}

fn line_col_iter<'a>(
    map: &'a Map,
    (x, y): (usize, usize),
) -> impl Iterator<Item = impl Iterator<Item = u32> + 'a> {
    let up = Box::new((0..y).rev().map(move |i| map[i][x]));
    let down = Box::new((y + 1..map.len()).map(move |i| map[i][x]));
    let left = Box::new((0..x).rev().map(move |i| map[y][i]));
    let right = Box::new((x + 1..map[0].len()).map(move |i| map[y][i]));

    let a: [Box<dyn Iterator<Item = u32> + Send + 'a>; 4] = [up, down, left, right];
    a.into_iter()
}
