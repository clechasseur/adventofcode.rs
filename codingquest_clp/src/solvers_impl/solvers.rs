use std::collections::HashMap;
use std::fmt::Display;

use aoclp::solvers_impl::solvers::{Solver, SolverWrapper};
use itertools::Itertools;

#[macro_export]
macro_rules! build_solvers {
    ( [$($problem:literal),+] ) => {
        build_solvers! {
            solvers, [$($problem),+]
        }
    };
    ( $fn_name:ident, [$($problem:literal),+] ) => {
        $crate::aoclp::paste::paste! {
            pub fn $fn_name() -> $crate::solvers_impl::solvers::Solvers {
                let mut solvers = $crate::solvers_impl::solvers::Solvers::default();
                $(
                    solvers.push_problem($problem, [<problem_ $problem>]::solve);
                )+
                solvers
            }
        }
    };
}

#[derive(Default)]
pub struct Solvers {
    solvers: HashMap<u32, Box<dyn Solver>>,
}

impl Solvers {
    pub fn push_problem<S, T>(&mut self, problem: u32, solver: S)
    where
        S: Fn() -> T + 'static,
        T: Display + 'static,
    {
        self.solvers
            .insert(problem, Box::new(SolverWrapper::new(solver)));
    }

    pub fn problems(&self) -> impl Iterator<Item = u32> + '_ {
        self.solvers.keys().copied().sorted_unstable()
    }

    pub fn solve(&self, problem: u32) -> String {
        self.solvers[&problem].solve()
    }
}
