use common::prelude::*;

use chumsky::prelude::*;
use rayon::prelude::*;

register_solver!(2024, 7, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let equations = parser().parse(input).unwrap();

        let res = equations
            .iter()
            .filter(|e| e.check(&[Operator::Add, Operator::Multiply]))
            .map(|e| e.result)
            .sum::<i64>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let equations = parser().parse(input).unwrap();

        let res = equations
            .iter()
            .filter(|e| e.check(&[Operator::Add, Operator::Multiply, Operator::Concat]))
            .map(|e| e.result)
            .sum::<i64>();

        PartResult::new(res)
    }
}

// register_solver!(2024, 7, SolverIterativeBruteForce);
pub struct SolverIterativeBruteForce;

impl DualDaySolver for SolverIterativeBruteForce {
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
    debug_assert!(numbers.len() == operators.len() + 1);
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

fn num_digits_in_base_10(n: i64) -> u32 {
    if n == 0 {
        // Special case: 0 has exactly 1 digit in any base.
        1
    } else {
        n.ilog10() + 1
    }
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
            Operator::Concat => a * 10_i64.pow(num_digits_in_base_10(b)) as i64 + b,
        }
    }

    fn try_unapply(&self, from: i64, number: i64) -> Option<i64> {
        match self {
            Operator::Add => (from > number).then(|| from - number),
            Operator::Multiply => (from % number == 0).then(|| from / number),
            Operator::Concat => {
                let ten_power = 10_i64.pow(num_digits_in_base_10(number));
                if (from - number) % ten_power == 0 {
                    Some(from / ten_power)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn check(&self, operators: &[Operator]) -> bool {
        fn check_impl(
            operators: &[Operator],
            current: i64,
            numbers: &[i64],
            operator: Operator,
        ) -> bool {
            if numbers.len() == 1 {
                return current == numbers[0];
            }

            let (last, numbers) = numbers.split_last().unwrap();

            let Some(current) = operator.try_unapply(current, *last) else {
                return false;
            };

            operators
                .iter()
                .any(|op| check_impl(operators, current, numbers, *op))
        }

        operators
            .iter()
            .any(|op| check_impl(operators, self.result, &self.numbers, *op))
    }
}

fn parser() -> impl Parser<char, Vec<Equation>, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<i64>().unwrap());

    let equation = number
        .then_ignore(just(":"))
        .then(number.separated_by(just(" ")).allow_leading())
        .map(|(result, numbers)| Equation { result, numbers });

    equation.separated_by(text::newline())
}
