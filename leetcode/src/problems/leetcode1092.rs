//! [LeetCode problem 1092: Shortest Common Supersequence][1]
//!
//! [1]: https://leetcode.com/problems/shortest-common-supersequence

pub fn shortest_common_supersequence(s1: String, s2: String) -> String {
    let n = s1.len();
    let m = s2.len();

    let mut prev = Vec::with_capacity(m + 1);
    for j in 0..=m {
        prev.push(s2[0..j].to_owned());
    }

    for i in 0..n {
        let mut curr = Vec::with_capacity(m + 1);
        curr.push(s1[0..=i].to_owned());
        for j in 0..m {
            let c1 = &s1[i..=i];
            let c2 = &s2[j..=j];
            if c1 == c2 {
                let mut prev = prev[j].to_owned();
                prev.push_str(c2);
                curr.push(prev);
            } else {
                let a = &prev[j + 1];
                let b = &curr[j];
                let s = if a.len() < b.len() {
                    let mut a = a.to_owned();
                    a.push_str(c1);
                    a
                } else {
                    let mut b = b.to_owned();
                    b.push_str(c2);
                    b
                };
                curr.push(s);
            }
        }
        prev = curr;
    }

    prev.into_iter().last().unwrap()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("abac", "cab", "cabac")]
    #[test_case("aaaaaaaa", "aaaaaaaa", "aaaaaaaa")]
    fn test(a: &str, b: &str, z: &str) {
        let a = a.to_owned();
        let b = b.to_owned();
        assert_eq!(z, &super::shortest_common_supersequence(a, b));
    }
}
