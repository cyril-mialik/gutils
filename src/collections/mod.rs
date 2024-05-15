use std::{collections::{HashMap, HashSet}, hash::Hash};

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

/// Return length of the longest non-repeatable substring
///
/// # Examples
///
/// ```
/// use gutils::collections::lenght_of_longest_non_repeatable_substring;
///
/// let answer = lenght_of_longest_non_repeatable_substring("abcda".to_string());
/// assert_eq!(answer, 4);
///
/// let answer = lenght_of_longest_non_repeatable_substring("abc".to_string());
/// assert_eq!(answer, 3);
/// ```
pub fn lenght_of_longest_non_repeatable_substring(s: String) -> i32 {
    let mut hm = HashMap::new();
    let mut len = 0;
    let mut start = -1;

    for (end, ch) in s.chars().enumerate() {
        if let Some(i) = hm.insert(ch, end as i32) {
            start = start.max(i);
        }

        len = len.max((end as i32) - start);
    }

    len as i32
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

    #[test]
    fn test_lenght_of_longest_non_repeatable_substring() {
        let answer = lenght_of_longest_non_repeatable_substring("abcdeffeacb".to_string());
        assert_eq!(answer, 6);

        let answer = lenght_of_longest_non_repeatable_substring("".to_string());
        assert_eq!(answer, 0);

        let answer = lenght_of_longest_non_repeatable_substring("a".to_string());
        assert_eq!(answer, 1);
    }
}
