use common::{
    map::{Map, Vec2},
    DayResult,
};
use itertools::Itertools;

pub struct Solver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Vertical,
    Horizontal,
    BottomLeft,
    TopLeft,
    TopRight,
    BottomRight,
    Ground,
    Start,
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            '|' => Element::Vertical,
            '-' => Element::Horizontal,
            'L' => Element::BottomLeft,
            'F' => Element::TopLeft,
            '7' => Element::TopRight,
            'J' => Element::BottomRight,
            '.' => Element::Ground,
            'S' => Element::Start,
            _ => unreachable!(),
        }
    }
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let map = Map::new(
            input
                .lines()
                .map(|line| line.chars().map(Element::from).collect_vec())
                .collect_vec(),
        );

        DayResult::new(pipe_iter(&map).count() / 2)
    }
}

fn pipe_iter(map: &Map<Element>) -> impl Iterator<Item = (Vec2, Element)> + '_ {
    use Element as E;

    let (start, _) = map
        .iter()
        .find(|(_pos, val)| **val == Element::Start)
        .unwrap();

    let (first_direction, first_step, _) = [
        (Vec2::LEFT, &[E::Horizontal, E::BottomLeft, E::TopLeft]),
        (Vec2::RIGHT, &[E::Horizontal, E::BottomRight, E::TopRight]),
        (Vec2::UP, &[E::Vertical, E::TopLeft, E::TopRight]),
        (Vec2::DOWN, &[E::Vertical, E::BottomLeft, E::BottomRight]),
    ]
    .into_iter()
    .map(|(direction, valid)| (direction, start + direction, valid))
    .find(|(_direction, pos, valid)| {
        let Some(elem) = map.get(*pos) else {
            return false;
        };
        valid.contains(elem)
    })
    .unwrap();

    let mut from_dir = first_direction;
    let mut to = first_step;
    std::iter::once((start, map[start])).chain(std::iter::from_fn(move || {
        let res = (to, map[to]);
        from_dir = match map[to] {
            E::Vertical | E::Horizontal => from_dir,
            E::BottomLeft if from_dir == Vec2::LEFT => Vec2::UP,
            E::BottomLeft if from_dir == Vec2::DOWN => Vec2::RIGHT,
            E::TopRight if from_dir == Vec2::RIGHT => Vec2::DOWN,
            E::TopRight if from_dir == Vec2::UP => Vec2::LEFT,
            E::TopLeft if from_dir == Vec2::LEFT => Vec2::DOWN,
            E::TopLeft if from_dir == Vec2::UP => Vec2::RIGHT,
            E::BottomRight if from_dir == Vec2::RIGHT => Vec2::UP,
            E::BottomRight if from_dir == Vec2::DOWN => Vec2::LEFT,
            E::Start => return None,
            E::Ground => unreachable!(),
            other => unreachable!("{:?}", other),
        };
        to = to + from_dir;
        Some(res)
    }))
}
