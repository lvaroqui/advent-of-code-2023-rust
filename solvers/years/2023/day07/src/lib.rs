use std::{cmp::Ordering, str::FromStr};

use common::prelude::*;
use itertools::Itertools;

register_solver!(2023, 7, Solver);
pub struct Solver;

#[derive(Debug, Clone, Copy, strum_macros::FromRepr, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars().map(Card::from).collect_vec().try_into().unwrap(),
        ))
    }
}

impl Hand {
    fn iter_count(&self) -> impl Iterator<Item = (Card, usize)> + '_ {
        let mut res = [0; Card::A as usize + 1];
        for c in self.0 {
            res[c as usize] += 1;
        }
        res.into_iter()
            .enumerate()
            .map(|(i, c)| (Card::from_repr(i).unwrap(), c))
    }
}

impl Hand {
    fn type_(&self) -> Type {
        let cards = self
            .iter_count()
            .map(|(_c, count)| count)
            .filter(|count| *count > 0)
            .sorted()
            .collect_vec();

        let mut it = cards.iter().rev();

        match it.next().unwrap() {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => match it.next().unwrap() {
                2 => Type::FullHouse,
                1 => Type::ThreeOfAKind,
                _ => unreachable!(),
            },
            2 => match it.next().unwrap() {
                2 => Type::TwoPair,
                1 => Type::OnePair,
                _ => unreachable!(),
            },
            1 => Type::HighCard,
            _ => unreachable!(),
        }
    }

    fn ord1(&self) -> (Type, [Card; 5]) {
        (self.type_(), self.0)
    }

    fn cmp1(&self, other: &Self) -> std::cmp::Ordering {
        self.ord1().cmp(&other.ord1())
    }

    fn type_with_joker(&self) -> Type {
        let mut joker = 0;

        let cards = self
            .iter_count()
            .filter(|(c, count)| {
                if *c == Card::J {
                    joker = *count;
                    false
                } else {
                    *count > 0
                }
            })
            .map(|(_c, count)| count)
            .sorted()
            .collect_vec();

        if joker == 5 {
            return Type::FiveOfAKind;
        }

        let mut it = cards.iter().rev();

        match it.next().unwrap() {
            5 => Type::FiveOfAKind,
            4 if joker == 1 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 if joker == 2 => Type::FiveOfAKind,
            3 if joker == 1 => Type::FourOfAKind,
            3 => match it.next().unwrap() {
                2 => Type::FullHouse,
                1 => Type::ThreeOfAKind,
                _ => unreachable!(),
            },
            2 if joker == 3 => Type::FiveOfAKind,
            2 if joker == 2 => Type::FourOfAKind,
            2 => match it.next().unwrap() {
                2 if joker == 1 => Type::FullHouse,
                2 => Type::TwoPair,
                1 if joker == 1 => Type::ThreeOfAKind,
                1 => Type::OnePair,
                _ => unreachable!(),
            },
            1 if joker == 4 => Type::FiveOfAKind,
            1 if joker == 3 => Type::FourOfAKind,
            1 if joker == 2 => Type::ThreeOfAKind,
            1 if joker == 1 => Type::OnePair,
            1 => Type::HighCard,
            _ => unreachable!(),
        }
    }

    fn ord2(&self) -> (Type, [u8; 5]) {
        (
            self.type_with_joker(),
            self.0.map(|c| match c {
                Card::J => 0, // Joker lowest card
                Card::Two => 1,
                Card::Three => 2,
                Card::Four => 3,
                Card::Five => 4,
                Card::Six => 5,
                Card::Seven => 6,
                Card::Eight => 7,
                Card::Nine => 8,
                Card::T => 9,
                Card::Q => 10,
                Card::K => 11,
                Card::A => 12,
            }),
        )
    }

    fn cmp2(&self, other: &Self) -> std::cmp::Ordering {
        self.ord2().cmp(&other.ord2())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        solve(input, Hand::cmp1)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        solve(input, Hand::cmp2)
    }
}

fn solve(input: &str, cmp: impl Fn(&Hand, &Hand) -> Ordering) -> PartResult {
    let hands = input.lines().map(|l| {
        let mut words = l.split_whitespace();
        let hand = Hand::from_str(words.next().unwrap()).unwrap();
        let bid = words.next().unwrap().parse::<usize>().unwrap();
        (hand, bid)
    });

    let res = hands
        .sorted_by(|(x, _), (y, _)| cmp(x, y))
        .enumerate()
        .map(|(i, (_hand, bid))| (i + 1) * bid)
        .sum::<usize>();

    PartResult::new(res)
}
