use crate::utils::part::Part;
use anyhow::Result;
pub trait AdventDay {
    fn part_1(&self) -> Result<()>;
    fn part_2(&self) -> Result<()>;

    fn run(&self, part: Part) -> Result<()> {
        match part {
            Part::Part1 => self.part_1()?,
            Part::Part2 => self.part_2()?,
        }
        Ok(())
    }
}
