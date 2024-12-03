use std::collections::{HashMap, HashSet};

use common::{
    map::{vec2, Map},
    prelude::*,
};
use itertools::Itertools;

register_solver!(2023, 3, Solver);

pub struct Solver;

enum Element {
    Digit(u8),
    Gear(u32),
    Part,
    None,
}

#[derive(Default)]
struct Number<Metadata> {
    value: u32,
    digit_index: u32,
    metadata: Metadata,
}

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        solve::<bool>(
            input,
            |has_neighbor, neighbor| {
                // Record if the current number has at list one adjacent part
                // number
                *has_neighbor =
                    *has_neighbor || matches!(neighbor, Element::Part | Element::Gear(_));
            },
            |numbers| {
                let res = numbers
                    .iter()
                    .filter(|v| v.metadata)
                    .map(|v| v.value)
                    .sum::<u32>();

                PartResult::new(res)
            },
        )
    }

    fn solve_2(&self, input: &str) -> PartResult {
        solve::<HashSet<u32>>(
            input,
            |gear_ids, neighbor| {
                // Record all gears adjacent to numbers
                if let Element::Gear(id) = neighbor {
                    gear_ids.insert(*id);
                }
            },
            |numbers| {
                // Construct map of gears with adjacent numbers
                let mut gears = HashMap::new();
                for n in numbers {
                    for gear_id in &n.metadata {
                        let gear = gears.entry(gear_id).or_insert_with(std::vec::Vec::new);
                        gear.push(n.value);
                    }
                }

                // Find gears adjacent to exactly 2 numbers, compute the sum of
                // theirs individual powers
                let res = gears
                    .values()
                    .filter(|v| v.len() == 2)
                    .map(|v| v.iter().product::<u32>())
                    .sum::<u32>();

                PartResult::new(res)
            },
        )
    }
}

fn solve<NumberMetadata: Default>(
    input: &str,
    inspect_number_neighbor: impl Fn(&mut NumberMetadata, &Element),
    compute_result: impl Fn(&Vec<Number<NumberMetadata>>) -> PartResult,
) -> PartResult {
    // Parse map
    let lines = input.split('\n');
    let mut gear_id = 0;
    let map = Map::new(
        lines
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        c if c.is_numeric() => Element::Digit(c.to_digit(10).unwrap() as u8),
                        '.' => Element::None,
                        '*' => {
                            gear_id += 1;
                            Element::Gear(gear_id)
                        }
                        _part => Element::Part,
                    })
                    .collect_vec()
            })
            .collect_vec(),
    );

    let mut numbers = vec![];

    for (y, line) in map.inner().iter().enumerate() {
        let mut current_number: Option<Number<NumberMetadata>> = None;

        // Iterating in reverse allows easy parsing of the numbers (by reading
        // them from right to left and adding `digit * 10**digit_index` to the
        // number)
        let mut it = line.iter().enumerate().rev().peekable();
        while let Some((x, e)) = it.next() {
            // Current number must be consumed if:
            //  1. The current element is not a digit
            //  2. There is no more element to parse
            let mut consume_current_number = it.peek().is_none();

            if let Element::Digit(digit) = e {
                let c = current_number.get_or_insert(Number::default());
                c.value += 10_u32.pow(c.digit_index) * *digit as u32;
                c.digit_index += 1;

                for a in map.adjacent_iter(vec2(x as i64, y as i64)) {
                    inspect_number_neighbor(&mut c.metadata, a);
                }
            } else {
                consume_current_number = true;
            }

            if consume_current_number {
                if let Some(number) = current_number.take() {
                    numbers.push(number)
                }
            }
        }
    }

    PartResult::new(compute_result(&numbers))
}
