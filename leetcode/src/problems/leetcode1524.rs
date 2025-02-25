//! [LeetCode problem 1524: Number of Sub-arrays With Odd Sum][1]
//!
//! [1]: https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum

pub fn num_of_subarrays(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut even = vec![0; n + 1];
    let mut odd = vec![0; n + 1];

    for (i, k) in nums.into_iter().enumerate() {
        if k % 2 == 0 {
            even[i + 1] = even[i] + 1;
            odd[i + 1] = odd[i];
        } else {
            odd[i + 1] = even[i] + 1;
            even[i + 1] = odd[i];
        }
    }

    odd.into_iter()
        .reduce(|acc, e| (acc + e) % 1_000_000_007)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(&[1,3,5], 4)]
    #[test_case(&[2,4,6], 0)]
    #[test_case(&[1,2,3,4,5,6,7], 16)]
    fn test(a: &[i32], z: i32) {
        let a = a.to_owned();
        assert_eq!(z, super::num_of_subarrays(a));
    }
}
