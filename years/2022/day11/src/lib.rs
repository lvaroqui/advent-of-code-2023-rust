use std::{cell::RefCell, collections::VecDeque};

use common::DayResult;

pub struct Solver;

struct Monkey {
    items: VecDeque<i32>,
    op: Box<dyn Fn(i32) -> i32>,
    divider_test: i32,
    if_true: usize,
    if_false: usize,
    inspection_count: usize,
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        solve(input, true)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        solve(input, false)
    }
}

fn solve(input: &str, part1: bool) -> DayResult {
    let mut monkeys = parse(input);

    let n = monkeys
        .iter()
        .map(|m| m.borrow().divider_test)
        .product::<i32>();

    let iterations = if part1 { 20 } else { 10000 };

    for _round in 0..iterations {
        for m in &monkeys {
            let mut m = m.borrow_mut();
            let m = &mut *m;
            while let Some(mut i) = m.items.pop_front() {
                m.inspection_count += 1;
                i = (m.op)(i);
                if part1 {
                    i /= 3;
                }
                i %= n;
                let target = if i % m.divider_test == 0 {
                    m.if_true
                } else {
                    m.if_false
                };
                monkeys[target].borrow_mut().items.push_back(i)
            }
        }
    }
    monkeys.sort_by_key(|m| m.borrow().inspection_count);
    DayResult::new(
        monkeys
            .iter()
            .map(|m| m.borrow().inspection_count)
            .rev()
            .take(2)
            .product::<usize>(),
    )
}

fn parse(input: &str) -> Vec<RefCell<Monkey>> {
    input
        .split("\n\n")
        .map(|m| {
            let mut l = m.split('\n').skip(1).map(|l| l.trim());
            Monkey {
                items: l
                    .next()
                    .unwrap()
                    .trim_start_matches("Starting items: ")
                    .split(',')
                    .map(|i| i.trim().parse().unwrap())
                    .collect(),
                op: {
                    let mut e = l
                        .next()
                        .unwrap()
                        .trim_start_matches("Operation: new = old ")
                        .split(' ');
                    let op = e.next().unwrap();
                    let val = e.next().unwrap().parse::<i32>();
                    if let Ok(val) = val {
                        match op {
                            "+" => Box::new(move |i| i + val),
                            "*" => Box::new(move |i| i * val),
                            _ => unreachable!(),
                        }
                    } else {
                        match op {
                            "+" => Box::new(move |i| i + i),
                            "*" => Box::new(move |i| i * i),
                            _ => unreachable!(),
                        }
                    }
                },
                divider_test: l
                    .next()
                    .unwrap()
                    .trim_start_matches("Test: divisible by ")
                    .parse()
                    .unwrap(),
                if_true: l
                    .next()
                    .unwrap()
                    .trim_start_matches("If true: throw to monkey ")
                    .parse()
                    .unwrap(),
                if_false: l
                    .next()
                    .unwrap()
                    .trim_start_matches("If false: throw to monkey ")
                    .parse()
                    .unwrap(),
                inspection_count: 0,
            }
        })
        .map(RefCell::new)
        .collect()
}
