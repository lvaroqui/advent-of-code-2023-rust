use common::prelude::*;

register_solver!(2024, 1, Solver);

pub struct Solver;

impl common::MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (DayResult, DayResult) {
        let (mut list_a, mut list_b): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|l| {
                let mut line = l.split_whitespace();
                (
                    line.next().unwrap().parse::<i32>().unwrap(),
                    line.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .unzip();

        list_a.sort();
        list_b.sort();

        let res1 = list_a
            .iter()
            .zip(list_b.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>();

        let res2 = list_a
            .iter()
            .map(|a| a * list_b.iter().filter(|b| **b == *a).count() as i32)
            .sum::<i32>();

        (DayResult::new(res1), DayResult::new(res2))
    }
}
