use std::collections::HashSet;

register_solver!(2022, 9, Solver);
use common::prelude::*;

pub struct Solver;

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (DayResult, DayResult) {
        let mut rope = [(0, 0); 10];
        let mut visited_first = HashSet::new();
        let mut visited_second = HashSet::new();
        for l in input.split('\n') {
            let mut l = l.split(' ');
            let direction = l.next().unwrap();
            let distance = l.next().unwrap().parse::<i32>().unwrap();
            for _ in 0..distance {
                match direction {
                    "R" => rope[0].0 += 1,
                    "L" => rope[0].0 -= 1,
                    "U" => rope[0].1 += 1,
                    "D" => rope[0].1 -= 1,
                    _ => unreachable!(),
                }

                for i in 0..rope.len() - 1 {
                    let (head_x, head_y) = rope[i];
                    let (tail_x, tail_y) = &mut rope[i + 1];
                    let d_x: i32 = head_x - *tail_x;
                    let d_y: i32 = head_y - *tail_y;

                    let adjacent = !(d_x.abs() > 1 || d_y.abs() > 1) as i32;

                    if *tail_x < head_x - adjacent {
                        *tail_x += 1;
                    }
                    if *tail_x > head_x + adjacent {
                        *tail_x -= 1;
                    }
                    if *tail_y < head_y - adjacent {
                        *tail_y += 1;
                    }
                    if *tail_y > head_y + adjacent {
                        *tail_y -= 1;
                    }
                }

                visited_first.insert((rope[1].0, rope[1].1));
                visited_second.insert((rope.last().unwrap().0, rope.last().unwrap().1));
            }
        }

        (
            DayResult::new(visited_first.len()),
            DayResult::new(visited_second.len()),
        )
    }
}
