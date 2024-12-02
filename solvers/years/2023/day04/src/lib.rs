use std::collections::HashSet;

use chumsky::prelude::*;
use common::prelude::*;

register_solver!(2023, 4, Solver);

pub struct Solver;

#[derive(Debug)]
struct Card {
    matches: usize,
}

impl Card {
    fn points(&self) -> u32 {
        if self.matches > 0 {
            2_u32.pow(self.matches as u32 - 1)
        } else {
            0
        }
    }
}

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let cards = parser().parse(input).unwrap();
        let res = cards.iter().map(Card::points).sum::<u32>();
        DayResult::new(res)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let cards = parser().parse(input).unwrap();
        let mut card_counts = vec![1; cards.len()];

        for (i, card) in cards.iter().enumerate() {
            for j in i + 1..i + 1 + card.matches {
                card_counts[j] += card_counts[i]
            }
        }

        DayResult::new(card_counts.iter().sum::<usize>())
    }
}

fn parser() -> impl Parser<char, Vec<Card>, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<u32>().unwrap());

    let numbers = number.padded().repeated();

    let card = just("Card")
        .padded()
        .ignore_then(number)
        .then_ignore(just(':'))
        .then(numbers)
        .then_ignore(just('|'))
        .then(numbers)
        .map(|((_id, winning_numbers), numbers)| {
            let winning_numbers = winning_numbers.into_iter().collect::<HashSet<u32>>();
            let numbers = numbers.into_iter().collect::<HashSet<u32>>();
            Card {
                matches: winning_numbers.intersection(&numbers).count(),
            }
        });

    card.repeated().then_ignore(end())
}
