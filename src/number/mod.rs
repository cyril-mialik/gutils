/// Return whether true or false if it's palindrome or not.
///
/// # Examples
///
/// ```
/// use gutils::number::is_palindrome;
///
/// let answer = is_palindrome(0);
/// assert_eq!(answer, true);
///
/// let answer = is_palindrome(11);
/// assert_eq!(answer, true);
///
/// let answer = is_palindrome(212);
/// assert_eq!(answer, true);
///
/// let answer = is_palindrome(223);
/// assert_eq!(answer, false);
///
/// let answer = is_palindrome(4441);
/// assert_eq!(answer, false);
///
/// let answer = is_palindrome(55251);
/// assert_eq!(answer, false);
/// ```
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let (mut x, mut rev_x) = (x, 0);

    while x > rev_x {
        rev_x = rev_x * 10 + x % 10;
        x /= 10;
    }

    x == rev_x || x == rev_x / 10
}

/// Return whether true or false if the number is odd or not.
///
/// # Examples
///
/// ```
/// use gutils::number::is_odd;
///
/// let answer = is_odd(20);
/// assert_eq!(answer, false);
///
/// let answer = is_odd(21);
/// assert_eq!(answer, true);
/// ```
pub fn is_odd(x: u32) -> bool {
    x & 1 == 1
}

/// Return whether true or false if the number is even or not.
///
/// # Examples
///
/// ```
/// use gutils::number::is_even;
///
/// let answer = is_even(20);
/// assert_eq!(answer, true);
///
/// let answer = is_even(21);
/// assert_eq!(answer, false);
/// ```
pub fn is_even(x: u32) -> bool {
    x & 1 != 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        let answer = is_palindrome(0);
        assert_eq!(answer, true);

        let answer = is_palindrome(1);
        assert_eq!(answer, true);

        let answer = is_palindrome(2);
        assert_eq!(answer, true);

        let answer = is_palindrome(3);
        assert_eq!(answer, true);

        let answer = is_palindrome(4);
        assert_eq!(answer, true);

        let answer = is_palindrome(5);
        assert_eq!(answer, true);

        let answer = is_palindrome(6);
        assert_eq!(answer, true);

        let answer = is_palindrome(7);
        assert_eq!(answer, true);

        let answer = is_palindrome(8);
        assert_eq!(answer, true);

        let answer = is_palindrome(9);
        assert_eq!(answer, true);

        let answer = is_palindrome(11);
        assert_eq!(answer, true);

        let answer = is_palindrome(222);
        assert_eq!(answer, true);

        let answer = is_palindrome(3333);
        assert_eq!(answer, true);

        let answer = is_palindrome(52225);
        assert_eq!(answer, true);

        let answer = is_palindrome(12);
        assert_eq!(answer, false);

        let answer = is_palindrome(223);
        assert_eq!(answer, false);

        let answer = is_palindrome(3334);
        assert_eq!(answer, false);

        let answer = is_palindrome(52226);
        assert_eq!(answer, false);
    }

    #[test]
    fn is_odd_test() {
        let answer = is_odd(21);
        assert_eq!(answer, true);

        let answer = is_odd(20);
        assert_eq!(answer, false);
    }

    #[test]
    fn is_even_test() {
        let answer = is_even(21);
        assert_eq!(answer, false);

        let answer = is_even(20);
        assert_eq!(answer, true);
    }
}
