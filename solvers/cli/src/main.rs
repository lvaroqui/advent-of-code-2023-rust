use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use anyhow::Context;
use clap::Parser;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use common::PartResult;
#[allow(unused_imports)]
use common::{DaySolver, DualDaySolver, MonoDaySolver};
use maybe_shared::MaybeShared;
use reqwest::blocking::Client;

mod maybe_shared;
mod scaffold_solver;

extern crate imports;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year to solve
    #[arg(value_parser = clap::value_parser!(u16))]
    year: u16,

    /// Day to solve
    #[arg(value_parser = clap::value_parser!(u8).range(0..=24))]
    day: Option<u8>,

    #[arg(short, long)]
    test: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let solvers = common::inventory::Solvers::new().map_err(anyhow::Error::msg)?;

    let solvers = match args.day {
        Some(day) => {
            let Some(solver) = solvers.get(args.year, day) else {
                scaffold_solver::scaffold_solver(args.year, day)?;
                return Ok(());
            };
            vec![(day, solver)]
        }
        None => solvers.year(args.year).collect::<Vec<_>>(),
    };

    let results = solvers
        .into_iter()
        .map(|(day, solver)| solve(args.year, day, solver, args.test).map(|res| (day, res)))
        .collect::<Result<Vec<_>, _>>()?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Day", "First Part", "Second Part", "Statistics"])
        .add_rows(results.into_iter().map(|(day, result)| {
            [
                day.to_string(),
                format!(
                    "{}{}",
                    result.part_1,
                    validated_to_string(result.part_1_validated)
                ),
                format!(
                    "{}{}",
                    result.part_2,
                    validated_to_string(result.part_2_validated)
                ),
                match result.stats {
                    MaybeShared::Shared(stats) => format!("{:?}", stats),
                    MaybeShared::Separate(stats_1, stats_2) => {
                        format!("{:?} ({:?} + {:?})", stats_1 + stats_2, stats_1, stats_2)
                    }
                },
            ]
        }));

    println!("{table}");

    Ok(())
}

fn validated_to_string(validated: Option<bool>) -> impl std::fmt::Display {
    match validated {
        Some(true) => " ✅",
        Some(false) => " ❌",
        None => "",
    }
}

fn solve(year: u16, day: u8, solver: DaySolver, test: bool) -> anyhow::Result<DayResult> {
    let runner = if test {
        get_test_runner(year, day)?
    } else {
        get_input_runner(year, day)?
    };
    let inputs = runner.get_inputs();
    let inputs = inputs.map(|s| s.trim_end());
    let (part_1, part_2, stats) = match solver {
        DaySolver::Mono(s) => match inputs {
            MaybeShared::Shared(input) => {
                let ((part_1, part_2), stats) = instrument(|| s.solve(input));
                (part_1, part_2, MaybeShared::Shared(stats))
            }
            MaybeShared::Separate(first, second) => {
                let ((part_1, _), stats_1) = instrument(|| s.solve(first));
                let ((_, part_2), stats_2) = instrument(|| s.solve(second));
                (part_1, part_2, MaybeShared::Separate(stats_1, stats_2))
            }
        },
        DaySolver::Dual(s) => {
            let (part_1, stats_1) = instrument(|| s.solve_1(inputs.first()));
            let (part_2, stats_2) = instrument(|| s.solve_2(inputs.second()));

            (part_1, part_2, MaybeShared::Separate(stats_1, stats_2))
        }
    };

    let part_1_validated = part_1
        .resolved()
        .and_then(|r| runner.validate(Part::One, &r));
    let part_2_validated = part_2
        .resolved()
        .and_then(|r| runner.validate(Part::Two, &r));

    Ok(DayResult {
        part_1,
        part_1_validated,
        part_2,
        part_2_validated,
        stats,
    })
}

fn get_input_runner(year: u16, day: u8) -> anyhow::Result<Box<dyn Runner>> {
    let mut path = PathBuf::from("inputs").join(year.to_string());
    std::fs::create_dir_all(&path)?;
    path.push(day.to_string());

    if !path.exists() {
        let client = Client::new();
        let session_key = std::fs::read_to_string("session-key")
            .with_context(|| "could not open `session-key` file")?;
        let mut req = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                year, day
            ))
            .header("Cookie", format!("session={}", session_key))
            .send()?
            .error_for_status()
            .with_context(|| format!("could not download input for year {year} day {day}"))?;

        let mut f = std::fs::File::create(&path)?;
        let mut buf = [0; 4096];
        while let Ok(w) = req.read(&mut buf) {
            if w == 0 {
                break;
            }
            f.write_all(&buf[0..w])?;
        }
    }

    struct RealRunner {
        input: String,
    }

    impl Runner for RealRunner {
        fn get_inputs(&self) -> MaybeShared<&str> {
            MaybeShared::Shared(&self.input)
        }
    }

    Ok(Box::new(RealRunner {
        input: std::fs::read_to_string(path)?,
    }))
}

fn get_test_runner(year: u16, day: u8) -> anyhow::Result<Box<dyn Runner>> {
    let mut path = PathBuf::from("tests").join(year.to_string());
    std::fs::create_dir_all(&path)?;
    path.push(day.to_string());
    std::fs::create_dir_all(&path)?;

    let open_or_create_and_read = |p: &Path, create: bool| -> anyhow::Result<String> {
        let mut buf = String::new();
        let _ = OpenOptions::new()
            .create(create)
            .append(true)
            .read(true)
            .open(p)
            .and_then(|mut f| f.read_to_string(&mut buf));

        if create && buf.is_empty() {
            eprintln!("{} is empty...", p.display());
        }

        Ok(buf)
    };

    let input_1 = open_or_create_and_read(&path.join("input_1"), true)?;
    let input_2 = open_or_create_and_read(&path.join("input_2"), false)?;
    let answer_1 = open_or_create_and_read(&path.join("answer_1"), true)?;
    let answer_2 = open_or_create_and_read(&path.join("answer_2"), true)?;

    struct TestRunner {
        inputs: MaybeShared<String>,
        answer_1: String,
        answer_2: String,
    }

    impl Runner for TestRunner {
        fn get_inputs(&self) -> MaybeShared<&str> {
            self.inputs.as_deref()
        }

        fn validate(&self, part: Part, solution: &str) -> Option<bool> {
            Some(match part {
                Part::One => solution == self.answer_1,
                Part::Two => solution == self.answer_2,
            })
        }
    }

    let inputs = if input_2.is_empty() {
        MaybeShared::Shared(input_1)
    } else {
        MaybeShared::Separate(input_1, input_2)
    };

    Ok(Box::new(TestRunner {
        inputs,
        answer_1,
        answer_2,
    }))
}

struct DayResult {
    part_1: PartResult,
    part_1_validated: Option<bool>,
    part_2: PartResult,
    part_2_validated: Option<bool>,
    stats: MaybeShared<Duration>,
}

#[derive(Debug, Clone, Copy)]
enum Part {
    One,
    Two,
}

trait Runner {
    fn get_inputs(&self) -> MaybeShared<&str>;
    fn validate(&self, _part: Part, _solution: &str) -> Option<bool> {
        None
    }
}

fn instrument<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let n = Instant::now();
    let res = f();
    let duration = n.elapsed();

    (res, duration)
}
