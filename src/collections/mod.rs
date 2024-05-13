use std::{collections::HashSet, hash::Hash};

/// Return whether true or false if it's a duplicate
///
/// # Examples
///
/// ```
/// use gutils::collections::is_duplicate;
///
/// let answer = is_duplicate(vec![1, 2, 3, 4, 5]);
/// assert_eq!(answer, false);
///
/// let answer = is_duplicate(vec![1, 2, 3, 3, 5]);
/// assert_eq!(answer, true);
/// ```
pub fn is_duplicate<T: Eq + Hash>(list: Vec<T>) -> bool {
    let mut hs: HashSet<T> = HashSet::with_capacity(list.len());

    for item in list {
        if hs.contains(&item) {
            return true;
        } else {
            hs.insert(item);
        }
    }

    false
}

/// Return whether Some(T) or None if it's a duplicate
///
/// # Examples
///
/// ```
/// use gutils::collections::find_duplicate;
///
/// let answer = find_duplicate(vec![1, 2, 3, 4, 5]);
/// assert_eq!(answer, None);
///
/// let answer = find_duplicate(vec![1, 2, 3, 3, 5]);
/// assert_eq!(answer, Some(3));
/// ```
pub fn find_duplicate<T: Eq + Hash + Copy>(list: Vec<T>) -> Option<T> {
    let mut hs: HashSet<T> = HashSet::with_capacity(list.len()); 

    for item in list {
        if hs.insert(item) {
            continue;
        }

        return Some(item);
    }

    None
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_duplicate() {
        let answer = is_duplicate(vec![1, 2, 3, 4, 5]);
        assert_eq!(answer, false);

        let answer = is_duplicate(vec![1, 2, 3, 3, 5]);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_find_duplicate() {
        let answer = find_duplicate(vec![1, 2, 3, 4, 5]);
        assert_eq!(answer, None);

        let answer = find_duplicate(vec![1, 2, 3, 3, 5]);
        assert_eq!(answer, Some(3));
    }
}
