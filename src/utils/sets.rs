// Source - https://stackoverflow.com/a
// Posted by harmic, modified by community. See post 'Timeline' for change history
// Retrieved 2025-12-08, License - CC BY-SA 4.0

use std::collections::HashSet;
use std::hash::Hash;

pub fn inplace_intersection<T>(a: &mut HashSet<T>, b: &mut HashSet<T>) -> HashSet<T>
where
    T: Hash,
    T: Eq,
{
    let mut c = HashSet::new();

    for v in a.iter() {
        if let Some(found) = b.take(v) {
            c.insert(found);
        }
    }

    a.retain(|v| !c.contains(v));

    c
}
