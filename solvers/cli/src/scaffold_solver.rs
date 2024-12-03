use std::{io::Write, path::PathBuf};

const TOML_TEMPLATE: &str = r#"[package]
name = "<package_name>"
version = "0.1.0"
edition = "2021"

[dependencies]
common.workspace = true
itertools.workspace = true
chumsky.workspace = true
"#;

const LIB_RS_TEMPLATE: &str = r#"use common::prelude::*;

use chumsky::prelude::*;

register_solver!(<year>, <day>, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        // let input = parser().parse(input).unwrap();

        PartResult::default()
    }
}

// fn parser() -> impl Parser<char, _, Error = Simple<char>> {
//     todo!()
// }

"#;

pub(crate) fn scaffold_solver(year: u16, day: u8) -> anyhow::Result<()> {
    eprintln!(
        "Solver for year {} day {} does not exists, scaffolding crate...",
        year, day
    );

    let crate_root = PathBuf::from("years")
        .join(year.to_string())
        .join(format!("day{day:02}"));

    std::fs::create_dir_all(&crate_root)?;

    let toml = crate_root.join("Cargo.toml");
    std::fs::File::create_new(toml)?.write_all(
        TOML_TEMPLATE
            .replace("<package_name>", &format!("year{year}_day{day:02}"))
            .as_bytes(),
    )?;

    let src_dir = crate_root.join("src");
    std::fs::create_dir_all(&src_dir)?;
    let lib_rs = src_dir.join("lib.rs");
    std::fs::File::create_new(&lib_rs)?.write_all(
        LIB_RS_TEMPLATE
            .replace("<year>", &year.to_string())
            .replace("<day>", &day.to_string())
            .as_bytes(),
    )?;

    eprintln!(
        "Scaffolding done, now implement your solution: {}",
        lib_rs.display()
    );

    Ok(())
}
