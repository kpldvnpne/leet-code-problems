pub struct Solution;
fn main() {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      return Solution::add_two_numbers_recursive(l1, l2, 0);
  }

  pub fn add_two_numbers_recursive(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
      if l1 == Option::None && l2 == Option::None {
          if carry > 0 {
              return Option::Some(Box::new(ListNode::new(carry)));
          } else {
              return Option::None;
          }
      }

      let l1 = l1.unwrap_or(Box::new(ListNode::new(0)));
      let l2 = l2.unwrap_or(Box::new(ListNode::new(0)));
      let sum = l1.val + l2.val + carry;
      let digit = sum % 10;
      let carry = sum / 10;
      let mut sum_node = Box::new(ListNode::new(digit));
      sum_node.next = Solution::add_two_numbers_recursive(l1.next, l2.next, carry);

      return Option::Some(sum_node);
  }
}
