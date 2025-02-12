//! [LeetCode problem 1028: Recover a Tree From Preorder Traversal][1]
//!
//! [1]: https://leetcode.com/problems/recover-a-tree-from-preorder-traversal

use std::{cell::RefCell, rc::Rc};

/// Definition for a binary tree node.
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

pub fn recover_from_preorder(
    traversal: String,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nodes = vec![];

    let mut input = &traversal[..];
    while !input.is_empty() {
        let depth = input
            .find(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
            .unwrap();

        let digits = input[depth..]
            .find('-')
            .unwrap_or_else(|| input.len() - depth);

        let len = depth + digits;
        let value = input[depth..len].parse().unwrap();

        let node = TreeNode::new(value);
        nodes.push((depth, node));

        input = &input[len..];
    }

    let mut stack = vec![];
    for (depth, node) in nodes {
        let node = Rc::new(RefCell::new(node));

        let Some(mut last) = stack.last_mut() else {
            stack.push((depth, node));
            continue;
        };

        while depth < last.0 + 1 {
            stack.pop();
            last = stack.last_mut().unwrap();
        }

        if last.1.borrow().left.is_none() {
            last.1.borrow_mut().left = Some(Rc::clone(&node));
            stack.push((depth, node));
            continue;
        }

        if last.1.borrow().right.is_none() {
            last.1.borrow_mut().right = Some(Rc::clone(&node));
            stack.push((depth, node));
            continue;
        }

        panic!()
    }

    stack
        .first()
        .map(|(_, node)| Rc::clone(node))
}
