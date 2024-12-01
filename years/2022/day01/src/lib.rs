use common::DayResult;
use itertools::Itertools;

pub struct Solver;

impl common::MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (DayResult, DayResult) {
        let iter = input
            .split("\n\n")
            .map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>());

        (
            DayResult::new(iter.clone().max().unwrap()),
            DayResult::new(iter.sorted_unstable().rev().take(3).sum::<u32>()),
        )
    }
}
