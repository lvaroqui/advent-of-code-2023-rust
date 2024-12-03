use common::prelude::*;

register_solver!(2022, 13, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let res = input
            .split("\n\n")
            .map(|l| {
                let mut it = l.split('\n');
                (parse(it.next().unwrap()), parse(it.next().unwrap()))
            })
            .enumerate()
            .filter(|(_, (a, b))| a <= b)
            .map(|(i, _)| i + 1)
            .sum::<usize>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let mut res: Vec<_> = input.split_whitespace().map(parse).collect();
        let divider1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
        let divider2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
        res.push(divider1.clone());
        res.push(divider2.clone());

        res.sort();

        let pos1 = res.iter().position(|p| *p == divider1).unwrap() + 1;
        let pos2 = res.iter().position(|p| *p == divider2).unwrap() + 1;

        PartResult::new(pos1 * pos2)
    }
}

#[derive(Eq, Clone, Debug)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                let mut a_it = a.iter();
                let mut b_it = b.iter();
                loop {
                    let (a, b) = (a_it.next(), b_it.next());
                    match (a, b) {
                        (None, None) => break std::cmp::Ordering::Equal,
                        (None, Some(_)) => break std::cmp::Ordering::Less,
                        (Some(_), None) => break std::cmp::Ordering::Greater,
                        (Some(a), Some(b)) => match a.cmp(b) {
                            std::cmp::Ordering::Equal => continue,
                            other => break other,
                        },
                    };
                }
            }
            (a, b) => a.upgrade().cmp(&b.upgrade()),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn upgrade(&self) -> Self {
        match self {
            Packet::Integer(a) => Packet::List(vec![Packet::Integer(*a)]),
            list => list.clone(),
        }
    }
}

fn parse(input: &str) -> Packet {
    if input.as_bytes()[0] == b'[' {
        let mut nest = 0;
        let mut cur = 0;
        let mut split = vec![];
        let input = &input[1..input.len() - 1];
        for (i, c) in input.chars().enumerate() {
            match c {
                '[' => nest += 1,
                ']' => nest -= 1,
                ',' if nest == 0 => {
                    split.push(parse(&input[cur..i]));
                    cur = i + 1;
                }
                _ => (),
            };
        }
        if !input.is_empty() {
            split.push(parse(&input[cur..]));
        }
        Packet::List(split)
    } else {
        Packet::Integer(input.parse().unwrap())
    }
}
