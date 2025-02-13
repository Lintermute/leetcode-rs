//! <a href="https://leetcode.com/problems/
//! minimum-operations-to-exceed-threshold-value-ii">
//! LeetCode problem 3066: Minimum Operations to Exceed Threshold Value II
//! </a>

use core::cmp::Reverse;

use std::collections::BinaryHeap;

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use core::cmp::{max, min};

    let mut nums: BinaryHeap<Reverse<u64>> = nums
        .into_iter()
        .map(|e| Reverse(u64::try_from(e).unwrap()))
        .collect();

    let k = u64::try_from(k).unwrap();

    let mut count = 0;
    while let Some((a, b)) = pop_pair_less_than(&mut nums, k) {
        nums.push(Reverse(min(a, b) * 2 + max(a, b)));
        count += 1;
    }

    count
}

fn pop_pair_less_than(
    heap: &mut BinaryHeap<Reverse<u64>>,
    k: u64,
) -> Option<(u64, u64)> {
    match (heap.pop(), heap.pop()) {
        (Some(Reverse(a)), Some(Reverse(b))) if a < k => Some((a, b)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(&[2,11,10,1,3], 10, 2)]
    #[test_case(&[1,1,2,4,9], 20, 4)]
    #[test_case(&[97,24,43,45,45,26], 45, 2)]
    #[test_case(
        &[1000000000,999999999,1000000000,999999999,1000000000,999999999],
        1000000000,
        2)]
    #[test_case(
        &[999999999,999999999,999999999],
        1000000000,
        2)]
    fn test(nums: &[i32], k: i32, r: i32) {
        let nums = nums.to_owned();
        assert_eq!(super::min_operations(nums, k), r);
    }
}
