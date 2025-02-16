//! <a href="https://leetcode.com/problems/
//! construct-the-lexicographically-largest-valid-sequence">LeetCode problem
//! 1718: Construct the Lexicographically Largest Valid Sequence</a>

use std::collections::HashSet;

pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    if n < 1 {
        return vec![];
    }

    let len = usize::try_from(2 * n - 1).unwrap();
    let mut done = HashSet::new();
    let mut output = vec![0; len];

    pick(0, n, len, &mut done, &mut output);
    output
}

fn pick(
    i: usize,
    n: i32,
    len: usize,
    done: &mut HashSet<i32>,
    output: &mut Vec<i32>,
) -> bool {
    if i == len {
        return true;
    }

    if output[i] != 0 {
        return pick(i + 1, n, len, done, output);
    }

    for k in (1..=n).rev() {
        if done.contains(&k) {
            continue;
        }

        if k == 1 {
            output[i] = 1;
            done.insert(1);
            if pick(i + 1, n, len, done, output) {
                return true;
            } else {
                output[i] = 0;
                done.remove(&1);
            }
        } else {
            let j = i + usize::try_from(k).unwrap();
            if j < len && output[j] == 0 {
                output[i] = k;
                output[j] = k;
                done.insert(k);
                if pick(i + 1, n, len, done, output) {
                    return true;
                } else {
                    output[i] = 0;
                    output[j] = 0;
                    done.remove(&k);
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(3, &[3,1,2,3,2])]
    #[test_case(5, &[5,3,1,4,3,5,2,4,2])]
    #[test_case(6, &[6,4,2,5,2,4,6,3,5,1,3])]
    #[test_case(10, &[10,8,6,9,3,1,7,3,6,8,10,5,9,7,4,2,5,2,4])]
    fn test(n: i32, r: &[i32]) {
        let r = r.to_owned();
        assert_eq!(super::construct_distanced_sequence(n), r);
    }
}
