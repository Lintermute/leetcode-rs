//! [LeetCode problem 1980: Find Unique Binary String][1]
//!
//! [1]: https://leetcode.com/problems/find-unique-binary-string

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let nums: std::collections::HashSet<u16> = nums
        .into_iter()
        .map(|s| u16::from_str_radix(&s, 2).unwrap())
        .collect();

    for k in 0..=u16::MAX {
        if !nums.contains(&k) {
            return format!("{k:0width$b}", width = nums.len());
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    // Additional solutions exist for each test case!
    #[test_case(&["01","10"], "00")]
    #[test_case(&["00","01"], "10")]
    #[test_case(&["111","011","001"], "000")]
    #[test_case(
        &["001011","111110","010110","010010","101111","011001"],
        "000000")]
    fn test(nums: &[&str], r: &str) {
        let nums = nums
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(r, &super::find_different_binary_string(nums));
    }
}
