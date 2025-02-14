//! [LeetCode problem 1352: Product of the Last K Numbers][1]
//!
//! [1]: https://leetcode.com/problems/product-of-the-last-k-numbers

#[derive(Default)]
pub struct ProductOfNumbers {
    nums: Vec<i32>,
}

impl ProductOfNumbers {
    pub fn new() -> Self {
        Self { nums: vec![] }
    }

    pub fn add(&mut self, num: i32) {
        self.nums.push(num)
    }

    pub fn get_product(&self, k: i32) -> i32 {
        let k = usize::try_from(k).unwrap();
        self.nums.iter().rev().take(k).product()
    }
}
