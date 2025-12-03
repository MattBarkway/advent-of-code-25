pub fn into_matching_snippets(val: u64, len: usize) -> Option<Vec<String>> {
    split_string_evenly(&val.to_string(), len).filter(|split| all_match(split))
}

fn split_string_evenly(s: &str, n: usize) -> Option<Vec<String>> {
    if !s.len().is_multiple_of(n) || s.len() == n {
        return None;
    }
    Some(
        s.chars()
            .collect::<Vec<_>>()
            .chunks(n)
            .map(|chunk| chunk.iter().collect())
            .collect(),
    )
}

pub fn all_match<T: PartialEq>(vals: &[T]) -> bool {
    if vals.len() <= 1 {
        return false;
    }
    let first = &vals[0];
    vals[1..].iter().all(|item| item == first)
}
