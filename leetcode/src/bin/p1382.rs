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

pub struct Solution;

fn main() {}

type OptionalNode = Option<Rc<RefCell<TreeNode>>>;

fn rotate_right(old_root: &Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
  // let Some(old_root) = root else {return None};
  if old_root.borrow().left.as_ref().unwrap().borrow().left.is_none() {
    let left = old_root.borrow().left.clone().unwrap();
    let new_root = left.borrow().right.clone().unwrap();
    left.borrow_mut().right = new_root.borrow().left.clone();
    old_root.borrow_mut().left = new_root.borrow().right.clone();
    new_root.borrow_mut().left = Some(left);
    new_root.borrow_mut().right = Some(old_root.clone());
    return new_root;
  }
  let new_root = old_root.borrow().left.clone().unwrap();
  old_root.borrow_mut().left = new_root.borrow().right.clone();
  let (old_root, _) = balance_and_get_height(&Some(old_root.clone()));
  new_root.borrow_mut().right = old_root;
  return new_root;
}

fn rotate_left(old_root: &Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
  // let Some(old_root) = root else { return None };
  if old_root.borrow().right.as_ref().unwrap().borrow().right.is_none() {
    let right = old_root.borrow().right.clone().unwrap();
    let new_root = right.borrow().left.clone().unwrap();
    right.borrow_mut().left = new_root.borrow().right.clone();
    old_root.borrow_mut().right = new_root.borrow().left.clone();
    new_root.borrow_mut().left = Some(old_root.clone());
    new_root.borrow_mut().right = Some(right);
    return new_root;
  }
  let new_root = old_root.borrow().right.clone().unwrap();
  old_root.borrow_mut().right = new_root.borrow().left.clone();
  let (old_root, _) = balance_and_get_height(&Some(old_root.clone()));
  new_root.borrow_mut().left = old_root;
  return new_root;
}
fn balance_and_get_height(root: &OptionalNode) -> (OptionalNode, u32) {
  let Some(root) = root else { return (None, 0) };
  let (left_node, mut left_height) = balance_and_get_height(&root.borrow().left);
  let (right_node, mut right_height) = balance_and_get_height(&root.borrow().right);

  root.borrow_mut().left = left_node;
  root.borrow_mut().right = right_node;

  let mut new_root = root.clone();

  while left_height.abs_diff(right_height) > 1 {
    if left_height > right_height {
      new_root = rotate_right(&new_root);
      left_height -= 1;
      right_height += 1;
    } else {
      new_root = rotate_left(&new_root);
      right_height -= 1;
      left_height += 1;
    }
  }

  return (Some(new_root), left_height.max(right_height) + 1);
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      let (new_root, _) = balance_and_get_height(&root);
      new_root
    }
}