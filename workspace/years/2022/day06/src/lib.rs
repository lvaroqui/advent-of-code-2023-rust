use common::prelude::*;
use itertools::Itertools;

register_solver!(2022, 6, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        DayResult::new(find_unique_sequence(input, 4).unwrap())
    }

    fn solve_2(&self, input: &str) -> DayResult {
        DayResult::new(find_unique_sequence(input, 14).unwrap())
    }
}

fn find_unique_sequence(input: &str, nb: usize) -> Option<usize> {
    for (i, w) in input.as_bytes().windows(nb).enumerate() {
        if w.iter().unique().count() == nb {
            return Some(i + nb);
        }
    }
    None
}
