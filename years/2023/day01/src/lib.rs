use common::DayResult;

pub struct Solver;

const MAPPING: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        solve(input, first_char_to_digit)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        solve(input, |s| {
            if let Some(digit) = first_char_to_digit(s) {
                return Some(digit);
            } else {
                for (pat, val) in &MAPPING {
                    if s.starts_with(pat) {
                        return Some(*val);
                    }
                }
            }
            None
        })
    }
}

fn first_char_to_digit(s: &str) -> Option<u32> {
    let next = s.chars().next()?;
    if next.is_numeric() {
        Some(next.to_digit(10).expect("to be convertible"))
    } else {
        None
    }
}

fn solve(input: &str, filter: impl Fn(&str) -> Option<u32>) -> DayResult {
    let res: u32 = input
        .split_whitespace()
        .map(|line| {
            let mut first = None;
            let mut last = None;

            // Find first
            for i in 0..line.len() {
                let l = &line[i..];
                if let Some(f) = filter(l) {
                    first = Some(f);
                    break;
                }
            }

            // Find last
            for i in (0..line.len()).rev() {
                let l = &line[i..];
                if let Some(f) = filter(l) {
                    last = Some(f);
                    break;
                }
            }

            first.unwrap() * 10 + last.unwrap()
        })
        .sum();

    DayResult::new(res)
}
