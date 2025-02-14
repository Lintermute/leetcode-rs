//! [LeetCode problem 1352: Product of the Last K Numbers][1]
//!
//! [1]: https://leetcode.com/problems/product-of-the-last-k-numbers

#[derive(Default)]
pub struct ProductOfNumbers {
    prods: Vec<i32>,
}

impl ProductOfNumbers {
    pub fn new() -> Self {
        Self { prods: vec![] }
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.prods.clear();
        } else {
            let last = self.prods.last().unwrap_or(&1);
            self.prods.push(last * num);
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        let k = usize::try_from(k).unwrap();
        let n = self.prods.len();
        let last = self.prods.last().copied().unwrap_or(0);

        use core::cmp::Ordering::*;
        match k.cmp(&n) {
            Greater => 0,
            Equal => last,
            Less => {
                let i = n - k - 1;
                let prefix = self.prods[i];
                last / prefix
            }
        }
    }
}
