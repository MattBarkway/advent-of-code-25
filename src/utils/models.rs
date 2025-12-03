#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Day {
    Day1,
    Day2,
    Day3,
}

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}
