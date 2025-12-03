mod days;
mod utils;

use crate::days::day_1::run::DayOne;
use crate::days::day_2::run::DayTwo;
use crate::days::day_3::run::DayThree;
use crate::utils::advent_day::AdventDay;
use crate::utils::day::Day;
use crate::utils::part::Part;
use anyhow::Result;
use clap::Parser;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// The day to run (e.g., -d 1, --day 3)
    #[arg(short, long)]
    day: Day,

    /// The part to run (e.g., -p 1, --part 2)
    #[arg(short, long)]
    part: Part,
}

fn main() -> Result<()> {
    FmtSubscriber::builder().init();

    let cli = Cli::parse();

    match &cli.day {
        Day::Day1 => DayOne.run(cli.part)?,
        Day::Day2 => DayTwo.run(cli.part)?,
        Day::Day3 => DayThree.run(cli.part)?,
    }
    Ok(())
}
