use std::collections::{BTreeMap, HashMap};

use crate::{DaySolver, DaySolverImpl};

pub use inventory::submit;

type DaySolverBuilder = fn() -> DaySolverImpl;
pub struct RegisteredSolver {
    pub year: u16,
    pub day: u8,
    pub constructor: DaySolverBuilder,
    pub label: Option<&'static str>,
}

inventory::collect!(RegisteredSolver);

#[macro_export]
macro_rules! register_solver {
    ($year:tt, $day:tt, $constructor:tt) => {
        $crate::inventory::submit! {
            $crate::inventory::RegisteredSolver { day: $day, year: $year, constructor: || ($constructor).to_day_solver_impl(), label: None }
        }
    };
    ($year:tt, $day:tt, $constructor:tt, $label:literal) => {
        $crate::inventory::submit! {
            $crate::inventory::RegisteredSolver { day: $day, year: $year, constructor: || ($constructor).to_day_solver_impl(), label: Some($label) }
        }
    };
}

pub use register_solver;

type YearSolverMap = BTreeMap<u16, DaySolverMap>;

#[derive(Debug, Default)]
struct DaySolvers {
    default: Option<DaySolverBuilder>,
    labeled: HashMap<&'static str, DaySolverBuilder>,
}

type DaySolverMap = BTreeMap<u8, DaySolvers>;

pub struct Solvers {
    years: YearSolverMap,
}

#[derive(Debug, Clone, Copy)]
pub enum LabelQuery<'a> {
    DefaultOnly,
    Labeled(&'a [&'a str]),
    All,
}

impl Solvers {
    pub fn new() -> Result<Self, String> {
        let mut years = BTreeMap::new();
        for solver in inventory::iter::<RegisteredSolver> {
            let year = years.entry(solver.year).or_insert_with(BTreeMap::new);
            let day = year.entry(solver.day).or_insert_with(DaySolvers::default);

            match solver.label {
                Some(label) => {
                    if label == "all" {
                        return Err(format!(
                            "Forbidden label for year {} day {}: `{}`",
                            solver.year, solver.day, label
                        ));
                    }

                    if day.labeled.insert(label, solver.constructor).is_some() {
                        return Err(format!(
                            "Attempted to register two solvers for year {} day {} with label `{}`",
                            solver.year, solver.day, label
                        ));
                    }
                }
                None => {
                    if day.default.is_some() {
                        return Err(format!(
                            "Attempted to register two default solvers for year {} day {}",
                            solver.year, solver.day
                        ));
                    }

                    day.default = Some(solver.constructor);
                }
            }

            if true {};
        }

        Ok(Solvers { years })
    }

    pub fn query<'a>(
        &'a self,
        years_query: Option<&'a [u16]>,
        days_query: Option<&'a [u8]>,
        label_query: LabelQuery<'a>,
    ) -> impl Iterator<Item = DaySolver> + 'a {
        self.years
            .iter()
            .filter(move |(year, _)| years_query.map(|y| y.contains(year)).unwrap_or(true))
            .flat_map(move |(year, days)| {
                let year = *year;
                days.iter()
                    .filter(move |(day, _)| days_query.map(|d| d.contains(day)).unwrap_or(true))
                    .flat_map(move |(day, solvers)| {
                        let day = *day;
                        match label_query {
                            LabelQuery::DefaultOnly => {
                                if let Some(default) = solvers.default {
                                    vec![(None, default())]
                                } else {
                                    vec![]
                                }
                            }
                            LabelQuery::Labeled(labels) => solvers
                                .labeled
                                .iter()
                                .filter(|(label, _)| labels.contains(label))
                                .map(|(label, c)| (Some(*label), c()))
                                .collect::<Vec<_>>(),

                            LabelQuery::All => solvers
                                .default
                                .as_ref()
                                .map(|c| (None, c()))
                                .into_iter()
                                .chain(solvers.labeled.iter().map(|(label, c)| (Some(*label), c())))
                                .collect::<Vec<_>>(),
                        }
                        .into_iter()
                        .map(move |(label, implementation)| DaySolver {
                            day,
                            year,
                            label,
                            implementation,
                        })
                    })
            })
    }
}
