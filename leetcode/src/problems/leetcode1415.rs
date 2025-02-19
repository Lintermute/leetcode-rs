//! <a href="https://leetcode.com/problems/
//! the-k-th-lexicographical-string-of-all-happy-strings-of-length-n">
//! LeetCode problem 1415: The k-th Lexicographical String of All Happy Strings
//! of Length n</a>

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
enum Char {
    A,
    B,
    C,
}

pub fn get_happy_string(n: i32, k: i32) -> String {
    let n = usize::try_from(n).unwrap();
    let k = usize::try_from(k).unwrap();

    [Char::A, Char::B, Char::C]
        .into_iter()
        .flat_map(|next| gen_happy(n, next))
        .nth(k - 1)
        .map(|chars| {
            let mut string = String::with_capacity(chars.len());
            for char in chars.into_iter().rev() {
                let char = match char {
                    Char::A => 'a',
                    Char::B => 'b',
                    Char::C => 'c',
                };
                string.push(char);
            }
            string
        })
        .unwrap_or_else(String::new)
}

fn gen_happy(n: usize, next: Char) -> Vec<Vec<Char>> {
    if n == 1 {
        return vec![vec![next]];
    }

    let (first, second) = match next {
        Char::A => (Char::B, Char::C),
        Char::B => (Char::A, Char::C),
        Char::C => (Char::A, Char::B),
    };

    gen_happy(n - 1, first)
        .into_iter()
        .chain(gen_happy(n - 1, second))
        .map(|mut s| {
            s.push(next);
            s
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(1, 3, "c")]
    #[test_case(1, 4, "")]
    #[test_case(3, 9, "cab")]
    fn test(n: i32, k: i32, r: &str) {
        let r = r.to_owned();
        assert_eq!(super::get_happy_string(n, k), r);
    }
}
