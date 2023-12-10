use std::{
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
    time::{Duration, Instant},
};

use anyhow::Context;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
#[allow(unused_imports)]
use common::{DaySolver, DualDaySolver, MonoDaySolver};
use reqwest::blocking::Client;

fn main() -> anyhow::Result<()> {
    let arg = std::env::args()
        .nth(1)
        .with_context(|| "Please provide a day number / all")?;

    let days = match arg.as_ref() {
        "all" => 1..25,
        day => {
            let day = day.parse()?;
            day..day + 1
        }
    };

    let mut results = Vec::with_capacity(days.len());
    for day in days {
        results.push((day, solve_day(day)?));
    }

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

fn solve_day(
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
    let solver: DaySolver = match day {
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
        _ => anyhow::bail!("Day {} not implemented!", day),
    };
    let mut path = PathBuf::from_str("inputs")?;
    std::fs::create_dir_all(&path)?;
    path.push(day.to_string());
    if !path.exists() {
        let mut f = std::fs::File::create(&path)?;

        let client = Client::new();
        let session_key = std::fs::read_to_string("session-key")?;
        let mut req = client
            .get(format!("https://adventofcode.com/2023/day/{}/input", day))
            .header("Cookie", format!("session={}", session_key))
            .send()?;
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
