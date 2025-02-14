//! [LeetCode problem 1352: Product of the Last K Numbers][1]
//!
//! [1]: https://leetcode.com/problems/product-of-the-last-k-numbers

#[derive(Default)]
pub struct ProductOfNumbers {
    prods: Vec<i32>,
}

impl ProductOfNumbers {
    pub fn new() -> Self {
        Self { prods: vec![1] }
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.prods = vec![1];
        } else {
            let n = self.prods.len();
            let last = self.prods[n - 1];
            self.prods.push(last * num);
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        let k = usize::try_from(k).unwrap();
        let n = self.prods.len();
        let m = n - 1;

        use core::cmp::Ordering::*;
        match k.cmp(&(m)) {
            Greater => 0,
            Equal => self.prods[m],
            Less => {
                let i = m - k;
                let last = self.prods[m];
                let pref = self.prods[i];
                last / pref
            }
        }
    }
}
