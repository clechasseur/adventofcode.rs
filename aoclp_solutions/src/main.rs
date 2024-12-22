use std::time::Instant;

use aoclp_solutions::helpers::solvers::Solvers;
use aoclp_solutions::solvers;
use clap::Parser;

fn main() {
    let solvers = solvers();
    let args = Cli::parse();

    println!();

    if let Some(day) = args.day {
        run_day(&solvers, args.year.unwrap_or_else(|| default_year(&solvers)), day, args.part);
    } else {
        for year in solvers.years() {
            for day in solvers.days(year) {
                run_day(&solvers, year, day as u32, args.part);
            }
        }
    }
}

fn run_day(solvers: &Solvers, year: i32, day: u32, part: Option<u32>) {
    println!("Year {year}, day {day}");

    if let Some(part) = part {
        run_part(solvers, year, day, part);
    } else {
        run_part(solvers, year, day, 1);
        run_part(solvers, year, day, 2);
    }

    println!();
}

fn run_part(solvers: &Solvers, year: i32, day: u32, part: u32) {
    let start = Instant::now();
    let solution = solvers.solve(year, day as usize, part as usize);
    let elapsed = start.elapsed();
    println!("  Part {part}: {solution} ({elapsed:.2?})");
}

fn default_year(solvers: &Solvers) -> i32 {
    solvers.years().last().unwrap()
}

/// Find solution(s) to Advent of Code challenges
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Find solution(s) for this year only (defaults to latest year)
    #[arg(short, long)]
    year: Option<i32>,

    /// Find solution(s) for this day only
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=25))]
    day: Option<u32>,

    /// Find solution(s) for this part only
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=2))]
    part: Option<u32>,
}
