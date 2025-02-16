//! [LeetCode problem 2375: Construct Smallest Number From DI String][1]
//!
//! [1]: https://leetcode.com/problems/construct-smallest-number-from-di-string

pub fn smallest_number(pattern: String) -> String {
    for i in 1..=9 {
        let digits = (1..=9).filter(|&j| i != j).collect();
        if let Some(digits) = gen_digits(&pattern, digits, i) {
            return core::iter::once(i)
                .chain(digits.into_iter().rev())
                .map(|d| (b'0' + d) as char)
                .collect();
        }
    }

    panic!();
}

fn gen_digits(pattern: &str, digits: Vec<u8>, pred: u8) -> Option<Vec<u8>> {
    match pattern.chars().next() {
        None => Some(vec![]),
        Some('I') => pick_candidate(pattern, digits, |&&d| pred < d),
        Some('D') => pick_candidate(pattern, digits, |&&d| pred > d),
        _ => panic!(),
    }
}

fn pick_candidate(
    pattern: &str,
    digits: Vec<u8>,
    filter: impl Fn(&&u8) -> bool,
) -> Option<Vec<u8>> {
    for &next in digits.iter().filter(filter) {
        let mut digits = digits.clone();
        digits.retain(|&e| e != next);

        if let Some(mut result) = gen_digits(&pattern[1..], digits, next) {
            result.push(next);
            return Some(result);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("IIIDIDDD", "123549876")]
    #[test_case("DDD", "4321")]
    fn test(s: &str, r: &str) {
        let s = s.to_owned();
        let r = r.to_owned();
        assert_eq!(super::smallest_number(s), r);
    }
}
