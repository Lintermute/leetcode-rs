//! [LeetCode problem 2698: Find the Punishment Number of an Integer][1]
//!
//! [1]: https://leetcode.com/problems/find-the-punishment-number-of-an-integer

pub fn punishment_number(n: i32) -> i32 {
    (1..=n)
        .filter_map(|i| {
            let p = i * i;
            let d = digits(p);
            let x = partition(&d);
            x.iter().any(|e| e == &i).then_some(p)
        })
        .sum()
}

fn digits(mut k: i32) -> Vec<i32> {
    if k == 0 {
        return vec![0];
    }

    let mut digits = vec![];
    while k != 0 {
        digits.push(k % 10);
        k /= 10;
    }

    digits.reverse();
    digits
}

fn to_dec(d: &[i32]) -> i32 {
    let mut sum = 0;
    for k in d {
        sum *= 10;
        sum += k;
    }
    sum
}

fn partition(d: &[i32]) -> Vec<i32> {
    if d.is_empty() {
        return vec![];
    }

    let mut partitions = vec![];
    for j in 1..d.len() {
        let a = to_dec(&d[..j]);
        let p = partition(&d[j..]);
        for b in p {
            partitions.push(a + b);
        }
    }

    partitions.push(to_dec(d));

    partitions
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(10, 182)]
    #[test_case(37, 1478)]
    fn test(n: i32, r: i32) {
        assert_eq!(super::punishment_number(n), r);
    }

    #[test_case(&[1, 2, 1], &[4, 22, 13, 121])]
    fn partition(d: &[i32], r: &[i32]) {
        let r = r.to_vec();
        assert_eq!(super::partition(d), r);
    }
}
