use common::prelude::*;

use chumsky::prelude::*;

register_solver!(2024, 2, Solver);
pub struct Solver;

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let input = parser().parse(input).unwrap();

        let res1 = input.iter().filter(|line| check(line, false)).count();
        let res2 = input.iter().filter(|line| check(line, true)).count();

        (PartResult::new(res1), PartResult::new(res2))
    }
}

fn parser() -> impl Parser<char, Vec<Vec<i32>>, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<i32>().unwrap());

    let line = number.separated_by(just(" "));

    line.separated_by(text::newline())
}

fn check(line: &[i32], problem_dampener: bool) -> bool {
    if problem_dampener {
        is_valid(line) || is_valid_with_one_removed(line)
    } else {
        is_valid(line)
    }
}

fn is_valid(line: &[i32]) -> bool {
    if line.len() <= 1 {
        return true;
    }

    let range = if line[0] < line[1] { 1..=3 } else { -3..=-1 };

    for (a, b) in line.iter().zip(line.iter().skip(1)) {
        if !range.contains(&(b - a)) {
            return false;
        }
    }
    true
}

fn is_valid_with_one_removed(line: &[i32]) -> bool {
    (0..line.len()).any(|index_to_remove| {
        let mut l = line.to_owned();
        l.remove(index_to_remove);
        is_valid(&l)
    })
}
