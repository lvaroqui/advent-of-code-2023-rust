use std::{io::Write, path::PathBuf};

use anyhow::{bail, Context};

fn main() -> anyhow::Result<()> {
    let workspace_root: PathBuf = std::env::args()
        .nth(1)
        .with_context(|| "Workspace root missing as first param")?
        .into();

    let command = std::env::args()
        .nth(2)
        .with_context(|| "Command missing as second param")?;

    match command.as_str() {
        "generate_imports" => generate_imports(workspace_root),
        _ => bail!("Unknown command `{command}`"),
    }
}

const TOML_HEADER: &str = r#"
[package]
name = "imports"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

fn generate_imports(workspace_root: PathBuf) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(workspace_root.join("imports/src"))?;
    let mut toml = std::fs::File::create(workspace_root.join("imports/Cargo.toml"))?;
    toml.write_all(TOML_HEADER.as_bytes())?;
    let mut lib = std::fs::File::create(workspace_root.join("imports/src/lib.rs"))?;

    for year in std::fs::read_dir(workspace_root.join("years"))? {
        let year_path = year?;
        let year: u16 = year_path
            .file_name()
            .to_str()
            .with_context(|| format!("Could not parse year {}", year_path.path().display()))?
            .parse()?;

        for day in std::fs::read_dir(year_path.path())? {
            let day_path = day?;
            let day: u16 = day_path
                .file_name()
                .to_str()
                .with_context(|| format!("Could not parse year {}", day_path.path().display()))?
                .strip_prefix("day")
                .with_context(|| {
                    format!(
                        "Could not remove prefix 'day' to {}",
                        day_path.path().display()
                    )
                })?
                .parse()?;
            writeln!(
                toml,
                "year{year}_day{day:02} = {{ path = \"../years/{year}/day{day:02}\" }}"
            )?;
            writeln!(lib, "extern crate year{year}_day{day:02};")?;
        }
    }

    Ok(())
}
