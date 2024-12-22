use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;

use itertools::Itertools;

#[macro_export]
macro_rules! build_solvers {
    ( $({ $year:literal, [$($day:literal),+] }),+ ) => {
        build_solvers! {
            solvers, $({ $year, [$($day),+] }),+
        }
    };
    ( $fn_name:ident, $({ $year:literal, [$($day:literal),+] }),+ ) => {
        $crate::paste::paste! {
            pub fn $fn_name() -> $crate::solvers_impl::solvers::Solvers {
                let mut solvers = $crate::solvers_impl::solvers::Solvers::default();
                $(
                    $(
                        solvers.push_day($year, [<y $year>]::[<day_ $day>]::part_1, [<y $year>]::[<day_ $day>]::part_2);
                    )+
                )+
                solvers
            }
        }
    };
}

pub trait Solver {
    fn solve(&self) -> String;
}

pub struct SolverWrapper<S, T> {
    solver: S,
    _phantom_t: PhantomData<T>,
}

impl<S, T> SolverWrapper<S, T> {
    pub fn new(solver: S) -> Self {
        Self { solver, _phantom_t: Default::default() }
    }
}

impl<S, T> Solver for SolverWrapper<S, T>
where
    S: Fn() -> T,
    T: Display,
{
    fn solve(&self) -> String {
        (self.solver)().to_string()
    }
}

#[derive(Default)]
pub struct Solvers {
    solvers: HashMap<i32, Vec<Vec<Box<dyn Solver>>>>,
}

impl Solvers {
    pub fn push_day<S1, T, S2, U>(&mut self, year: i32, part_1: S1, part_2: S2)
    where
        S1: Fn() -> T + 'static,
        T: Display + 'static,
        S2: Fn() -> U + 'static,
        U: Display + 'static,
    {
        self.solvers.entry(year).or_default().push(vec![
            Box::new(SolverWrapper::<_, T>::new(part_1)),
            Box::new(SolverWrapper::<_, U>::new(part_2)),
        ]);
    }

    pub fn years(&self) -> impl Iterator<Item = i32> + '_ {
        self.solvers.keys().copied().sorted_unstable()
    }

    pub fn days(&self, year: i32) -> impl Iterator<Item = usize> {
        1..=self.solvers.get(&year).map(Vec::len).unwrap_or_default()
    }

    pub fn solve(&self, year: i32, day: usize, part: usize) -> String {
        self.solvers[&year][day - 1][part - 1].solve()
    }
}
