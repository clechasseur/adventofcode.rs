use std::fmt::Display;
use std::marker::PhantomData;

#[macro_export]
macro_rules! build_solvers {
    ( $($day:literal),+ ) => {
        ::paste::paste! {
            pub fn solvers() -> $crate::helpers::solvers::Solvers {
                let mut solvers = $crate::helpers::solvers::Solvers::default();
                $(
                    solvers.push_day($crate::[<day_ $day>]::part_1, $crate::[<day_ $day>]::part_2);
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
    solvers: Vec<Vec<Box<dyn Solver>>>,
}

impl Solvers {
    pub fn push_day<S1, T, S2, U>(&mut self, part_1: S1, part_2: S2)
    where
        S1: Fn() -> T + 'static,
        T: Display + 'static,
        S2: Fn() -> U + 'static,
        U: Display + 'static,
    {
        self.solvers.push(vec![
            Box::new(SolverWrapper::<_, T>::new(part_1)),
            Box::new(SolverWrapper::<_, U>::new(part_2)),
        ]);
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.solvers.len()
    }

    pub fn solve(&self, day: usize, part: usize) -> String {
        self.solvers[day - 1][part - 1].solve()
    }
}
