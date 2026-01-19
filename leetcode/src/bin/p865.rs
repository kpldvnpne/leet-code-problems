pub struct Solution;

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

fn main() {}

type OptionalNode = Option<Rc<RefCell<TreeNode>>>;

fn create_tree_with_depth(node: &OptionalNode) -> OptionalNode {
    match node {
        Some(some_node) => {
            let left_node = &some_node.borrow().left;
            let left_depth_node = create_tree_with_depth(left_node);
            let left_depth = left_depth_node.as_ref().map_or(0, |it| it.borrow().val);

            let right_node = &some_node.borrow().right;
            let right_depth_node = create_tree_with_depth(right_node);
            let right_depth = right_depth_node.as_ref().map_or(0, |it| it.borrow().val);

            Some(Rc::new(RefCell::new(TreeNode {
                val: std::cmp::max(left_depth, right_depth) + 1,
                left: left_depth_node,
                right: right_depth_node,
            })))
        },
        None => None,
    }
}

fn get_min_subtree_with_all_depest(node: &OptionalNode, depth_node: &OptionalNode) -> OptionalNode {
    if let (Some(some_depth_node), Some(some_node)) = (depth_node, node) {
        let left_node = &some_node.borrow().left;
        let left_depth_node = &some_depth_node.borrow().left;
        let left_depth = left_depth_node.as_ref().map_or(0, |it| it.borrow().val);
        let right_node = &some_node.borrow().right;
        let right_depth_node = &some_depth_node.borrow().right;
        let right_depth = right_depth_node.as_ref().map_or(0, |it| it.borrow().val);

        if left_depth == right_depth {
            return node.clone();
        } else if left_depth > right_depth {
            return get_min_subtree_with_all_depest(left_node, left_depth_node);
        } else {
            return get_min_subtree_with_all_depest(right_node, right_depth_node);
        }
    } else {
        panic!("Node or depth node is None");
    }
}


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let depth_root = create_tree_with_depth(&root);

        return get_min_subtree_with_all_depest(&root, &depth_root);
    }  
}