//! [LeetCode problem 3174: Clear Digits][1]
//!
//! [1]: https://leetcode.com/problems/clear-digits

pub fn clear_digits(s: String) -> String {
    let mut bytes = s.into_bytes();

    let mut w: usize = 0;
    for r in 0..bytes.len() {
        let byte = bytes[r];
        if byte.is_ascii_digit() {
            w = w.saturating_sub(1);
            continue;
        }

        bytes[w] = byte;
        w += 1;
    }

    bytes.truncate(w);
    String::from_utf8(bytes).unwrap()
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
