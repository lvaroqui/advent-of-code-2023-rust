use std::collections::HashSet;

use common::{
    map::{Map, Vec2},
    prelude::*,
};
use itertools::Itertools;

register_solver!(2023, 10, Solver);
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
            _ => Element::Ground,
        }
    }
}

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let map = parse(input);

        PartResult::new(pipe_iter(&map, false).count() / 2)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let map = parse(input);

        let mut turn_count = 0;
        let pipe_parts = pipe_iter(&map, false)
            .inspect(|p| {
                turn_count += match p.turn_dir {
                    Some(Direction::Left) => -1,
                    Some(Direction::Right) => 1,
                    None => 0,
                }
            })
            .map(|p| p.pos)
            .collect::<HashSet<_>>();

        // We want an iterator that goes through the pipe clockwise
        let iter = pipe_iter(&map, turn_count < 0);

        let mut inner_parts = HashSet::<Vec2>::new();
        for p in iter {
            use Element as E;
            let to_check = match p.element {
                E::Vertical if p.from_dir == Vec2::UP => vec![Vec2::RIGHT],
                E::Vertical if p.from_dir == Vec2::DOWN => vec![Vec2::LEFT],
                E::Horizontal if p.from_dir == Vec2::LEFT => vec![Vec2::UP],
                E::Horizontal if p.from_dir == Vec2::RIGHT => vec![Vec2::DOWN],
                E::BottomLeft if p.from_dir == Vec2::LEFT => vec![],
                E::BottomLeft if p.from_dir == Vec2::DOWN => {
                    vec![Vec2::DOWN, Vec2::LEFT]
                }
                E::TopRight if p.from_dir == Vec2::RIGHT => vec![],
                E::TopRight if p.from_dir == Vec2::UP => {
                    vec![Vec2::UP, Vec2::RIGHT]
                }
                E::TopLeft if p.from_dir == Vec2::LEFT => {
                    vec![Vec2::UP, Vec2::LEFT]
                }
                E::TopLeft if p.from_dir == Vec2::UP => vec![],
                E::BottomRight if p.from_dir == Vec2::RIGHT => {
                    vec![Vec2::DOWN, Vec2::RIGHT]
                }
                E::BottomRight if p.from_dir == Vec2::DOWN => vec![],
                _ => unreachable!("{:?}", p),
            };

            for dir in to_check {
                let adjacent = p.pos + dir;
                if map.get(adjacent).is_some() && !pipe_parts.contains(&adjacent) {
                    inner_parts.insert(adjacent);
                }
            }
        }

        loop {
            let to_add = inner_parts
                .iter()
                .flat_map(|p| map.adjacent_pos_iter(*p))
                .filter(|p| !inner_parts.contains(p) && !pipe_parts.contains(p))
                .collect::<HashSet<_>>();
            if to_add.is_empty() {
                break;
            }
            inner_parts.extend(to_add);
        }

        PartResult::new(inner_parts.len())
    }
}

fn parse(input: &str) -> Map<Element> {
    let map = Map::new(
        input
            .lines()
            .map(|line| line.chars().map(Element::from).collect_vec())
            .collect_vec(),
    );
    map
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct PipePart {
    pos: Vec2,
    element: Element,
    from_dir: Vec2,
    turn_dir: Option<Direction>,
}

fn pipe_iter(map: &Map<Element>, reverse: bool) -> impl Iterator<Item = PipePart> + '_ {
    use Element as E;

    let (start, _) = map
        .iter()
        .find(|(_pos, val)| **val == Element::Start)
        .unwrap();

    let directions = [
        (Vec2::LEFT, &[E::Horizontal, E::BottomLeft, E::TopLeft]),
        (Vec2::RIGHT, &[E::Horizontal, E::BottomRight, E::TopRight]),
        (Vec2::UP, &[E::Vertical, E::TopLeft, E::TopRight]),
        (Vec2::DOWN, &[E::Vertical, E::BottomLeft, E::BottomRight]),
    ]
    .into_iter()
    .map(|(direction, valid)| (direction, start + direction, valid))
    .filter(|(_direction, pos, valid)| {
        let Some(elem) = map.get(*pos) else {
            return false;
        };
        valid.contains(elem)
    })
    .collect_vec();

    assert_eq!(directions.len(), 2);

    let start_element = match (directions[0].0, directions[1].0) {
        (Vec2::LEFT, Vec2::RIGHT) | (Vec2::RIGHT, Vec2::LEFT) => Element::Horizontal,
        (Vec2::DOWN, Vec2::UP) | (Vec2::UP, Vec2::DOWN) => Element::Vertical,
        (Vec2::DOWN, Vec2::RIGHT) | (Vec2::RIGHT, Vec2::DOWN) => Element::TopLeft,
        (Vec2::DOWN, Vec2::LEFT) | (Vec2::LEFT, Vec2::DOWN) => Element::TopRight,
        (Vec2::UP, Vec2::RIGHT) | (Vec2::RIGHT, Vec2::UP) => Element::BottomLeft,
        (Vec2::UP, Vec2::LEFT) | (Vec2::LEFT, Vec2::UP) => Element::BottomRight,
        other => unreachable!("{:?}", other),
    };

    let (first_direction, first_step, _) = if reverse {
        directions[0]
    } else {
        directions[1]
    };
    let mut from_dir = first_direction;
    let mut to = first_step;

    std::iter::once(PipePart {
        pos: start,
        element: start_element,
        from_dir: if reverse {
            directions[1].0 * -1
        } else {
            directions[0].0 * -1
        },
        turn_dir: None,
    })
    .chain(std::iter::from_fn(move || {
        let (new_from_dir, turn_dir) = match map[to] {
            E::Vertical | E::Horizontal => (from_dir, None),
            E::BottomLeft if from_dir == Vec2::LEFT => (Vec2::UP, Some(Direction::Right)),
            E::BottomLeft if from_dir == Vec2::DOWN => (Vec2::RIGHT, Some(Direction::Left)),
            E::TopRight if from_dir == Vec2::RIGHT => (Vec2::DOWN, Some(Direction::Right)),
            E::TopRight if from_dir == Vec2::UP => (Vec2::LEFT, Some(Direction::Left)),
            E::TopLeft if from_dir == Vec2::LEFT => (Vec2::DOWN, Some(Direction::Left)),
            E::TopLeft if from_dir == Vec2::UP => (Vec2::RIGHT, Some(Direction::Right)),
            E::BottomRight if from_dir == Vec2::RIGHT => (Vec2::UP, Some(Direction::Left)),
            E::BottomRight if from_dir == Vec2::DOWN => (Vec2::LEFT, Some(Direction::Right)),
            E::Start => return None,
            E::Ground => unreachable!(),
            other => unreachable!("{:?} from dir {:?} at {:?}", other, from_dir, to),
        };
        let res = PipePart {
            pos: to,
            element: map[to],
            from_dir,
            turn_dir,
        };
        from_dir = new_from_dir;
        to = to + from_dir;
        Some(res)
    }))
}
