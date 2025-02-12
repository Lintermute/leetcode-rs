//! <a href="https://leetcode.com/problems/
//! construct-binary-tree-from-preorder-and-postorder-traversal">
//! LeetCode problem 889:
//! Construct Binary Tree from Preorder and Postorder Traversal</a>

use std::{cell::RefCell, rc::Rc};

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

pub fn construct_from_pre_post(
    preorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut postorder = postorder;
    postorder.pop(); // Ensure root does not get popped from `stack` later
    let mut postorder = postorder.into_iter().peekable();

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
    for value in preorder {
        let node = Rc::new(RefCell::new(TreeNode::new(value)));
        if let Some(last) = stack.last_mut() {
            let mut last = last.borrow_mut();
            if last.left.is_none() {
                last.left = Some(Rc::clone(&node));
            } else if last.right.is_none() {
                last.right = Some(Rc::clone(&node));
            } else {
                panic!()
            }
        }

        stack.push(node);

        loop {
            let Some(&to_pop) = postorder.peek() else {
                break;
            };

            let Some(pos) = stack
                .iter()
                .position(|level| level.borrow().val == to_pop)
            else {
                break;
            };

            stack.truncate(pos);
            postorder.next();
        }
    }

    stack.into_iter().next()
}
