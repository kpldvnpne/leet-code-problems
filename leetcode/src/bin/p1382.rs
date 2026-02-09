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

fn traverse(root: &OptionalNode) -> Vec<i32> {
  let Some(root) = root else { return vec![] };
  let mut left = traverse(&root.borrow().left);
  left.push(root.borrow().val);
  left.append(&mut traverse(&root.borrow().right));

  return left;
}

fn construct_bst(a: &[i32]) -> OptionalNode {
  if a.len() == 0 {
    return None;
  }

  let mid = a.len() / 2;
  let val = a[mid];
  let root = Some(Rc::new(RefCell::new(TreeNode {
    val,
    left: construct_bst(&a[0..mid]),
    right: construct_bst(&a[mid+1..a.len()])
  })));
  return root;
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let all_elems = traverse(&root);
    construct_bst(&all_elems)
  }
}