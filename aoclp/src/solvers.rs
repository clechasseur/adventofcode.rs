use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;

use itertools::Itertools;

/// Macro used to gather functions used to solve [Advent of Code] puzzles
/// and build a [`Solvers`] object allowing to call them easily.
///
/// ## Usage
///
/// In order to use this macro, your solver functions must follow a strict
/// module structure so that the macro can locate the solver functions.
/// Your project can contain solvers for multiple years. Each day must have
/// two solver functions, one for each puzzle part.
///
/// The macro then generates a function named `solvers` that takes no
/// argument and returns a [`Solvers`], which can then be used to invoke
/// the solver for a given year, day and part (1 or 2).
///
/// Solver functions must be accessible from the macro invocation point.
/// Each solver function must take no argument and return a type that
/// implements [`Display`].
///
/// ## Example
///
/// ```
/// # use aoclp::build_solvers;
///
/// /// First, a module for all puzzles in a given year.
/// /// The name must be in the format `y<year>`.
/// ///
/// /// (Of course, the layout could
/// pub mod y2024 {
///     /// Next, a module for the solvers of the puzzles in a given day.
///     /// The name must be in the format `day_<day>`.
///     ///
///     /// Trailing zeroes are optional.
///     pub mod day_01 {
///         /// Finally, two functions, one for each puzzle part.
///         /// The name must be in the format `part_<part>` where `<part>`
///         /// must be `1` or `2` (no trailing zeroes).
///         pub fn part_1() -> usize {
///             // ...
///             # 0
///         }
///
///         /// Each day must have two solvers, one for each part; if the
///         /// second part isn't solved yet or if the day has only one
///         /// part (e.g., the last day of the Advent calendar), simply
///         /// define a function that returns a dummy value.
///         pub fn part_2() -> usize {
///             // ...
///             # 0
///         }
///     }
///
///     /// Each year can have solvers for multiple days.
///     pub mod day_02 {
///         // ...
///         # pub fn part_1() -> usize {
///         #     0
///         # }
///         #
///         # pub fn part_2() -> usize {
///         #     0
///         # }
///     }
/// }
///
/// /// Your project can contain solvers for multiple years.
/// pub mod y2025 {
///     pub mod day_01 {
///         // ...
///         # pub fn part_1() -> usize {
///         #     0
///         # }
///         #
///         # pub fn part_2() -> usize {
///         #     0
///         # }
///     }
/// }
///
/// /// This builds the `solvers` function.
/// build_solvers! {
///     { 2024, [01, 02] }, // If you used trailing zeroes, you must also use them here.
///     { 2025, [01] }
/// }
///
/// // This returns a `Solvers`.
/// let aoc_solvers = solvers();
///
/// // `years` can be used to iterate all years for which there are solvers.
/// // They will be returned as `i32` values, in ascending order.
/// let mut years = aoc_solvers.years();
/// assert_eq!(Some(2024), years.next());
/// assert_eq!(Some(2025), years.next());
/// assert_eq!(None, years.next());
///
/// // `days` can be used to iterate all days for which there are solvers in a given year.
/// // They will be returned as `usize` values, in the order in which they have been
/// // passed to the `build_solvers!` macro.
/// let mut days = aoc_solvers.days(2024);
/// assert_eq!(Some(1), days.next());
/// assert_eq!(Some(2), days.next());
/// assert_eq!(None, days.next());
///
/// // `solve` can be used to invoke the solver for a given year, day and puzzle part.
/// // The value returned by the solver function will be converted to a string via
/// // `to_string` and returned as a `String`.
/// let solution = aoc_solvers.solve(2024, 1, 1);
/// assert_eq!("0", solution);
/// ```
///
/// ## See Also
///
/// For more information on the API used to invoke the solvers, see [`Solvers`].
///
/// For an example on how to use this API to build a command-line tool that can be
/// used to invoke your solvers, see [this lib file] and [this main file].
///
/// [Advent of Code]: https://adventofcode.com/
/// [this lib file]: https://github.com/clechasseur/adventofcode.rs/blob/d604ab1da82dd8fa1dc4fc9e6b0c7c6d322037f9/aoclp_solutions/src/lib.rs
/// [this main file]: https://github.com/clechasseur/adventofcode.rs/blob/d604ab1da82dd8fa1dc4fc9e6b0c7c6d322037f9/aoclp_solutions/src/main.rs
#[macro_export]
macro_rules! build_solvers {
    ( $({ $year:literal, [$($day:literal),+] }),+ ) => {
        build_solvers! {
            solvers, $({ $year, [$($day),+] }),+
        }
    };
    ( $fn_name:ident, $({ $year:literal, [$($day:literal),+] }),+ ) => {
        $crate::paste::paste! {
            pub fn $fn_name() -> $crate::solvers::Solvers {
                let mut solvers = $crate::solvers::Solvers::default();
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
