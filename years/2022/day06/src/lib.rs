use common::DayResult;
use itertools::Itertools;

pub struct Solver;

impl common::DualDaySolver for Solver {
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
