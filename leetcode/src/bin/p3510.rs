
pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![5, 2, 3, 1];
    let expected = 2;
    let actual = Solution::minimum_pair_removal(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![1, 2, 2];
    let expected = 0;
    let actual = Solution::minimum_pair_removal(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test7() {
    let nums = vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1];
    let expected = 9;
    let actual = Solution::minimum_pair_removal(nums);

    assert_eq!(expected, actual);
  }
}

use std::{cell::RefCell, rc::Rc};

struct Node {
  value: i64,
  index: (usize, usize),
  prev: Option<Rc<RefCell<Node>>>,
  next: Option<Rc<RefCell<Node>>>,
}

struct Sum {
  sum: i64,
  index: usize,
  first: Rc<RefCell<Node>>,
  second: Rc<RefCell<Node>>,
}

impl PartialOrd for Sum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Sum {
    fn eq(&self, other: &Self) -> bool {
        self.sum == other.sum && self.index == other.index
    }
}

impl Eq for Sum {}

impl Ord for Sum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.sum.cmp(&self.sum).then(other.index.cmp(&self.index)) // Reverse order from start
    }
}

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
      let n = nums.len();

      let mut n_smaller_than_prev = 0;
      let mut min_sums_heap = BinaryHeap::with_capacity(n);

      let mut prev = Rc::new(RefCell::new(Node {
        value: nums[0] as i64,
        index: (0, 0),
        prev: None,
        next: None,
      }));
      for i in 1..n {
        let cur = Rc::new(RefCell::new(Node {
          value: nums[i] as i64,
          index: (i, i),
          prev: Some(Rc::clone(&prev)),
          next: None,
        }));

        prev.borrow_mut().next = Some(Rc::clone(&cur));

        let sum = Sum {
          sum: prev.borrow().value + cur.borrow().value,
          index: cur.borrow().index.1,
          first: Rc::clone(&prev),
          second: Rc::clone(&cur)
        };

        min_sums_heap.push(sum);

        if nums[i-1] > nums[i] {
          n_smaller_than_prev += 1;
        }

        prev = cur;
      }

      let mut n_sums = 0;

      // println!("At first: {}", n_smaller_than_prev);

      while n_smaller_than_prev > 0 {
        let min_sum = min_sums_heap.pop().unwrap();
        if min_sum.first.borrow().next.is_none() || min_sum.second.borrow().prev.is_none() {
          // Skipping stale min sum
          continue;
        }

        let first_val = min_sum.first.borrow().value;
        let second_val = min_sum.second.borrow().value;
        if first_val > second_val {
          n_smaller_than_prev -= 1; // We replaced them, so this should be removed
        }

        // println!("{} {}", first_val, second_val);

        let new_val = first_val + second_val;
        let new_node = Rc::new(RefCell::new(Node{
          value: new_val,
          index: (min_sum.first.borrow().index.0, min_sum.second.borrow().index.1),
          prev: min_sum.first.borrow().prev.as_ref().map(|it| Rc::clone(&it)),
          next: min_sum.second.borrow().next.as_ref().map(|it| Rc::clone(&it)),
        }));
        if let Some(prev_node) = &min_sum.first.borrow().prev {
          let prev_val = prev_node.borrow().value;
          if prev_val > first_val && prev_val <= new_val {
            n_smaller_than_prev -= 1;
          }

          if prev_val <= first_val && prev_val > new_val {
            n_smaller_than_prev += 1;
          }

          prev_node.borrow_mut().next = Some(Rc::clone(&new_node));

          let prev_sum_node = Sum {
            sum: prev_val + new_val,
            index: new_node.borrow().index.1,
            first: Rc::clone(&prev_node),
            second: Rc::clone(&new_node),
          };
          min_sums_heap.push(prev_sum_node);
        }
        min_sum.first.borrow_mut().prev = None;
        min_sum.first.borrow_mut().next = None;

        if let Some(next_node) = &min_sum.second.borrow().next {
          let next_val = next_node.borrow().value;

          if second_val > next_val && new_val <= next_val {
            n_smaller_than_prev -= 1;
          }

          if second_val <= next_val && new_val > next_val {
            n_smaller_than_prev += 1;
          }

          next_node.borrow_mut().prev = Some(Rc::clone(&new_node));

          let next_sum_node = Sum {
            sum: next_val + new_val,
            index: next_node.borrow().index.1,
            first: Rc::clone(&new_node),
            second: Rc::clone(&next_node),
          };
          min_sums_heap.push(next_sum_node);
        }
        min_sum.second.borrow_mut().prev = None;
        min_sum.second.borrow_mut().next = None;

        n_sums += 1;
      }

      return n_sums;
    }
}
