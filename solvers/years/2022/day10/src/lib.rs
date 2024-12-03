use common::prelude::*;

register_solver!(2022, 10, Solver);
pub struct Solver;

enum Op {
    Noop,
    Addx(i32),
}

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let _ = input;
        let ops = input.split('\n').map(|l| {
            let mut it = l.split(' ');
            match it.next().unwrap() {
                "noop" => Op::Noop,
                "addx" => Op::Addx(it.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        });

        let mut cycle = 1;
        let mut sum = 0;
        let mut screen = String::with_capacity(41 * 6);
        let mut next_cycle = |x_reg: &i32| {
            if (x_reg - 1..=x_reg + 1).contains(&((cycle - 1) % 40)) {
                screen.push('█')
            } else {
                screen.push(' ')
            }
            if cycle % 40 == 0 {
                screen.push('\n')
            }
            if (cycle - 20) % 40 == 0 {
                sum += cycle * *x_reg;
            }
            cycle += 1;
        };

        let mut x_reg = 1;
        for op in ops {
            match op {
                Op::Noop => next_cycle(&x_reg),
                Op::Addx(val) => {
                    next_cycle(&x_reg);
                    next_cycle(&x_reg);
                    x_reg += val;
                }
            };
        }

        (PartResult::new(sum), PartResult::new(screen.trim()))
    }
}
