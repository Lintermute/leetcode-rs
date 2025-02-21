//! <a href="https://leetcode.com/problems/
//! find-elements-in-a-contaminated-binary-tree">LeetCode problem 1261:
//! Find Elements in a Contaminated Binary Tree</a>

use std::{cell::RefCell, collections::HashSet, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val:   i32,
    pub left:  Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct FindElements {
    values: HashSet<i32>,
}

impl FindElements {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut values = HashSet::new();

        let Some(root) = root else {
            return Self { values };
        };

        let mut next: Vec<(i32, Rc<RefCell<_>>)> = vec![(0, Rc::clone(&root))];
        while let Some((val, curr)) = next.pop() {
            values.insert(val);
            if let Some(left) = &curr.borrow().left {
                next.push((2 * val + 1, Rc::clone(left)));
            }
            if let Some(right) = &curr.borrow().right {
                next.push((2 * val + 2, Rc::clone(right)));
            }
        }

        Self { values }
    }

    pub fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }
}
