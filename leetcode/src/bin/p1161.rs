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



use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level_nodes = vec![root];
        let mut max_level_value = i32::MIN;
        let mut level = 1;
        let mut max_level = 1;
        while !level_nodes.is_empty() {
            let mut next_level_nodes = vec![];
            let mut level_value = 0;
            // println!("Level: {level}");
            // print!("Nodes: ");
            for node in level_nodes {
                if let Some(some_node) = node {
                    let some_node = some_node.borrow();
                    if some_node.left.is_some() {
                        next_level_nodes.push(some_node.left.clone());
                    }
                    if some_node.right.is_some() {
                        next_level_nodes.push(some_node.right.clone());
                    }
                    level_value += some_node.val;
                    // print!("{} ", some_node.val)
                }
            }
            // println!("");
            // println!("Level value: {level_value}");

            if level_value > max_level_value {
                // println!("At level {level}, the previous max_level_value was {max_level_value}, not it would be {level_value}");
                max_level_value = level_value;
                max_level = level;
            }

            level += 1;
            level_nodes = next_level_nodes;

        }

        println!("Max Level value: {}", max_level_value);
        return max_level;
    }
}
