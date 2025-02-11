//! [LeetCode problem 1910: Remove All Occurrences of a Substring][1]
//!
//! [1]: https://leetcode.com/problems/remove-all-occurrences-of-a-substring

pub fn remove_occurrences(s: String, part: String) -> String {
    let mut result = String::with_capacity(s.len());
    for char in s.chars() {
        result.push(char);
        while let Some(offs) = offset_of_suffix(&result, &part) {
            result.truncate(offs);
        }
    }

    result
}

fn offset_of_suffix(haystack: &str, needle: &str) -> Option<usize> {
    let h = haystack.len();
    let n = needle.len();

    if h < n {
        return None;
    }

    let offs = h - n;
    (&haystack[offs..] == needle).then_some(offs)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("daabcbaabcbc", "abc", "dab")]
    #[test_case("axxxxyyyyb", "xy", "ab")]
    fn test(s: &str, p: &str, r: &str) {
        let s = s.to_owned();
        let p = p.to_owned();
        assert_eq!(super::remove_occurrences(s, p), r);
    }
}
