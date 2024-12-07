use common::prelude::*;

use chumsky::prelude::*;
use rayon::prelude::*;

register_solver!(2024, 7, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let equations = parser().parse(input).unwrap();

        let res = equations
            .par_iter()
            .filter(|e| {
                generate_all_variants_iter([Operator::Add, Operator::Multiply], e.numbers.len() - 1)
                    .any(|ops| apply(&e.numbers, &ops) == e.result)
            })
            .map(|e| e.result)
            .sum::<i64>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let equations = parser().parse(input).unwrap();

        let res = equations
            .par_iter()
            .filter(|e| {
                generate_all_variants_iter(
                    [Operator::Add, Operator::Multiply, Operator::Concat],
                    e.numbers.len() - 1,
                )
                .any(|ops| apply(&e.numbers, &ops) == e.result)
            })
            .map(|e| e.result)
            .sum::<i64>();

        PartResult::new(res)
    }
}

fn apply(numbers: &[i64], operators: &[Operator]) -> i64 {
    assert!(numbers.len() == operators.len() + 1);
    let mut ops = operators.iter();

    numbers
        .iter()
        .copied()
        .reduce(|acc, n| {
            let op = ops.next().unwrap();
            op.apply(acc, n)
        })
        .unwrap_or(0)
}

fn generate_all_variants_iter<const N: usize>(
    variants: [Operator; N],
    length: usize,
) -> impl Iterator<Item = Vec<Operator>> {
    let num_variants = variants.len();

    (0..num_variants.pow(length as u32)).map(move |index| {
        let mut combination = Vec::with_capacity(length);
        let mut idx = index;

        for _ in 0..length {
            combination.push(variants[idx % num_variants]);
            idx /= num_variants;
        }

        combination
    })
}

fn num_digits_in_base(n: i64, base: u64) -> u32 {
    if n == 0 {
        return 1; // Special case: 0 has exactly 1 digit in any base.
    }
    // Compute the number of digits using the formula: floor(log_b(n)) + 1
    (n as f64).log(base as f64).floor() as u32 + 1
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}
impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => a * 10_i64.pow(num_digits_in_base(b, 10)) as i64 + b,
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

fn parser() -> impl Parser<char, Vec<Equation>, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<i64>().unwrap());

    let equation = number
        .then_ignore(just(":"))
        .then(number.separated_by(just(" ")).allow_leading())
        .map(|(result, numbers)| Equation { result, numbers });

    equation.separated_by(text::newline())
}
