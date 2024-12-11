use common::{math_helper::num_digits_in_base_10, prelude::*};

register_solver!(2024, 11, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        PartResult::new(solve(input, 25))
    }

    fn solve_2(&self, input: &str) -> PartResult {
        PartResult::new(solve(input, 75))
    }
}

fn solve(input: &str, iteration_count: u32) -> u64 {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .map(|n| compute(iteration_count, n))
        .sum::<u64>()
}

#[cached::proc_macro::cached]
fn compute(iteration: u32, value: u64) -> u64 {
    if iteration == 0 {
        return 1;
    }

    let next = iteration - 1;

    let num_digits = num_digits_in_base_10(value as i64);
    if value == 0 {
        compute(next, 1)
    } else if num_digits % 2 == 0 {
        let ten_power = 10_u64.pow(num_digits / 2);
        let left_part = value / ten_power;
        let right_part = value - (left_part * ten_power);
        compute(next, left_part) + compute(next, right_part)
    } else {
        compute(next, value * 2024)
    }
}
