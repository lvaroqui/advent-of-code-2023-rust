use std::{
    io::{Read, Write},
    path::PathBuf,
    time::{Duration, Instant},
};

use anyhow::{bail, Context};
use clap::Parser;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
#[allow(unused_imports)]
use common::{DaySolver, DualDaySolver, MonoDaySolver};
use reqwest::blocking::Client;

extern crate imports;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year to solve
    #[arg( value_parser = clap::value_parser!(u16))]
    year: u16,

    /// Day to solve
    #[arg( value_parser = clap::value_parser!(u8).range(0..=24))]
    day: Option<u8>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let solvers = common::inventory::Solvers::new().map_err(anyhow::Error::msg)?;

    let solvers = match args.day {
        Some(day) => {
            let Some(solver) = solvers.get(args.year, day) else {
                bail!(
                    "Solver for year {} day {} is not implemented",
                    args.year,
                    day
                );
            };
            vec![(day, solver)]
        }
        None => solvers.year(args.year).collect::<Vec<_>>(),
    };

    let results = solvers
        .into_iter()
        .map(|(day, solver)| solve(args.year, day, solver).map(|res| (day, res)))
        .collect::<Result<Vec<_>, _>>()?;

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
                        match (first.is_resolved(), second.is_resolved()) {
                            (true, true) => {
                                if let Some(second_stats) = second_stats {
                                    format!(
                                        "{:?} ({:?} + {:?})",
                                        first_stats + second_stats,
                                        first_stats,
                                        second_stats
                                    )
                                } else {
                                    format!("{:?}", first_stats)
                                }
                            }
                            (true, false) => format!("{:?}", first_stats),
                            (false, true) => format!("{:?}", second_stats.unwrap()),
                            (false, false) => "NA".to_string(),
                        },
                    ]
                }),
        );

    println!("{table}");

    Ok(())
}

fn solve(
    year: u16,
    day: u8,
    solver: DaySolver,
) -> Result<
    (
        common::DayResult,
        Duration,
        common::DayResult,
        Option<Duration>,
    ),
    anyhow::Error,
> {
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
