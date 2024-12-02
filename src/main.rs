use adventofcode2024_clp::helpers::solvers::Solvers;
use adventofcode2024_clp::solvers;
use clap::Parser;

fn main() {
    let solvers = solvers();
    let args = Cli::parse();

    println!();

    if let Some(day) = args.day {
        run_day(&solvers, day, args.part);
    } else {
        for day in 1..=(solvers.len() as u32) {
            run_day(&solvers, day, args.part);
        }
    }
}

fn run_day(solvers: &Solvers, day: u32, part: Option<u32>) {
    println!("Day {day}");

    if let Some(part) = part {
        run_part(solvers, day, part);
    } else {
        run_part(solvers, day, 1);
        run_part(solvers, day, 2);
    }

    println!();
}

fn run_part(solvers: &Solvers, day: u32, part: u32) {
    println!("  Part {part}: {}", solvers.solve(day as usize, part as usize));
}

/// Find solution(s) to Advent of Code 2024 challenges
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Find solution(s) for this day only
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..=25))]
    day: Option<u32>,

    /// Find solution(s) for this part only
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..=2))]
    part: Option<u32>,
}
