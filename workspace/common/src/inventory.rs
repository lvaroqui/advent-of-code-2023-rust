use std::collections::BTreeMap;

use crate::DaySolver;

pub use inventory::submit;

pub struct RegisteredSolver {
    pub year: u16,
    pub day: u8,
    pub constructor: fn() -> DaySolver,
}

inventory::collect!(RegisteredSolver);

#[macro_export]
macro_rules! register_solver {
    ($year:tt, $day:tt, $constructor:tt) => {
        $crate::inventory::submit! {
            $crate::inventory::RegisteredSolver { day: $day, year: $year, constructor: || ($constructor).to_day_solver() }
        }
    };
}

pub use register_solver;

type YearSolverMap = BTreeMap<u16, DaySolverMap>;
type DaySolverMap = BTreeMap<u8, fn() -> DaySolver>;

pub struct Solvers {
    years: YearSolverMap,
}

impl Solvers {
    pub fn new() -> Result<Self, String> {
        let mut years = BTreeMap::new();
        for solver in inventory::iter::<RegisteredSolver> {
            let year = years.entry(solver.year).or_insert_with(BTreeMap::new);
            if year.insert(solver.day, solver.constructor).is_some() {
                return Err(format!(
                    "Attempted to register two solvers for year {} day {}",
                    solver.year, solver.day
                ));
            };
        }

        Ok(Solvers { years })
    }

    pub fn get(&self, year: u16, day: u8) -> Option<DaySolver> {
        self.years
            .get(&year)?
            .get(&day)
            .map(|constructor| constructor())
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, u8, DaySolver)> + '_ {
        self.years.iter().flat_map(|(year, days)| {
            days.iter()
                .map(|(day, constructor)| (*year, *day, constructor()))
        })
    }

    pub fn year(&self, year: u16) -> impl Iterator<Item = (u8, DaySolver)> + '_ {
        let r = self.years.get(&year).unwrap();
        r.iter().map(|(day, constructor)| (*day, constructor()))
    }
}
