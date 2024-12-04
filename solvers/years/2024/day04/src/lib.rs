use common::{
    map::{Map, Vec2},
    prelude::*,
};

use itertools::Itertools;

register_solver!(2024, 4, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let map = parse(input);

        let res = map
            .iter()
            .map(|(pos, _c)| {
                let map = &map;
                Vec2::directions()
                    .into_iter()
                    .filter(move |d| {
                        itertools::equal(
                            map.iter_from_point(pos, *d).map(|(_p, &c)| c).take(4),
                            "XMAS".chars(),
                        )
                    })
                    .count()
            })
            .sum::<usize>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let map = &parse(input);

        let res = map
            .iter()
            .filter(|(pos, &c)| {
                if c != 'A' {
                    return false;
                }

                let pos = *pos;
                let get_point = |offset| map.get(pos + offset).copied().unwrap_or('.');

                let mut slash_points = [get_point(Vec2::SOUTH_WEST), get_point(Vec2::NORTH_EAST)];
                slash_points.sort();
                let mut antislash_points =
                    [get_point(Vec2::NORTH_WEST), get_point(Vec2::SOUTH_EAST)];
                antislash_points.sort();

                const MS: [char; 2] = ['M', 'S'];
                slash_points == MS && antislash_points == MS
            })
            .count();

        PartResult::new(res)
    }
}

fn parse(input: &str) -> Map<char> {
    Map::new(input.lines().map(|l| l.chars().collect_vec()).collect_vec())
}
