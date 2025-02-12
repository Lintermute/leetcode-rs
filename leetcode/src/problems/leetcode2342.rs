//! LeetCode problem 2342: Max Sum of a Pair With Equal Sum of Digits:
//! <https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits>

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    use std::{cmp, collections::HashMap};

    let mut max = -1;
    let mut dsum_to_max = HashMap::<i32, i32>::new();

    for k in nums {
        let dsum = digit_sum(k);
        if let Some(&l) = dsum_to_max.get(&dsum) {
            max = cmp::max(max, k + l);
            dsum_to_max.insert(dsum, cmp::max(k, l));
        } else {
            dsum_to_max.insert(dsum, k);
        }
    }

    max
}

fn digit_sum(mut k: i32) -> i32 {
    let mut digit_sum = 0;
    while k != 0 {
        digit_sum += k % 10;
        k /= 10;
    }
    digit_sum
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(&[18,43,36,13,7], 54)]
    #[test_case(&[10,12,19,14], -1)]
    fn test(nums: &[i32], r: i32) {
        let nums = nums.to_owned();
        assert_eq!(super::maximum_sum(nums), r);
    }
}
