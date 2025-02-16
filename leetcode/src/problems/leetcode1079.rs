//! [LeetCode problem 1079: Letter Tile Possibilities][1]
//!
//! [1]: https://leetcode.com/problems/letter-tile-possibilities

use std::collections::HashMap;

pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut chars: HashMap<char, usize> = HashMap::new();
    for char in tiles.chars() {
        if let Some(freq) = chars.get_mut(&char) {
            *freq += 1;
        } else {
            chars.insert(char, 1);
        }
    }

    combinations(&chars)
}

fn combinations(chars: &HashMap<char, usize>) -> i32 {
    if chars.is_empty() {
        return 0;
    }

    chars
        .iter()
        .map(|(&char, &freq)| {
            let mut chars = chars.clone();
            if freq == 1 {
                chars.remove(&char);
            } else {
                chars.insert(char, freq - 1);
            }

            1 + combinations(&chars)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("AAB", 8)]
    #[test_case("AAABBC", 188)]
    #[test_case("V", 1)]
    fn test(tiles: &str, r: i32) {
        let tiles = tiles.to_owned();
        assert_eq!(super::num_tile_possibilities(tiles), r);
    }
}
