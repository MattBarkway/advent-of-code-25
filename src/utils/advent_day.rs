use crate::utils::errors::RunError;
use crate::utils::part::Part;

pub trait AdventDay {
    fn part_1(&self) -> Result<(), RunError>;
    fn part_2(&self) -> Result<(), RunError>;

    fn run(&self, part: Part) -> Result<(), RunError> {
        match part {
            Part::Part1 => self.part_1()?,
            Part::Part2 => self.part_2()?,
        }
        Ok(())
    }
}
