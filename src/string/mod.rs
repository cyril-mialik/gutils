use std::collections::HashMap;

/// Return whether true or false if it's anagram or not.
///
/// # Examples
///
/// ```
/// use gutils::string::is_anagram;
///
/// let answer = is_anagram("lol".to_string(), "llo".to_string());
/// assert_eq!(answer, true);
///
/// let answer = is_anagram("hello".to_string(), "world".to_string());
/// assert_eq!(answer, false);
/// ```
pub fn is_anagram(s1: String, s2: String) -> bool {
    let mut hm = HashMap::new();

    if s1.len() != s2.len() {
        return false;
    }

    for n in 0..s1.len() {
        *hm.entry(s1.get(n..n+1)).or_insert(0) += 1;
        *hm.entry(s2.get(n..n+1)).or_insert(0) -= 1;
    }

    hm.into_values().max().unwrap() == 0
}

/// Return whether true or false if it's a match between pattern and string.
///
/// # Examples
///
/// ```
/// use gutils::string::is_word_pattern;
///
/// let answer = is_word_pattern("aabbaa", "lol lol kek kek lol lol");
/// assert_eq!(answer, true);
///
/// let answer = is_word_pattern("aaa", "lol kek lol");
/// assert_eq!(answer, false);
/// ```
pub fn is_word_pattern(pattern: &str, s: &str) -> bool {
    let is_valid_string = s.chars().all(|c| c.is_alphanumeric() || c.is_whitespace());
    let is_valid_count = s.split_ascii_whitespace().count() == pattern.len();

    if !is_valid_string || !is_valid_count {
        return false;
    }

    let mut string_hash = HashMap::new();
    let mut pattern_hash = HashMap::new();

    pattern
        .chars()
        .zip(s.split_ascii_whitespace())
        .all(|(c, w)| {
            if pattern_hash.entry(c).or_insert(w) != &w {
                return false;
            }

            if string_hash.entry(w).or_insert(c) != &c {
                return false;
            }

            return true;
        })
}

/// Return whether true or false if it's palindrome or not.
///
/// # Examples
///
/// ```
/// use gutils::string::is_palindrome;
///
/// let answer = is_palindrome("lol", true);
/// assert_eq!(answer, true);
///
/// let answer = is_palindrome("lol", false);
/// assert_eq!(answer, true);
///
/// let answer = is_palindrome("loL", true);
/// assert_eq!(answer, false);
///
/// let answer = is_palindrome("loL", false);
/// assert_eq!(answer, true);
///
/// let answer = is_palindrome("hello", true);
/// assert_eq!(answer, false);
///
/// let answer = is_palindrome("hello", false);
/// assert_eq!(answer, false);
/// ```
pub fn is_palindrome(word: &str, is_case_ignore: bool) -> bool {
    if word.is_empty() {
        return true;
    }

    let chars: Vec<char> = word.chars().collect();
    let (mut first_char, mut last_char) = (0, chars.len() - 1);

    while first_char < last_char {
        if !chars[first_char].is_alphabetic() {
            first_char += 1;
            continue;
        }

        if !chars[last_char].is_alphabetic() {
            last_char -= 1;
            continue;
        }

        if is_case_ignore && (chars[first_char] != chars[last_char]) {
            return false;
        }

        if !is_case_ignore
            && chars[first_char].to_lowercase().to_string()
                != chars[last_char].to_lowercase().to_string()
        {
            return false;
        }

        first_char += 1;
        last_char -= 1;
    }

    true
}

/// Return the length of the last word
///
/// # Examples
///
/// ```
/// use gutils::string::length_of_last_word;
///
/// let answer = length_of_last_word("Hello world!");
/// assert_eq!(answer, 5);
///
/// let answer = length_of_last_word("");
/// assert_eq!(answer, 0);
/// ```
pub fn length_of_last_word(s: &str) -> i32 {
    let result: String = s
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();

    if let Some(last) = result.split_whitespace().last() {
        last.len() as i32
    } else {
        0
    }
}

/// Return whether true or false if s1 has inclusion into s2
///
/// # Examples
///
/// ```
/// use gutils::string::check_inslusion;
///
/// let answer = check_inslusion("abbc".to_string(), "gfaabbcqqw".to_string());
/// assert_eq!(answer, true);
///
/// let answer = check_inslusion("abbc".to_string(), "wqqwe".to_string());
/// assert_eq!(answer, false);
/// ```
pub fn check_inslusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let n = s1.len();
    let mut dict = [0; 26];

    s1.chars().for_each(|c| {
        dict[c as usize - 'a' as usize] += 1;
    });
    s2.chars().take(n).for_each(|c| {
        dict[c as usize - 'a' as usize] -= 1;
    });

    if dict.into_iter().all(|x| x == 0) {
        return true;
    }

    let start = s2.chars();
    let end = s2.chars().skip(n);

    for (out, inn) in start.zip(end) {
        dict[out as usize - 'a' as usize] += 1;
        dict[inn as usize - 'a' as usize] -= 1;

        if dict.into_iter().all(|x| x == 0) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_word_pattern() {
        let answer = is_word_pattern("abba", "lol kek kek lol");
        assert_eq!(answer, true);

        let answer = is_word_pattern("aaa", "lol kek lol");
        assert_eq!(answer, false);
    }

    #[test]
    fn test_is_palindrome() {
        let answer = is_palindrome("lol", true);
        assert_eq!(answer, true);

        let answer = is_palindrome("lol", false);
        assert_eq!(answer, true);

        let answer = is_palindrome("loL", true);
        assert_eq!(answer, false);

        let answer = is_palindrome("loL", false);
        assert_eq!(answer, true);

        let answer = is_palindrome("hello", true);
        assert_eq!(answer, false);

        let answer = is_palindrome("hello", false);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_length_of_last_word() {
        let answer = length_of_last_word("Hello world!");
        assert_eq!(answer, 5);

        let answer = length_of_last_word("");
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_check_inslusion() {
        let answer = check_inslusion("abbc".to_string(), "cdoabbc".to_string());
        assert_eq!(answer, true);

        let answer = check_inslusion("abbc".to_string(), "ppwwm".to_string());
        assert_eq!(answer, false);
    }

    #[test]
    fn test_is_anagram() {
        let answer = is_anagram("abbc".to_string(), "bbca".to_string());
        assert_eq!(answer, true);

        let answer = is_anagram("abbca".to_string(), "dvvb".to_string());
        assert_eq!(answer, false);

        let answer = is_anagram("abb".to_string(), "abc".to_string());
        assert_eq!(answer, false);
    }
}

