//! [LeetCode problem 1749: Maximum Absolute Sum of Any Subarray][1]
//!
//! [1]: https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray

pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut min = 0;
    let mut sum = 0;
    for k in nums {
        sum += k;
        min = min.min(sum);
        max = max.max(sum);
    }
    max - min
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    #[test_case(&[1,-3,2,3,-4], 5)]
    #[test_case(&[2,-5,1,-4,3,-2], 8)]
    fn test(a: &[i32], z: i32) {
        let a = a.to_owned();
        assert_eq!(z, super::max_absolute_sum(a));
    }
}
