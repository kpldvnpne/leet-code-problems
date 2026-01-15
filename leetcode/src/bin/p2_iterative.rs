pub struct Solution {}

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
    fn get_val_or_zero(option_node: &Option<Box<ListNode>>) -> i32 {
        if let Option::Some(node) = option_node {
            node.val
        } else {
            0
        }
    }

    fn get_next(option_node: &Option<Box<ListNode>>) -> &Option<Box<ListNode>> {
        if let Option::Some(node) = option_node {
            &node.next
        } else {
            &Option::None
        }
    }

    fn sum<'a>(l1: &'a Option<Box<ListNode>>, l2: &'a Option<Box<ListNode>>, carry: i32) ->  (Box<ListNode>, &'a Option<Box<ListNode>>, &'a Option<Box<ListNode>>, i32) {
        let sum = Solution::get_val_or_zero(&l1) + Solution::get_val_or_zero(&l2) + carry;
        let carry = sum / 10;
        let rem = sum % 10;
        let sum_node = Box::new(ListNode::new(rem));
        return (sum_node, Solution::get_next(l1), Solution::get_next(l2), carry);
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1 == Option::None && l2 == Option::None {
            return Option::None;
        }

        let (mut head, new_l1, new_l2, new_carry) = Solution::sum(&l1, &l2, 0);

        let mut l1 = new_l1;
        let mut l2 = new_l2;
        let mut carry = new_carry;
        let mut prev = &mut head;

        while *l1 != Option::None || *l2 != Option::None {
            let (new_node, new_l1, new_l2, new_carry) = Solution::sum(l1, l2, carry);
            prev.next = Option::Some(new_node);
            prev = prev.next.as_mut().unwrap();
            l1 = new_l1;
            l2 = new_l2;
            carry = new_carry;
        }

        if carry > 0 {
            prev.next = Option::Some(Box::new(ListNode::new(carry)))
        }

        return Option::Some(head);
    }
}
