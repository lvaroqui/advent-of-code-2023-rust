use common::DayResult;

pub struct Solver;

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let res = input
            .lines()
            .map(|l| {
                let mut values = l.split_whitespace();
                (
                    Shape::from(values.next().unwrap()),
                    Shape::from(values.next().unwrap()),
                )
            })
            .map(|(other, me)| me.value() + me.fight(&other).score())
            .sum::<u32>();
        DayResult::new(res)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let res = input
            .lines()
            .map(|l| {
                let mut values = l.split_whitespace();
                (
                    Shape::from(values.next().unwrap()),
                    Outcome::from(values.next().unwrap()),
                )
            })
            .map(|(other, outcome)| {
                for s in Shape::iter() {
                    if s.fight(&other) == outcome {
                        return s.value() + outcome.score();
                    }
                }
                unreachable!();
            })
            .sum::<u32>();
        DayResult::new(res)
    }
}

#[derive(PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn fight(&self, other: &Shape) -> Outcome {
        match (self.value() as i32 - other.value() as i32 + 3) % 3 {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            2 => Outcome::Lost,
            _ => unreachable!(),
        }
    }

    fn iter() -> impl Iterator<Item = Self> {
        [Shape::Rock, Shape::Paper, Shape::Scissors].into_iter()
    }
}

impl From<&str> for Shape {
    fn from(v: &str) -> Self {
        match v {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Lost,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl From<&str> for Outcome {
    fn from(v: &str) -> Self {
        match v {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        }
    }
}
