use std::{
    collections::HashSet,
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use common::{
    map::{Map, Vec2},
    prelude::*,
};

use chumsky::prelude::*;

register_solver!(2024, 6, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let mut map = parser().parse(input).unwrap();
        let (start_pos, start_dir) = get_starting_pos_dir(&map);

        simulate(&mut map, start_pos, start_dir, |_| {});

        let res = map
            .iter()
            .filter(|(_, element)| matches!(element, Element::Guard(_)))
            .count();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let mut map = parser().parse(input).unwrap();
        let (start_pos, start_dir) = get_starting_pos_dir(&map);

        let mut default_map = map.clone();
        *default_map.get_mut(start_pos).unwrap() = Element::Nothing;
        let starting_map = Arc::new(default_map);

        let possible_obstructions: Arc<Mutex<HashSet<Vec2>>> = Arc::new(Mutex::new(HashSet::new()));

        rayon::scope(|scope| {
            simulate(&mut map, start_pos, start_dir, |p| {
                let obstruction_block = p.current_pos + p.current_dir;
                if obstruction_block == start_pos {
                    return;
                }
                match p.map.get(obstruction_block) {
                    // If there was already an obstruction, don't count it
                    //
                    // More importantly, if we would put an obstruction on the
                    // path that lead us here, we wouldn't have been here! I
                    // wonder it the theme of time travel refers to this subtle
                    // case ^^
                    //
                    // Avoid trying to put on obstruction outside the map
                    Some(Element::Obstruction | Element::Guard(_)) | None => return,
                    _ => (),
                }

                let possible_obstructions = Arc::clone(&possible_obstructions);
                let default_map = Arc::clone(&starting_map);
                scope.spawn(move |_| {
                    let mut obstruction_sim_map = (*default_map).clone();
                    *obstruction_sim_map.get_mut(obstruction_block).unwrap() = Element::Obstruction;
                    *obstruction_sim_map.get_mut(p.current_pos).unwrap() =
                        Element::Guard(HashSet::from([p.current_dir]));

                    if simulate(
                        &mut obstruction_sim_map,
                        p.current_pos,
                        p.current_dir,
                        |_| {},
                    )
                    .is_cyclic
                    {
                        possible_obstructions
                            .lock()
                            .unwrap()
                            .insert(obstruction_block);
                    }
                });
            });
        });

        let res = possible_obstructions.lock().unwrap().len();
        PartResult::new(res)
    }
}

fn get_starting_pos_dir(map: &Map<Element>) -> (Vec2, Vec2) {
    let (start_pos, start_dir) = map
        .iter()
        .find_map(|(pos, e)| match e {
            Element::Guard(v) => Some((pos, v.iter().copied().next().unwrap())),
            Element::Nothing | Element::Obstruction => None,
        })
        .unwrap();
    (start_pos, start_dir)
}

struct SimulationResult {
    is_cyclic: bool,
}

struct StepHookParam<'a> {
    map: &'a Map<Element>,
    current_pos: Vec2,
    current_dir: Vec2,
}

fn simulate(
    map: &mut Map<Element>,
    start_pos: Vec2,
    start_dir: Vec2,
    mut step_hook: impl FnMut(StepHookParam),
) -> SimulationResult {
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;

    let is_cyclic = loop {
        step_hook(StepHookParam {
            current_dir,
            current_pos,
            map,
        });
        match map.get(current_pos + current_dir) {
            Some(Element::Nothing | Element::Guard(_)) => {
                let next_pos = current_pos + current_dir;
                current_pos = next_pos
            }
            Some(Element::Obstruction) => current_dir = current_dir.rotate(PI / 2.0),
            None => break false,
        }
        match map.get_mut(current_pos).unwrap() {
            e @ Element::Nothing | e @ Element::Obstruction => {
                *e = Element::Guard(HashSet::from([current_dir]));
            }
            Element::Guard(v) => {
                if !v.insert(current_dir) {
                    break true;
                }
            }
        };
    };

    SimulationResult { is_cyclic }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Nothing,
    Obstruction,
    Guard(HashSet<Vec2>),
}

fn parser() -> impl Parser<char, Map<Element>, Error = Simple<char>> {
    let nothing = just(".").to(Element::Nothing);
    let obstruction = just("#").to(Element::Obstruction);
    let guard = just("^").to(Element::Guard(HashSet::from([Vec2::NORTH])));

    let element = nothing.or(obstruction).or(guard);

    let line = element.repeated();

    line.separated_by(text::newline())
        .map(Map::new)
        .then_ignore(end())
}
