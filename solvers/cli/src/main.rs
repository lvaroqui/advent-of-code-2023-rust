use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use anyhow::{bail, Context};
use clap::Parser;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use common::{inventory::LabelQuery, DaySolver, PartResult, UnimplementedSolver};
#[allow(unused_imports)]
use common::{DaySolverImpl, DualDaySolver, MonoDaySolver};
use maybe_shared::MaybeShared;
use reqwest::blocking::Client;

mod maybe_shared;
mod scaffold_solver;

extern crate imports;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    label: Option<Vec<String>>,

    #[arg(short, long)]
    test: bool,

    /// Year to solve
    #[arg(short, long, value_parser = clap::value_parser!(u16), num_args = 1.., value_delimiter = ',')]
    year: Option<Vec<u16>>,

    /// Day to solve
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(0..=24), num_args = 1.., value_delimiter = ',')]
    day: Option<Vec<u8>>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let solvers = common::inventory::Solvers::new().map_err(anyhow::Error::msg)?;

    let labels = args
        .label
        .as_ref()
        .map(|v| v.iter().map(|l| l.as_str()).collect::<Vec<_>>());
    let label_query = match labels.as_deref() {
        Some(s) if s.contains(&"all") => LabelQuery::All,
        Some(s) => LabelQuery::Labeled(s),
        None => LabelQuery::DefaultOnly,
    };

    let mut solvers: Vec<_> = solvers
        .query(args.year.as_deref(), args.day.as_deref(), label_query)
        .collect();

    if solvers.is_empty() {
        if let (Some([year]), Some([day]), None) =
            (args.year.as_deref(), args.day.as_deref(), args.label)
        {
            scaffold_solver::scaffold_solver(*year, *day)?;
            solvers.push(DaySolver {
                year: *year,
                day: *day,
                label: None,
                implementation: UnimplementedSolver.to_day_solver_impl(),
            });
        } else {
            bail!("No matching solver found!")
        }
    }

    let results = solvers
        .into_iter()
        .map(|solver| solve(&solver, args.test))
        .collect::<Result<Vec<_>, _>>()?;

    let total_time = results
        .iter()
        .map(|r| match r.stats {
            MaybeShared::Shared(a) => a,
            MaybeShared::Separate(a, b) => a + b,
        })
        .sum::<Duration>();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "Year",
            "Day",
            "Label",
            "First Part",
            "Second Part",
            "Statistics",
        ])
        .add_rows(results.iter().map(|result| {
            [
                result.year.to_string(),
                result.day.to_string(),
                match &result.label {
                    Some(label) => label.to_string(),
                    None => "-".to_string(),
                },
                format!(
                    "{}{}",
                    validated_to_string(result.part_1_validated),
                    result.part_1,
                ),
                format!(
                    "{}{}",
                    validated_to_string(result.part_2_validated),
                    result.part_2,
                ),
                match result.stats {
                    MaybeShared::Shared(stats) => format!("{:?}", stats),
                    MaybeShared::Separate(stats_1, stats_2) => {
                        format!("{:?} ({:?} + {:?})", stats_1 + stats_2, stats_1, stats_2)
                    }
                },
            ]
        }))
        .add_row_if(
            |_, _| results.len() > 1,
            vec!["", "", "", "", "", &format!("{:?}", total_time)],
        );

    println!("{table}");

    Ok(())
}

fn validated_to_string(validated: Option<bool>) -> impl std::fmt::Display {
    match validated {
        Some(true) => "✅ ",
        Some(false) => "❌ ",
        None => "",
    }
}

fn solve(solver: &DaySolver, test: bool) -> anyhow::Result<DayResult> {
    let runner = if test {
        get_test_io_driver(solver.year, solver.day)?
    } else {
        get_online_io_driver(solver.year, solver.day)?
    };
    let inputs = runner.get_inputs();
    let inputs = inputs.map(|s| s.trim_end());
    let (part_1, part_2, stats) = match &solver.implementation {
        DaySolverImpl::Mono(s) => match inputs {
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
        DaySolverImpl::Dual(s) => {
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
        day: solver.day,
        year: solver.year,
        label: solver.label,
        part_1,
        part_1_validated,
        part_2,
        part_2_validated,
        stats,
    })
}

fn get_online_io_driver(year: u16, day: u8) -> anyhow::Result<Box<dyn IODriver>> {
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

    impl IODriver for RealRunner {
        fn get_inputs(&self) -> MaybeShared<&str> {
            MaybeShared::Shared(&self.input)
        }
    }

    Ok(Box::new(RealRunner {
        input: std::fs::read_to_string(path)?,
    }))
}

fn get_test_io_driver(year: u16, day: u8) -> anyhow::Result<Box<dyn IODriver>> {
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

    impl IODriver for TestRunner {
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
    year: u16,
    day: u8,
    label: Option<&'static str>,
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

trait IODriver {
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
