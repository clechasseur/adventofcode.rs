use adventofcode2024_clp::RUNNERS;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    println!();

    if let Some(day) = args.day {
        run_day(day, args.part);
    } else {
        for day in 1..=(RUNNERS.len() as u32) {
            run_day(day, args.part);
        }
    }
}

fn run_day(day: u32, part: Option<u32>) {
    println!("Day {day}");

    if let Some(part) = part {
        run_part(day, part);
    } else {
        run_part(day, 1);
        run_part(day, 2);
    }

    println!();
}

fn run_part(day: u32, part: u32) {
    let runner_fn = RUNNERS[(day - 1) as usize][(part - 1) as usize];

    println!("  Part {part}: {}", runner_fn());
}

/// Find solution(s) to Advent of Code 2024 challenges
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Find solution(s) for this day only
    #[arg(long, value_parser = parse_day)]
    day: Option<u32>,

    /// Find solution(s) for this part only
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..=2))]
    part: Option<u32>,
}

fn parse_day(day: &str) -> Result<u32, String> {
    let day = day.parse().map_err(|e| format!("{e:?}"))?;
    if (1..(RUNNERS.len() as u32)).contains(&day) {
        Ok(day)
    } else {
        Err(format!("{day} is not in 1..={}", RUNNERS.len()))
    }
}
