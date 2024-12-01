use std::collections::HashSet;

use common::DayResult;

pub struct Solver;

const START: (i32, i32) = (500, 0);

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let mut map = parse(input);

        let max_y = map.iter().map(|(_x, y)| y).max().unwrap() + 1;
        let mut sand_units = 0;

        'l: loop {
            let mut cur_pos = START;

            while let Some(next_pos) = sand_next_pos(&map, cur_pos, None) {
                cur_pos = next_pos;
                if cur_pos.1 == max_y {
                    break 'l;
                }
            }
            map.insert(cur_pos);
            sand_units += 1;
        }

        DayResult::new(sand_units)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let mut map = parse(input);

        let floor = map.iter().map(|(_x, y)| y).max().unwrap() + 2;

        let mut sand_units = 0;

        loop {
            let mut cur_pos = START;

            while let Some(next_pos) = sand_next_pos(&map, cur_pos, Some(floor)) {
                cur_pos = next_pos;
            }
            map.insert(cur_pos);
            sand_units += 1;
            if cur_pos == START {
                break;
            }
        }

        DayResult::new(sand_units)
    }
}

fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut map = HashSet::new();
    for lines in input.split('\n').map(|l| {
        l.split(" -> ")
            .map(|coords| {
                let mut it = coords.split(',');
                (
                    it.next().unwrap().parse::<i32>().unwrap(),
                    it.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    }) {
        for l in lines.windows(2) {
            let (xa, ya) = l[0];
            let (xb, yb) = l[1];
            if xa == xb {
                for y in num::range_inclusive(ya, yb) {
                    map.insert((xa, y));
                }
            } else {
                for x in num::range_inclusive(xa, xb) {
                    map.insert((x, ya));
                }
            }
        }
    }
    map
}

fn sand_next_pos(
    map: &HashSet<(i32, i32)>,
    (x, y): (i32, i32),
    floor: Option<i32>,
) -> Option<(i32, i32)> {
    if let Some(floor) = floor {
        if y == floor - 1 {
            return None;
        }
    }
    let tests = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
    tests.into_iter().find(|t| map.get(t).is_none())
}
