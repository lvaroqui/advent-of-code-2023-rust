use std::collections::HashMap;

use common::prelude::*;

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

fn solve(input: &str, iterations: i32) -> u64 {
    let input: Vec<u64> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();

    let res = input
        .iter()
        .copied()
        .map(|n| compute(&mut cache, n, iterations))
        .sum::<u64>();
    res
}

fn num_digits_in_base_10(n: u64) -> u32 {
    if n == 0 {
        // Special case: 0 has exactly 1 digit in any base.
        1
    } else {
        n.ilog10() + 1
    }
}

fn compute(cache: &mut HashMap<(i32, u64), u64>, n: u64, iteration: i32) -> u64 {
    if iteration == 0 {
        return 1;
    }

    if let Some(res) = cache.get(&(iteration, n)) {
        return *res;
    }

    let update_cache = |cache: &mut HashMap<(i32, u64), u64>, n: u64, iteration: i32| {
        let res = compute(cache, n, iteration);
        cache.insert((iteration, n), res);
        res
    };

    let next = iteration - 1;

    let num_digits = num_digits_in_base_10(n);
    if n == 0 {
        update_cache(cache, 1, next)
    } else if num_digits % 2 == 0 {
        let ten_power = 10_u64.pow(num_digits / 2);
        let left_part = n / ten_power;
        let right_part = n - (left_part * ten_power);
        update_cache(cache, left_part, next) + update_cache(cache, right_part, next)
    } else {
        update_cache(cache, n * 2024, next)
    }
}
