use common::prelude::*;

use regex::Regex;

register_solver!(2024, 3, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let mul_parser = MulParser::new();

        let mut res = 0;
        for i in 0..input.len() {
            let slice = &input[i..];
            if let Some((a, b)) = mul_parser.parse(slice) {
                res += a * b;
            }
        }

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let mul_parser = MulParser::new();

        let mut enabled = true;
        let mut res = 0;
        for i in 0..input.len() {
            let slice = &input[i..];
            if slice.starts_with("don't()") {
                enabled = false
            } else if slice.starts_with("do()") {
                enabled = true;
            } else if enabled {
                if let Some((a, b)) = mul_parser.parse(slice) {
                    res += a * b;
                }
            }
        }

        PartResult::new(res)
    }
}

struct MulParser(Regex);

impl MulParser {
    fn new() -> Self {
        Self(Regex::new(r"^mul\(([0-9]+),([0-9]+)\)").unwrap())
    }

    fn parse(&self, input: &str) -> Option<(i32, i32)> {
        let captures = self.0.captures(input);
        captures.map(|c| (c[1].parse().unwrap(), c[2].parse().unwrap()))
    }
}
