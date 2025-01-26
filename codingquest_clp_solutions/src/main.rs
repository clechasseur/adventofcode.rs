use std::time::Instant;

use clap::Parser;
use codingquest_clp::solvers_impl::solvers::Solvers;
use codingquest_clp_solutions::solvers;

fn main() {
    let solvers = solvers();
    let args = Cli::parse();

    println!();

    if let Some(problem) = args.problem {
        run_problem(&solvers, problem);
    } else {
        for problem in solvers.problems() {
            run_problem(&solvers, problem);
        }
    }
}

fn run_problem(solvers: &Solvers, problem: u32) {
    let start = Instant::now();
    let solution = solvers.solve(problem);
    let elapsed = start.elapsed();
    println!("  Problem {problem}: {solution} ({elapsed:.2?})");
}

/// Find solution(s) to CodingQuest.io problems
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Find solution for this problem only
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=25))]
    problem: Option<u32>,
}
