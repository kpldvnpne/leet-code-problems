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
      right: None
    }
  }
}

type OptionalNode = Option<Rc<RefCell<TreeNode>>>;

fn sum(prev: i32, node: &OptionalNode) -> i32 {
  let Some(node) = node else { 
    return prev 
  };
  let left = &node.borrow().left;
  let right = &node.borrow().right;

  let prev = (prev << 1) + node.borrow().val;

  if left.is_none() && right.is_none() {
    return prev;
  } else {

    if right.is_none() && !left.is_none() {
      return sum(prev, left);
    } else if !right.is_none() && left.is_none() {
      return sum(prev, right);
    } else if !right.is_none() && !left.is_none() {
      return sum(prev, left) + sum(prev, right);
    } else {
      unreachable!();
    }
  }
} 


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return sum(0, &root);
    }
}