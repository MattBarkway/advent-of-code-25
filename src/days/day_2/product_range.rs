use crate::days::day_2::errors::ParseProductError;
use crate::days::day_2::errors::ParseProductError::InvalidFormat;
use crate::days::day_2::utils::into_matching_snippets;
use std::str::FromStr;

pub struct ProductRange {
    pub(crate) min: u64,
    pub(crate) max: u64,
}

impl FromStr for ProductRange {
    type Err = ParseProductError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim().split('-').collect::<Vec<&str>>();
        if value.len() != 2 {
            return Err(InvalidFormat);
        }
        let min = value[0].parse::<u64>()?;
        let max = value[1].parse::<u64>()?;
        Ok(Self { min, max })
    }
}

impl ProductRange {
    pub fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }

    pub fn get_repeating_twice(&self) -> Vec<u64> {
        (self.min..(self.max + 1))
            .filter(|&candidate| {
                let s = candidate.to_string();
                let mid = s.len() / 2;
                s.len() % 2 == 0 && s[..mid] == s[mid..]
            })
            .collect()
    }

    pub fn get_repeating(&self) -> Vec<u64> {
        (self.min..(self.max + 1))
            .filter_map(|candidate| self.check_one(candidate).map(|_| candidate))
            .collect()
    }

    fn check_one(&self, candidate: u64) -> Option<u64> {
        (1..(candidate / 2) as u32)
            .filter_map(move |f| into_matching_snippets(candidate, f as usize).map(|_| candidate))
            .next()
            .map(|_| candidate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_repeating_twice() {
        let range = ProductRange::new(11, 22);
        let repeating = range.get_repeating_twice();
        assert_eq!(repeating.len(), 2);

        let range = ProductRange::new(222220, 222224);
        let repeating = range.get_repeating_twice();
        assert_eq!(repeating.len(), 1);
    }

    #[test]
    fn test_get_repeating() {
        let range = ProductRange::new(11, 22);
        let repeating = range.get_repeating();
        assert_eq!(repeating.len(), 2);

        let range = ProductRange::new(998, 1012);
        let repeating = range.get_repeating();
        assert_eq!(repeating.len(), 2);
    }
}
