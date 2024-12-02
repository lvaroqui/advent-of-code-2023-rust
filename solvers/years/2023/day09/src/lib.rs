use std::collections::VecDeque;

use common::prelude::*;
use itertools::Itertools;
use std::ops::{Add, Sub};

register_solver!(2023, 9, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        solve(input, VecDeque::back, VecDeque::push_back, i32::add)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        solve(input, VecDeque::front, VecDeque::push_front, i32::sub)
    }
}

fn solve(
    input: &str,
    value_getter: impl Fn(&VecDeque<i32>) -> Option<&i32>,
    value_pusher: impl Fn(&mut VecDeque<i32>, i32),
    op: impl Fn(i32, i32) -> i32,
) -> DayResult {
    let series = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<VecDeque<_>>()
        })
        .collect_vec();

    let mut sum = 0;

    for s in series {
        let mut stack = vec![s];

        while stack.last().unwrap().iter().any(|e| *e != 0) {
            let derivative = stack
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(left, right)| right - left)
                .collect();
            stack.push(derivative)
        }

        while stack.len() > 1 {
            // Remove top of stack and get value for modification of bellow level
            let popped_value = *value_getter(&stack.pop().unwrap()).unwrap();
            let to_modify = stack.last_mut().unwrap();
            let value = *value_getter(to_modify).unwrap();
            value_pusher(to_modify, op(value, popped_value))
        }

        sum += value_getter(stack.last().unwrap()).unwrap();
    }

    DayResult::new(sum)
}
