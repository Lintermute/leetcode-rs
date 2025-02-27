//! [LeetCode problem 873: Length of Longest Fibonacci Subsequence][1]
//!
//! [1]: https://leetcode.com/problems/length-of-longest-fibonacci-subsequence

pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let nums: std::collections::HashSet<i32> = arr.iter().copied().collect();

    let mut max = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let mut len = 2;
            let mut pred = arr[i];
            let mut curr = arr[j];
            let mut next = pred + curr;
            while nums.contains(&next) {
                pred = curr;
                curr = next;
                next = pred + curr;
                len += 1;
                max = core::cmp::max(max, len);
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(&[1,2,3,4,5,6,7,8], 5)]
    #[test_case(&[1,3,7,11,12,14,18], 3)]
    fn test(a: &[i32], z: i32) {
        let a = a.to_owned();
        assert_eq!(z, super::len_longest_fib_subseq(a));
    }
}
