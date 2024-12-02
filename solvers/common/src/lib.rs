use std::fmt::Display;

pub mod inventory;
pub mod macros;
pub mod map;
pub mod prelude;

#[derive(Default)]
pub struct PartResult(Option<Box<dyn Display>>);

impl Display for PartResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(r) => write!(f, "{}", r),
            None => write!(f, "Unresolved"),
        }
    }
}

impl PartResult {
    pub fn new(val: impl Display) -> Self {
        Self(Some(Box::new(val.to_string())))
    }

    pub fn resolved(&self) -> Option<String> {
        self.0.as_ref().map(|val| val.to_string())
    }
}

pub trait DualDaySolver
where
    Self: 'static,
{
    #[allow(unused_variables)]
    fn solve_1(&self, input: &str) -> PartResult {
        let _ = input;
        PartResult::default()
    }

    #[allow(unused_variables)]
    fn solve_2(&self, input: &str) -> PartResult {
        PartResult::default()
    }

    fn to_day_solver(self) -> DaySolver
    where
        Self: Sized,
    {
        DaySolver::Dual(Box::new(self))
    }
}

pub trait MonoDaySolver
where
    Self: 'static,
{
    #[allow(unused_variables)]
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        (PartResult::default(), PartResult::default())
    }

    fn to_day_solver(self) -> DaySolver
    where
        Self: Sized,
    {
        DaySolver::Mono(Box::new(self))
    }
}

pub enum DaySolver {
    Mono(Box<dyn MonoDaySolver>),
    Dual(Box<dyn DualDaySolver>),
}
