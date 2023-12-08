use std::str::FromStr;

use common::DayResult;
use itertools::Itertools;

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

    fn ord(&self) -> (Type, [Card; 5]) {
        (self.type_(), self.0)
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

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ord().cmp(&other.ord()))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ord().cmp(&other.ord())
    }
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let hands = input.lines().map(|l| {
            let mut words = l.split_whitespace();
            let hand = Hand::from_str(words.next().unwrap()).unwrap();
            let bid = words.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        });

        let res = hands
            .sorted_by(|(x, _), (y, _)| x.cmp(y))
            .enumerate()
            .map(|(i, (_hand, bid))| (i + 1) * bid)
            .sum::<usize>();

        DayResult::new(res)
    }
}
