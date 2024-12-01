use std::{
    io::{Read, Write},
    path::PathBuf,
    time::{Duration, Instant},
};

use anyhow::Context;
use clap::Parser;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
#[allow(unused_imports)]
use common::{DaySolver, DualDaySolver, MonoDaySolver};
use reqwest::blocking::Client;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year to solve
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(2022..=2023))]
    year: u16,

    /// Day to solve
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(0..=24))]
    day: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let results = vec![(args.day, solve_day(args.year, args.day)?)];

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Day", "First Part", "Second Part", "Perf"])
        .add_rows(
            results
                .into_iter()
                .map(|(day, (first, first_stats, second, second_stats))| {
                    vec![
                        day.to_string(),
                        first.to_string(),
                        second.to_string(),
                        if let Some(second_stats) = second_stats {
                            format!(
                                "{:?} ({:?} + {:?})",
                                first_stats + second_stats,
                                first_stats,
                                second_stats
                            )
                        } else {
                            format!("{:?}", first_stats)
                        },
                    ]
                }),
        );

    println!("{table}");

    Ok(())
}

macro_rules! match_days {
    ($day:expr) => {
        match $day {
            1 => day01::Solver.to_day_solver(),
            2 => day02::Solver.to_day_solver(),
            3 => day03::Solver.to_day_solver(),
            4 => day04::Solver.to_day_solver(),
            5 => day05::Solver.to_day_solver(),
            6 => day06::Solver.to_day_solver(),
            7 => day07::Solver.to_day_solver(),
            8 => day08::Solver.to_day_solver(),
            9 => day09::Solver.to_day_solver(),
            10 => day10::Solver.to_day_solver(),
            11 => day11::Solver.to_day_solver(),
            12 => day12::Solver.to_day_solver(),
            13 => day13::Solver.to_day_solver(),
            14 => day14::Solver.to_day_solver(),
            15 => day15::Solver.to_day_solver(),
            16 => day16::Solver.to_day_solver(),
            17 => day17::Solver.to_day_solver(),
            18 => day18::Solver.to_day_solver(),
            19 => day19::Solver.to_day_solver(),
            20 => day20::Solver.to_day_solver(),
            21 => day21::Solver.to_day_solver(),
            22 => day22::Solver.to_day_solver(),
            23 => day23::Solver.to_day_solver(),
            24 => day24::Solver.to_day_solver(),
            day => anyhow::bail!("Day {} not implemented!", day),
        }
    };
}

macro_rules! match_years {
    ([$($year:tt),*], $y:expr, $day:expr) => {
        common::macros::paste! {
        match $y {
            $($year => {
                use [<year $year>]::*;
                match_days!($day)
            })*,
            year => anyhow::bail!("Year {} not implemented!", year)
            }
        }
    };
}

fn solve_day(
    year: u16,
    day: u8,
) -> Result<
    (
        common::DayResult,
        Duration,
        common::DayResult,
        Option<Duration>,
    ),
    anyhow::Error,
> {
    let solver: DaySolver = match_years!([2022, 2023], year, day);
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
    let input = std::fs::read_to_string(path)?;
    let input = input.trim_end();
    let (first, first_stats, second, second_stats) = match solver {
        DaySolver::Mono(s) => {
            let ((first, second), stats) = instrument(|| s.solve(input));
            (first, stats, second, None)
        }
        DaySolver::Dual(s) => {
            let (first, first_stats) = instrument(|| s.solve_1(input));
            let (second, second_stats) = instrument(|| s.solve_2(input));
            (first, first_stats, second, Some(second_stats))
        }
    };
    Ok((first, first_stats, second, second_stats))
}

fn instrument<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let n = Instant::now();
    let res = f();
    let duration = n.elapsed();

    (res, duration)
}
