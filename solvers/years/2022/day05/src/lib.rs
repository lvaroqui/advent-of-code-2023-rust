use common::prelude::*;

register_solver!(2022, 5, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        solve(input, |stacks, from, to, amount| {
            let i = stacks[from].len();
            let mut tmp = stacks[from].split_off(i - amount);
            tmp.reverse();
            stacks[to].extend_from_slice(&tmp);
        })
    }

    fn solve_2(&self, input: &str) -> DayResult {
        solve(input, |stacks, from, to, amount| {
            let i = stacks[from].len();
            let tmp = stacks[from].split_off(i - amount);
            stacks[to].extend_from_slice(&tmp);
        })
    }
}

fn solve(input: &str, stack_func: impl Fn(&mut Vec<Vec<u8>>, usize, usize, usize)) -> DayResult {
    let (layout, moves) = {
        let mut it = input.split("\n\n");
        (it.next().unwrap(), it.next().unwrap())
    };

    let layout: Vec<_> = layout.split('\n').collect();
    let stack_count = layout
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks = vec![vec![]; stack_count];

    for &l in layout.iter().rev().skip(1) {
        let mut it = l.as_bytes().iter();
        let mut i = 0;
        while let Some(&c) = it.nth(1) {
            if c != b' ' {
                stacks[i].push(c);
            }
            i += 1;
            it.nth(1);
        }
    }

    for (amount, from, to) in moves.split('\n').map(|l| {
        let mut it = l.split(' ');
        (
            it.nth(1).unwrap().parse::<usize>().unwrap(),
            it.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            it.nth(1).unwrap().parse::<usize>().unwrap() - 1,
        )
    }) {
        stack_func(&mut stacks, from, to, amount);
    }

    let res: String = stacks.iter().map(|s| *s.last().unwrap() as char).collect();
    DayResult::new(res)
}
