//! [LeetCode problem 3174: Clear Digits][1]
//!
//! [1]: https://leetcode.com/problems/clear-digits

pub fn clear_digits(s: String) -> String {
    let mut stack = Vec::with_capacity(s.len());

    for char in s.chars() {
        if !char.is_ascii_digit() {
            stack.push(char);
        } else {
            stack.pop();
        }
    }

    String::from_iter(stack)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("abc", "abc")]
    #[test_case("cb34", "")]
    fn test(s: &str, r: &str) {
        let s = s.to_owned();
        assert_eq!(super::clear_digits(s), r);
    }
}
