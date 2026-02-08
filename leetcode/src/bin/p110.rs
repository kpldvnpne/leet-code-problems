pub struct Solution;

fn main() {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

type OptionalNode = Option<Rc<RefCell<TreeNode>>>;

enum IsBalanced {
  Yes,
  No,
}

fn is_balanced(node: &OptionalNode) -> (u32, IsBalanced) {
  if let Some(some_node) = node {
    let left = is_balanced(&some_node.borrow().left);
    if matches!(left.1, IsBalanced::No) {
      return left;
    }
    let right = is_balanced(&some_node.borrow().right);
    if matches!(right.1, IsBalanced::No) {
      return right;
    }

    if (left.0 as i32 - right.0 as i32).abs() > 1 {
      return (left.0, IsBalanced::No);
    }

    return (1 + left.0.max(right.0), IsBalanced::Yes);
  } else {
    return (0, IsBalanced::Yes);
  }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let val = is_balanced(&root);
    return matches!(val.1, IsBalanced::Yes);
  }
}
