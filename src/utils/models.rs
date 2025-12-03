#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Day {
    #[clap(alias = "1")]
    Day1,
    #[clap(alias = "2")]
    Day2,
    #[clap(alias = "3")]
    Day3,
}

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Part {
    #[clap(alias = "1")]
    Part1,
    #[clap(alias = "2")]
    Part2,
}
