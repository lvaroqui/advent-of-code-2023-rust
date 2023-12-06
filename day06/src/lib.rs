use chumsky::prelude::*;

use common::DayResult;

pub struct Solver;

#[derive(Debug)]
struct Run {
    time: f64,
    distance: f64,
}

impl Run {
    fn number_of_way_to_win(&self) -> u64 {
        // To find at which time we beat the current record, we're going to
        // solve the equation:
        //      `distance = v_start * t_moving`
        // <=>  `distance = t_pressed * (t_run - t_pressed)`
        // <=>  `-t_pressed² + t_pressed * t_run - distance = 0`
        // Which forms a quadratic function in which t_pressed is the unknown.

        // Solve the roots of this function to know in what t_pressed time our
        // boat would win.
        let a = -1.0;
        let b = self.time;
        let c = -self.distance;
        let delta = (b * b) - 4.0 * a * c;
        assert!(
            delta >= 0.0,
            "Assume we have a lower and an upper bound to the result"
        );
        let root1 = (-b + delta.sqrt()) / (2.0 * a);
        let root2 = (-b - delta.sqrt()) / (2.0 * a);

        // Find first and last time for which we would have won.
        //
        // Use "+ 1.0 and floor()" instead of ceil() to exclude an exact integer
        // (eg. if the exact record would be reach by a 20ms long press, we need to press for at least 21ms).
        let from = (root1 + 1.0).floor() as u64;
        // Same as above but reverse for end of range.
        let to = (root2 - 1.0).ceil() as u64;

        to - from + 1
    }
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let runs = parser_1().parse(input).unwrap();
        let res = runs.iter().map(Run::number_of_way_to_win).product::<u64>();

        DayResult::new(res)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let run = parser_2().parse(input).unwrap();
        DayResult::new(run.number_of_way_to_win())
    }
}

fn parser_1() -> impl Parser<char, Vec<Run>, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<f64>().unwrap());

    just("Time:")
        .padded()
        .ignore_then(number.separated_by(text::whitespace()))
        .then_ignore(text::newline())
        .then(
            just("Distance:")
                .padded()
                .ignore_then(number.separated_by(text::whitespace())),
        )
        .map(|(time, distance)| {
            time.into_iter()
                .zip(distance)
                .map(|(time, distance)| Run { time, distance })
                .collect()
        })
}

fn parser_2() -> impl Parser<char, Run, Error = Simple<char>> {
    // Parse space separated number as one big number.
    let number = text::int(10)
        .separated_by(text::whitespace())
        .map(|s: Vec<String>| s.into_iter().collect::<String>().parse::<f64>().unwrap());

    just("Time:")
        .padded()
        .ignore_then(number)
        .then_ignore(text::newline())
        .then(just("Distance:").padded().ignore_then(number))
        .map(|(time, distance)| Run { time, distance })
}
