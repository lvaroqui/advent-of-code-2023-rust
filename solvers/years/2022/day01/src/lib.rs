use common::prelude::*;
use itertools::Itertools;

register_solver!(2022, 1, Solver);
pub struct Solver;

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let iter = input
            .split("\n\n")
            .map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>());

        (
            PartResult::new(iter.clone().max().unwrap()),
            PartResult::new(iter.sorted_unstable().rev().take(3).sum::<u32>()),
        )
    }
}
