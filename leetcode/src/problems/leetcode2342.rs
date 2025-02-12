//! LeetCode problem 2342: Max Sum of a Pair With Equal Sum of Digits:
//! <https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits>

#[derive(Debug)]
enum Max {
    Single(i32),
    Pair(i32, i32),
}

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut dsum_to_max = HashMap::<u32, Max>::new();
    for k in nums {
        let dsum = digit_sum(k);
        let max = match dsum_to_max.get(&dsum) {
            None => Max::Single(k),
            Some(&Max::Single(largest)) => {
                let min = std::cmp::min(k, largest);
                let max = std::cmp::max(k, largest);
                Max::Pair(max, min)
            }
            Some(&Max::Pair(largest, second)) => {
                if k > largest {
                    Max::Pair(k, largest)
                } else if k > second {
                    Max::Pair(largest, k)
                } else {
                    Max::Pair(largest, second)
                }
            }
        };

        dsum_to_max.insert(dsum, max);
    }

    dbg!(&dsum_to_max);

    dsum_to_max
        .into_values()
        .filter_map(|max| match max {
            Max::Single(_) => None,
            Max::Pair(a, b) => Some(a + b),
        })
        .max()
        .unwrap_or(-1)
}

fn digit_sum(k: i32) -> u32 {
    k.to_string()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .sum()
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
