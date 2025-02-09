//! [LeetCode problem 2364: Count Number of Bad Pairs][1]
//!
//! [1]: https://leetcode.com/problems/count-number-of-bad-pairs

pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    let n = nums.len();

    let mut freq_by_hash: HashMap<i32, usize> = HashMap::new();
    for (i, e) in nums.into_iter().enumerate() {
        let i = i32::try_from(i).unwrap();
        let hash = e - i;
        if let Some(freq) = freq_by_hash.get_mut(&hash) {
            *freq += 1;
        } else {
            freq_by_hash.insert(hash, 1);
        }
    }

    let pairs_total: usize = (n * (n - 1)) / 2;
    let pairs_good: usize = freq_by_hash
        .into_values()
        .map(|freq| (freq * (freq - 1)) / 2)
        .sum();

    let pairs_bad = pairs_total - pairs_good;
    pairs_bad.try_into().unwrap()
}

#[cfg(test)]
mod tests {

    use test_case::test_case;

    #[test_case(vec![4,1,3,3], 5)]
    #[test_case(vec![1,2,3,4,5], 0)]
    fn test(nums: Vec<i32>, r: i64) {
        assert_eq!(super::count_bad_pairs(nums), r);
    }
}
