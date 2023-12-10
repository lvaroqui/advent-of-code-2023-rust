use std::collections::VecDeque;

use common::DayResult;
use itertools::Itertools;

pub struct Solver;

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let series = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let mut sum = 0;

        for serie in series {
            let mut stack = vec![serie];

            while stack.last().unwrap().iter().any(|e| *e != 0) {
                let mut derivative = vec![];
                for window in stack.last().unwrap().windows(2) {
                    derivative.push(window[1] - window[0])
                }
                stack.push(derivative)
            }

            stack.last_mut().unwrap().push(0);

            while stack.len() > 1 {
                let poped = stack.pop().unwrap();
                let last = stack.last_mut().unwrap();
                let last_value = *last.last().unwrap();
                last.push(last_value + poped.last().unwrap())
            }

            sum += stack.last().unwrap().last().unwrap();
        }

        DayResult::new(sum)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let series = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<VecDeque<_>>()
            })
            .collect_vec();

        let mut sum = 0;

        for serie in series {
            let mut stack = vec![serie];

            while stack.last().unwrap().iter().any(|e| *e != 0) {
                let derivative = stack
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(left, right)| right - left)
                    .collect::<VecDeque<_>>();
                stack.push(derivative)
            }

            stack.last_mut().unwrap().push_front(0);

            while stack.len() > 1 {
                let poped = stack.pop().unwrap();
                let last = stack.last_mut().unwrap();
                let first_value = *last.front().unwrap();
                last.push_front(first_value - poped.front().unwrap())
            }

            sum += stack.last().unwrap().front().unwrap();
        }

        DayResult::new(sum)
    }
}
