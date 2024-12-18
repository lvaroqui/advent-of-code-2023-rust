use common::prelude::*;

use chumsky::prelude::*;
use itertools::Itertools;

register_solver!(2024, 17, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let mut machine = parser().parse(input).unwrap();

        machine.run();

        PartResult::new(machine.format_output())
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let machine = parser().parse(input).unwrap();

        let res = solve(&machine, 0, 0);

        PartResult::new(res.unwrap_or(666))
    }
}

// Observation:
// - My input is a loop that divides A by 2^3 (8) at each operation.
// - Also only the 3 least signifant bits are used during each iterations.
// - Same for example
//
// I believe that this is what makes this puzzle solvable and will solve the
// puzzle based on those facts.
//
// We can solve 3 bits by 3 bits and do a depth-first search for the next 3 bits
// that will produce a matching program part (starting from the end as each
// additional 3 bits adds a new value in front of the output).
fn solve(machine: &Machine, index: usize, a: i64) -> Option<i64> {
    (0..8)
        .filter_map(|p| {
            let mut m = machine.clone();
            let new_a = (a << 3) + p;
            m.registers[0] = new_a;
            m.run();

            if m.output.first().unwrap_or(&8) == m.program.iter().rev().nth(index).unwrap() {
                if index == m.program.len() - 1 {
                    Some(new_a)
                } else {
                    solve(machine, index + 1, new_a)
                }
            } else {
                None
            }
        })
        .min()
}

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn decode(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    registers: [i64; 3],
    program: tinyvec::TinyVec<[u8; 16]>,
    instruction_pointer: usize,
    output: tinyvec::TinyVec<[u8; 16]>,
}

#[derive(Debug, Clone, Copy)]
enum OpType {
    Literal,
    Combo,
}

impl Machine {
    fn run(&mut self) {
        while self.cycle() {}
    }

    fn cycle(&mut self) -> bool {
        let op = OpCode::decode(self.program[self.instruction_pointer]);
        let operand = self.program[self.instruction_pointer + 1];
        let combo_op = || self.decode_operand(OpType::Combo, operand);
        let literal_op = || self.decode_operand(OpType::Literal, operand);

        match op {
            OpCode::Adv => self.registers[0] >>= combo_op(),
            OpCode::Bxl => self.registers[1] ^= literal_op(),
            OpCode::Bst => self.registers[1] = combo_op() % 8,
            OpCode::Jnz => {
                if self.registers[0] != 0 {
                    self.instruction_pointer = literal_op() as usize;
                    return self.instruction_pointer < self.program.len();
                }
            }
            OpCode::Bxc => self.registers[1] ^= self.registers[2],
            OpCode::Out => self.output.push((combo_op() % 8) as u8),
            OpCode::Bdv => self.registers[1] = self.registers[0] >> combo_op(),
            OpCode::Cdv => self.registers[2] = self.registers[0] >> combo_op(),
        };

        self.instruction_pointer += 2;

        self.instruction_pointer < self.program.len()
    }

    fn decode_operand(&self, op_type: OpType, value: u8) -> i64 {
        match (op_type, value) {
            (OpType::Literal, _) | (OpType::Combo, 0..4) => value as i64,
            (OpType::Combo, 4..7) => self.registers[value as usize - 4],
            _ => unreachable!("{:?} {}", op_type, value),
        }
    }

    fn format_output(&self) -> impl std::fmt::Display + '_ {
        self.output.iter().format(",")
    }
}

fn parser() -> impl Parser<char, Machine, Error = Simple<char>> {
    let number = text::int(10).map(|i: String| i.parse::<i64>().unwrap());

    let register = just("Register ")
        .ignore_then(any())
        .ignore_then(just(": "))
        .ignore_then(number);

    let registers = register.separated_by(text::newline());

    let program = just("Program: ").ignore_then(number.map(|n| n as u8).separated_by(just(",")));

    registers
        .then_ignore(text::whitespace())
        .then(program)
        .map(|(registers, program)| Machine {
            registers: registers.try_into().unwrap(),
            program: program.as_slice().into(),
            instruction_pointer: 0,
            output: tinyvec::tiny_vec!([u8; 16]),
        })
}
