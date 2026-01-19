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

fn calculate_sum_node(node: &OptionalNode) -> OptionalNode {
    match node {
        Some(some_node) => {
            let left_sum_node = calculate_sum_node(&some_node.borrow().left);
            let left_sum = left_sum_node.as_ref().map_or(0, |it| it.borrow().val);
            let right_sum_node = calculate_sum_node(&some_node.borrow().right);
            let right_sum = right_sum_node.as_ref().map_or(0, |it| it.borrow().val);
            let total_sum = left_sum + right_sum + some_node.borrow().val;
            Some(Rc::new(RefCell::new(TreeNode {
                val: total_sum,
                left: left_sum_node,
                right: right_sum_node,
            })))
        },
        None => None,
    }
}

const MODULO_MAX: i64 = 1_000_000_000 + 7;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let sum_root = calculate_sum_node(&root);
        let mut current_node = sum_root;
        let mut max = -1; // All numbers are positive, so this works

        let mut up_sum = 0;
        loop {
            if let Some(some_current_node) = current_node {
                let left_sum = some_current_node.as_ref().borrow().left.as_ref().map_or(0, |it| it.borrow().val);
                let right_sum = some_current_node.as_ref().borrow().right.as_ref().map_or(0, |it| it.borrow().val);
                let node_sum = some_current_node.borrow().val;
                // let node_val = node_sum - left_sum - right-sum;
                let left_edge_break_prod: i64 = left_sum as i64 * (node_sum - left_sum + up_sum) as i64;
                let right_edge_break_prod = right_sum as i64 * (node_sum - right_sum + up_sum) as i64;

                if left_edge_break_prod > right_edge_break_prod {
                    if left_edge_break_prod < max {
                        return (max % MODULO_MAX) as i32;
                    } else {
                        max = left_edge_break_prod;
                        current_node = some_current_node.borrow().left.clone();
                        up_sum = node_sum - left_sum + up_sum;
                    }
                } else {
                    if right_edge_break_prod < max {
                        return (max % MODULO_MAX) as i32;
                    } else {
                        max = right_edge_break_prod;
                        current_node = some_current_node.borrow().right.clone();
                        up_sum = node_sum - right_sum + up_sum;
                    }
                }

            } else {
                return 0;
            }
        }
    }
}
